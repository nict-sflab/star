use std::fmt;
use std::io::{self, BufReader};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use crate::io as utilsio;
use crate::core;


#[derive(Debug)]
pub enum SubprocessError {
    Spawn(io::Error),
    MissingStdin,
    MissingStdout,
    Io(io::Error),
    Remote { status: u8, payload: Vec<u8> },
}

impl fmt::Display for SubprocessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spawn(err) => write!(f, "failed to spawn subprocess: {err}"),
            Self::MissingStdin => write!(f, "subprocess stdin was not available"),
            Self::MissingStdout => write!(f, "subprocess stdout was not available"),
            Self::Io(err) => write!(f, "subprocess IO failed: {err}"),
            Self::Remote { status, payload } => write!(
                f,
                "subprocess returned status {status} with {} bytes",
                payload.len()
            ),
        }
    }
}

impl std::error::Error for SubprocessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Spawn(err) | Self::Io(err) => Some(err),
            Self::MissingStdin | Self::MissingStdout | Self::Remote { .. } => None,
        }
    }
}

pub struct Subprocess {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Subprocess {
    pub fn init(bin: &str) -> Result<Self, SubprocessError> {
        let mut child = Command::new(&bin)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(SubprocessError::Spawn)?;

        let stdin = child
            .stdin
            .take()
            .ok_or(SubprocessError::MissingStdin)?;
        let stdout = child
            .stdout
            .take()
            .ok_or(SubprocessError::MissingStdout)?;

        let runtime = Subprocess {
            _child: child,
            stdin,
            stdout: BufReader::new(stdout),
        };

        Ok(runtime)
    }

    pub fn call(&mut self, op: u8, payload: &[u8]) -> Result<Vec<u8>, SubprocessError> {
        let stdin = &mut self.stdin;
        let stdout = &mut self.stdout;

        utilsio::write_request(stdin, op, payload).map_err(SubprocessError::Io)?;
        let response = utilsio::read_response(stdout).map_err(SubprocessError::Io)?;

        if response.status == core::STATUS_OK {
            Ok(response.payload)
        } else {
            Err(SubprocessError::Remote {
                status: response.status,
                payload: response.payload,
            })
        }
    }
}
