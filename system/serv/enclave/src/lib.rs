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

use core::ptr;
use sgx_types::error::SgxStatus;
use star_core::serv::enclave::Enclave;

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
