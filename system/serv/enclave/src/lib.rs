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

#![cfg_attr(not(target_vendor = "teaclave"), no_std)]
#![cfg_attr(target_vendor = "teaclave", feature(rustc_private))]

#[cfg(not(target_vendor = "teaclave"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_types;

use core::{ffi::c_char, ptr, slice};
use sgx_types::error::SgxStatus;
use star_core::serv::enclave::Enclave;
use rand::{Error, RngCore};

use sgx_trts::rand::rand as sgx_rand;

#[no_mangle]
pub extern "C" fn __xpg_strerror_r(_errnum: i32, buf: *mut c_char, buflen: usize) -> i32 {
    unsafe {
        if !buf.is_null() && buflen > 0 {
            *buf = 0;
        }
    }
    0
}

struct SgxRng;

impl RngCore for SgxRng {
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf);
        u32::from_ne_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.fill_bytes(&mut buf);
        u64::from_ne_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.try_fill_bytes(dest)
            .expect("sgx_rand failed")
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        sgx_rand(dest).map_err(|_| {
            Error::from(
                core::num::NonZeroU32::new(Error::CUSTOM_START)
                    .expect("CUSTOM_START is non-zero"),
            )
        })
    }
}

static mut ENCLAVE: Option<Enclave> = None;

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn init() -> SgxStatus {
    if ENCLAVE.is_some() {
        return SgxStatus::Success;
    }

    // Bootstrap enclave secrets.
    let token_sk = [0u8; 32];
    let hpke_sk = [0u8; 32];
    let max_tokens_per_credential = 3u32;

    match Enclave::new(max_tokens_per_credential, token_sk, hpke_sk) {
        Ok(enclave) => {
            ENCLAVE = Some(enclave);
            SgxStatus::Success
        }
        Err(_) => SgxStatus::Unexpected,
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn public_key(
    public_key: *mut u8,
    // out_len: *mut usize,
) -> SgxStatus {
    if public_key.is_null() {
        return SgxStatus::InvalidParameter;
    }

    let enclave = match ENCLAVE.as_ref() {
        Some(enclave) => enclave,
        None => return SgxStatus::InvalidState,
    };

    let pk = enclave.public_key();
    ptr::copy_nonoverlapping(pk.as_ptr(), public_key, pk.len());

    SgxStatus::Success
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn issue_cred(
    output: *mut u8,
    input: *const u8,
) -> SgxStatus {
    let mut rng = SgxRng;

    if output.is_null() || input.is_null() {
        return SgxStatus::InvalidParameter;
    };

    let enclave = match ENCLAVE.as_ref() {
        Some(enclave) => enclave,
        None => return SgxStatus::InvalidState,
    };


    // Fixed-size request envelope: HPKE envelope (aad 32 + info 32 + enc 32 + ciphertext 48).
    const CRED_REQUEST_LEN: usize = 32 + 32 + 32 + 48;
    let input_slice = slice::from_raw_parts(input as *const u8, CRED_REQUEST_LEN);
    
    let out = enclave.issue_cred(input_slice, &mut rng);
    ptr::copy_nonoverlapping(out.as_ptr(), output, out.len());

    SgxStatus::Success
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn issue_token(
    output: *mut u8,
    input: *const u8,
) -> SgxStatus {
    if output.is_null() || input.is_null() {
        return SgxStatus::InvalidParameter;
    };

    let enclave = match ENCLAVE.as_ref() {
        Some(enclave) => enclave,
        None => return SgxStatus::InvalidState,
    };

    let TICKET_REQUEST_LEN =  192;
    let input_slice = slice::from_raw_parts(input as *const u8, TICKET_REQUEST_LEN);
    let out = enclave.issue_token(input_slice).unwrap();
    println!("hello: {}", out.len());
    ptr::copy_nonoverlapping(out.as_ptr(), output, out.len());

    SgxStatus::Success
}
