use std::{io::{self, BufReader}, process::{Child, ChildStdin, ChildStdout, Command, Stdio}, sync::Mutex};

use crate::io as utilsio;
use crate::core;


#[derive(Debug)]
pub enum SubprocessError {
    Remote { status: u8, payload: Vec<u8> },
}

pub struct Subprocess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Subprocess {
    pub fn init(bin: &str) -> Self {
        let mut child = Command::new(&bin)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        let stdin = child
            .stdin
            .take()
            .unwrap();
        let stdout = child
            .stdout
            .take()
            .unwrap();

        let runtime = Subprocess {
            child,
            stdin,
            stdout: BufReader::new(stdout),
        };

        return runtime;
    }

    pub fn call(&mut self, op: u8, payload: &[u8]) -> Result<Vec<u8>, SubprocessError> {
        let stdin = &mut self.stdin;
        let stdout = &mut self.stdout;

        utilsio::write_request(stdin, op, payload).unwrap();
        let response = utilsio::read_response(stdout).unwrap();

        if response.status == core::STATUS_OK {
            Ok(response.payload)
        } else {
            Err(SubprocessError::Remote {
                status: response.status,
                payload: response.payload,
            })
        }
    }

    pub fn call_or_panic(&mut self, op: u8, payload: &[u8]) -> Vec<u8> {
        self.call(op, payload)
            .unwrap_or_else(|err| panic!("enclave stdio {op}: {:?}", err))
    }
}
