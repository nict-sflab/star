//! Operation decoding for the host's SGX library calls.
use enclave::Enclave;
use std::sync::RwLock;

const INIT: u64 = 0;
const PUBLIC_KEY: u64 = 1;
const ISSUE_CRED: u64 = 2;
const ISSUE_TOKEN: u64 = 3;
const ATTEST: u64 = 4;
const TRANSFER_PUBLIC_KEY: u64 = 5;
const INSTALL_KEYS: u64 = 6;
const ONE: u64 = 7;

pub(crate) fn dispatch(
    enclave: &RwLock<Enclave>,
    op: u64,
    payload: &[u8],
) -> Result<Vec<u8>, String> {
    match op {
        INIT => {
            if !payload.is_empty() {
                return Err("init does not accept keys; install a key-manager envelope".into());
            }
            enclave.write().unwrap().prepare_key_transfer()?;
            Ok(Vec::new())
        }
        INSTALL_KEYS => {
            let mut remaining = payload;
            let mut blobs = [&[][..]; 2];
            for blob in &mut blobs {
                let length = remaining.get(..4).ok_or("missing blob length")?;
                let len = u32::from_be_bytes(length.try_into().unwrap()) as usize;
                remaining = &remaining[4..];
                *blob = remaining.get(..len).ok_or("truncated blob")?;
                remaining = &remaining[len..];
            }
            if !remaining.is_empty() {
                return Err("trailing install request bytes".into());
            }
            let [evidence, envelope] = blobs;
            enclave.write().unwrap().install_keys(evidence, envelope)?;
            Ok(Vec::new())
        }
        _ => {
            let enclave = enclave.read().unwrap();
            match op {
                PUBLIC_KEY => Ok(enclave.public_key()?.to_vec()),
                ISSUE_CRED => enclave.issue_cred(payload),
                ISSUE_TOKEN => {
                    let period = payload.get(..8).ok_or("issue_token missing period")?;
                    enclave.issue_token(&payload[8..], period.try_into().unwrap())
                }
                ATTEST => {
                    if !payload.is_empty() {
                        return Err("transfer attestation does not accept host report_data".into());
                    }
                    enclave.attest()
                }
                TRANSFER_PUBLIC_KEY => Ok(enclave.transfer_public_key().to_vec()),
                ONE => Ok(vec![1; 32]),
                _ => Err("unknown operation".into()),
            }
        }
    }
}

#[path = "protocol_test.rs"]
#[cfg(test)]
mod tests;
