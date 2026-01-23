// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

extern crate sgx_types;
extern crate sgx_urts;

use sgx_types::*;

use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use sgx_urts::SgxEnclave;
use std::net::SocketAddr;
use std::sync::Arc;

static ENCLAVE_FILE: &str = "../bin/enclave.signed.so";

extern "C" {
    pub fn init(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
    ) -> sgx_status_t;
    
    pub fn public_key(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        public_key: *mut u8,
    ) -> sgx_status_t;

    pub fn issue_cred(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        output: *mut u8,
        input: *const u8,
    ) -> sgx_status_t;

    pub fn issue_token(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        output: *mut u8,
        input: *const u8,
    ) -> sgx_status_t;
}

const PUBLIC_KEY_LEN: usize = 32;
const CRED_REQUEST_LEN: usize = 144;
const CRED_RESPONSE_LEN: usize = 92;

#[derive(Clone)]
struct AppState {
    enclave: Arc<SgxEnclave>,
}

fn ensure_sgx_success(
    result: sgx_status_t,
    retval: sgx_status_t,
    context: &str,
) -> Result<(), (StatusCode, String)> {
    if result != sgx_status_t::SGX_SUCCESS {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{context} failed: {}", result.as_str()),
        ));
    }
    if retval != sgx_status_t::SGX_SUCCESS {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{context} failed: {}", retval.as_str()),
        ));
    }
    Ok(())
}

async fn handle_public_key(
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    let mut pk = [0u8; PUBLIC_KEY_LEN];
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let result = unsafe { public_key(state.enclave.geteid(), &mut retval, pk.as_mut_ptr()) };
    ensure_sgx_success(result, retval, "public_key")?;

    Ok(URL_SAFE.encode(pk))
}

async fn handle_issue_credential(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<String, (StatusCode, String)> {
    let encoded = std::str::from_utf8(&body).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "credential request must be utf-8".to_string(),
        )
    })?;
    let input = URL_SAFE.decode(encoded.trim()).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "credential request must be base64url".to_string(),
        )
    })?;
    if input.len() != CRED_REQUEST_LEN {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "credential request must be {CRED_REQUEST_LEN} bytes, got {}",
                input.len()
            ),
        ));
    }

    let mut output = [0u8; CRED_RESPONSE_LEN];
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let result = unsafe {
        issue_cred(
            state.enclave.geteid(),
            &mut retval,
            output.as_mut_ptr(),
            input.as_ptr(),
        )
    };
    ensure_sgx_success(result, retval, "issue_cred")?;

    Ok(URL_SAFE.encode(output))
}

#[tokio::main]
async fn main() {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    let enclave = match SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    ) {
        Ok(enclave) => {
            println!("[+] Init Enclave Successful {}!", enclave.geteid());
            enclave
        }
        Err(err) => {
            println!("[-] Init Enclave Failed {}!", err.as_str());
            return;
        }
    };

    let mut retval = sgx_status_t::SGX_SUCCESS;
    let init_result = unsafe { init(enclave.geteid(), &mut retval) };
    if let Err(err) = ensure_sgx_success(init_result, retval, "init") {
        let (_status, message) = err;
        panic!("[-] Enclave init failed: {message}");
    }

    let state = AppState {
        enclave: Arc::new(enclave),
    };

    let app = Router::new()
        .route("/star/public_key", get(handle_public_key))
        .route("/star/credential", post(handle_issue_credential))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// #[test]
// fn test_library() {
//     let mut rng = OsRng;

//     let enclave = match SgxEnclave::create(ENCLAVE_FILE, true) {
//         Ok(enclave) => {
//             println!("[+] Init Enclave Successful {}!", enclave.eid());
//             enclave
//         }
//         Err(err) => {
//             println!("[-] Init Enclave Failed {}!", err.as_str());
//             return;
//         }
//     };

//     let mut retval = SgxStatus::Success;

//     let init_result = unsafe { init(enclave.eid(), &mut retval) };
//     if init_result != SgxStatus::Success || retval != SgxStatus::Success {
//         panic!(
//             "[-] Enclave init failed {} (retval: {})!",
//             init_result.as_str(),
//             retval.as_str()
//         );
//     }

//     let mut pk: [u8; 32] = [0u8; 32];
//     let result = unsafe {
//         let status = public_key(
//             enclave.eid(),
//             &mut retval,
//             pk.as_mut_ptr(),
//         );

//         println!("{:?}", pk);
//         status
//     };
//     match result {
//         SgxStatus::Success => println!("[+] ECall Success..."),
//         _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
//     }

//     let sender = SecureChannelClient::new(pk).unwrap();
//     let mut user = User::new(10, sender);
//     let (req, shared_key) = user.request_credential(&mut rng);

//     let mut cipher = [0 as u8; 92];

//     let result = unsafe {
//         let status = issue_cred(
//             enclave.eid(),
//             &mut retval,
//             cipher.as_mut_ptr(),
//             (&req).as_ptr(),
//         );

//         println!("{:?}", cipher);
//         status
//     };
    
//     let mut token: [u8; 32] = [0u8; 32];

//     user.receive_credential(&cipher, &shared_key);
//     let request = user.request_auth(1, &mut rng).unwrap();

//     match result {
//         SgxStatus::Success => println!("[+] ECall Success..."),
//         _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
//     }

//     let result = unsafe {
//         let status = issue_token(
//             enclave.eid(),
//             &mut retval,
//             token.as_mut_ptr(),
//             (&request).as_ptr(),
//         );

//         println!("token: {:?}", token);
//         status
//     };

//     match result {
//         SgxStatus::Success => println!("[+] ECall Success..."),
//         _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
//     }

//     let result = unsafe {
//         let status = issue_token(
//             enclave.eid(),
//             &mut retval,
//             token.as_mut_ptr(),
//             (&request).as_ptr(),
//         );

//         println!("token: {:?}", token);
//         status
//     };

//     match result {
//         SgxStatus::Success => println!("[+] ECall Success..."),
//         _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
//     }
    
//     let request = user.request_auth(2, &mut rng).unwrap();

//     let result = unsafe {
//         let status = issue_token(
//             enclave.eid(),
//             &mut retval,
//             token.as_mut_ptr(),
//             (&request).as_ptr(),
//         );

//         println!("token: {:?}", token);
//         status
//     };

//     match result {
//         SgxStatus::Success => println!("[+] ECall Success..."),
//         _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
//     }
// }
