use std::env;
use std::fmt;
use std::io::{self, BufReader, ErrorKind};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{Mutex, OnceLock};
use env_logger::init;
use utils::io::{
    MAX_PAYLOAD_LEN, OP_INIT, OP_ISSUE_CRED, OP_ISSUE_TOKEN, OP_PUBLIC_KEY, STATUS_ENCLAVE_ERROR,
    STATUS_OK, read_response, write_request,
};


pub struct EnclaveRuntime {
    process: utils::parent::Subprocess,
}

impl EnclaveRuntime {
    pub fn init(name: &str) -> Self {
        let process  = utils::parent::Subprocess::init(name);
        EnclaveRuntime {
            process,
        }
    }

    pub fn from_env() -> Self {
        let name = default_enclave_bin();
        Self::init(&name)
    }

    pub fn public_key(&mut self) -> [u8; 32] {
        let response = self.process.call_or_panic(OP_PUBLIC_KEY, &[]);
        response
            .as_slice()
            .try_into()
            .unwrap_or_else(|_| panic!("enclave stdio public_key: bad public_key length"))
    }

    pub fn issue_cred(&mut self, cipher: &[u8]) -> Vec<u8> {
        self.process.call_or_panic(OP_ISSUE_CRED, cipher)
    }

    pub fn issue_token(&mut self, cipher: &[u8], current_period: [u8; 8]) -> Vec<u8> {
        let mut payload = Vec::with_capacity(current_period.len() + cipher.len());
        payload.extend_from_slice(&current_period);
        payload.extend_from_slice(cipher);
        self.process.call_or_panic(OP_ISSUE_TOKEN, &payload)
    }
}

fn default_enclave_bin() -> String {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let manifest_dir = option_env!("CARGO_MANIFEST_DIR").unwrap_or(".");

    PathBuf::from(manifest_dir)
        .join("../enclave/target")
        .join(profile)
        .join("enclave")
        .to_string_lossy()
        .into_owned()
}
