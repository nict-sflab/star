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

use rand::rngs::OsRng;
use sgx_types::error::SgxStatus;
use sgx_types::types::*;
use sgx_urts::enclave::SgxEnclave;
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;

static ENCLAVE_FILE: &str = "enclave.signed.so";

 extern "C" {
    pub fn init(
        eid: EnclaveId,
        retval: *mut SgxStatus,
    ) -> SgxStatus;
    
    pub fn public_key(
        eid: EnclaveId,
        retval: *mut SgxStatus,
        public_key: *mut u8,
    ) -> SgxStatus;

    pub fn issue_cred(
        eid: EnclaveId,
        retval: *mut SgxStatus,
        output: *mut u8,
        input: *const u8,
    ) -> SgxStatus;

    pub fn issue_token(
        eid: EnclaveId,
        retval: *mut SgxStatus,
        output: *mut u8,
        input: *const u8,
    ) -> SgxStatus;
}

fn main() {
    let mut rng = OsRng;

    let enclave = match SgxEnclave::create(ENCLAVE_FILE, true) {
        Ok(enclave) => {
            println!("[+] Init Enclave Successful {}!", enclave.eid());
            enclave
        }
        Err(err) => {
            println!("[-] Init Enclave Failed {}!", err.as_str());
            return;
        }
    };

    let mut retval = SgxStatus::Success;

    let init_result = unsafe { init(enclave.eid(), &mut retval) };
    if init_result != SgxStatus::Success || retval != SgxStatus::Success {
        panic!(
            "[-] Enclave init failed {} (retval: {})!",
            init_result.as_str(),
            retval.as_str()
        );
    }

    let mut pk: [u8; 32] = [0u8; 32];
    let result = unsafe {
        let status = public_key(
            enclave.eid(),
            &mut retval,
            pk.as_mut_ptr(),
        );

        println!("{:?}", pk);
        status
    };
    match result {
        SgxStatus::Success => println!("[+] ECall Success..."),
        _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
    }

    let sender = SecureChannelClient::new(pk).unwrap();
    let mut user = User::new(10, sender);
    let (req, shared_key) = user.request_credential(&mut rng);

    let mut cipher = [0 as u8; 92];

    let result = unsafe {
        let status = issue_cred(
            enclave.eid(),
            &mut retval,
            cipher.as_mut_ptr(),
            (&req).as_ptr(),
        );

        println!("{:?}", cipher);
        status
    };
    
    let mut token: [u8; 32] = [0u8; 32];

    user.receive_credential(&cipher, &shared_key);
    let request = user.request_auth(1, &mut rng).unwrap();

    match result {
        SgxStatus::Success => println!("[+] ECall Success..."),
        _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
    }

    let result = unsafe {
        let status = issue_token(
            enclave.eid(),
            &mut retval,
            token.as_mut_ptr(),
            (&request).as_ptr(),
        );

        println!("token: {:?}", token);
        status
    };

    match result {
        SgxStatus::Success => println!("[+] ECall Success..."),
        _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
    }

    let result = unsafe {
        let status = issue_token(
            enclave.eid(),
            &mut retval,
            token.as_mut_ptr(),
            (&request).as_ptr(),
        );

        println!("token: {:?}", token);
        status
    };

    match result {
        SgxStatus::Success => println!("[+] ECall Success..."),
        _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
    }
    
    let request = user.request_auth(2, &mut rng).unwrap();

    let result = unsafe {
        let status = issue_token(
            enclave.eid(),
            &mut retval,
            token.as_mut_ptr(),
            (&request).as_ptr(),
        );

        println!("token: {:?}", token);
        status
    };

    match result {
        SgxStatus::Success => println!("[+] ECall Success..."),
        _ => panic!("[-] ECall Enclave Failed {}!", result.as_str()),
    }
}
