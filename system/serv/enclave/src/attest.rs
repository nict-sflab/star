const CHALLENGE_LEN: usize = 64;
#[cfg(any(test, target_env = "sgx"))]
const SGX_QL_ALG_ECDSA_P256: u32 = 2;
#[cfg(any(test, target_env = "sgx"))]
const ATT_KEY_ALGORITHM_OFFSET: usize = 154;

#[cfg(target_env = "sgx")]
pub fn attest(report_data: [u8; CHALLENGE_LEN]) -> Result<Vec<u8>, String> {
    use aesm_client::sgx::AesmClientExt;
    use sgx_isa::{Report, Targetinfo};
    use std::net::TcpStream;

    let aesm_proxy = std::env::var("AESM_PROXY").unwrap_or_else(|_| "127.0.0.1:5555".to_string());
    let stream =
        TcpStream::connect(&aesm_proxy).map_err(|e| format!("connect to AESM proxy: {e}"))?;
    let aesm = aesm_client::AesmClient::new(stream);

    let att_key_id = select_ecdsa_p256_key(&aesm)?;

    let qe_info = aesm
        .init_quote_ex(att_key_id.clone())
        .map_err(|e| format!("initialize ECDSA quote: {e:?}"))?;

    let target_info = Targetinfo::try_copy_from(qe_info.target_info())
        .ok_or_else(|| "parse QE target info".to_string())?;

    let report = Report::for_target(&target_info, &report_data);

    let quote = aesm
        .get_quote_ex(
            att_key_id,
            <Report as AsRef<[u8]>>::as_ref(&report).to_vec(),
            Some(qe_info.target_info().to_vec()),
            vec![0u8; 16],
        )
        .map_err(|e| format!("get ECDSA quote from AESM: {e:?}"))?;

    Ok(quote.quote().to_vec())
}

#[cfg(not(target_env = "sgx"))]
pub fn attest(_: [u8; CHALLENGE_LEN]) -> Result<Vec<u8>, String> {
    Err("remote attestation is only available inside the Fortanix SGX target".to_string())
}

#[cfg(target_env = "sgx")]
fn select_ecdsa_p256_key(aesm: &aesm_client::AesmClient) -> Result<Vec<u8>, String> {
    let key_ids = aesm
        .get_supported_att_key_ids()
        .map_err(|e| format!("get supported AESM attestation key IDs: {e:?}"))?;

    select_ecdsa_p256_key_id(key_ids)
        .ok_or_else(|| "ECDSA/P-256 attestation key is not available".to_string())
}

#[cfg(any(test, target_env = "sgx"))]
fn select_ecdsa_p256_key_id(key_ids: impl IntoIterator<Item = Vec<u8>>) -> Option<Vec<u8>> {
    key_ids
        .into_iter()
        .find(|id| read_le_u32(id, ATT_KEY_ALGORITHM_OFFSET) == Some(SGX_QL_ALG_ECDSA_P256))
}

#[cfg(any(test, target_env = "sgx"))]
fn read_le_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = bytes.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn att_key_id(algorithm: u32) -> Vec<u8> {
        let mut id = vec![0u8; ATT_KEY_ALGORITHM_OFFSET + 4];
        id[ATT_KEY_ALGORITHM_OFFSET..ATT_KEY_ALGORITHM_OFFSET + 4]
            .copy_from_slice(&algorithm.to_le_bytes());
        id
    }

    #[test]
    fn read_le_u32_reads_offset_and_rejects_short_inputs() {
        let mut bytes = vec![0u8; ATT_KEY_ALGORITHM_OFFSET + 4];
        bytes[ATT_KEY_ALGORITHM_OFFSET..ATT_KEY_ALGORITHM_OFFSET + 4]
            .copy_from_slice(&0x7856_3412u32.to_le_bytes());

        assert_eq!(
            read_le_u32(&bytes, ATT_KEY_ALGORITHM_OFFSET),
            Some(0x7856_3412)
        );
        assert_eq!(
            read_le_u32(
                &bytes[..ATT_KEY_ALGORITHM_OFFSET + 3],
                ATT_KEY_ALGORITHM_OFFSET
            ),
            None
        );
    }

    #[test]
    fn select_ecdsa_p256_key_id_skips_short_and_non_ecdsa_keys() {
        let rsa_key = att_key_id(1);
        let ecdsa_key = att_key_id(SGX_QL_ALG_ECDSA_P256);

        assert_eq!(
            select_ecdsa_p256_key_id(vec![vec![0u8; 12], rsa_key, ecdsa_key.clone()]),
            Some(ecdsa_key)
        );
    }

    #[cfg(not(target_env = "sgx"))]
    #[test]
    fn attest_reports_unsupported_outside_sgx() {
        assert!(attest([0u8; CHALLENGE_LEN])
            .expect_err("attest should require SGX")
            .contains("Fortanix SGX target"));
    }

    #[cfg(target_env = "sgx")]
    #[test]
    fn remote_attestation_quote_contains_report_data_when_aesm_proxy_is_available() {
        let mut report_data = [0u8; CHALLENGE_LEN];
        for (index, byte) in report_data.iter_mut().enumerate() {
            *byte = index as u8 ^ 0xa5;
        }

        let quote = match attest(report_data) {
            Ok(quote) => quote,
            Err(err) if err.contains("connect to AESM proxy") => {
                eprintln!("skipping remote attestation smoke test: {err}");
                return;
            }
            Err(err) => panic!("remote attestation quote failed: {err}"),
        };

        assert!(!quote.is_empty());
        assert!(quote
            .windows(CHALLENGE_LEN)
            .any(|window| window == report_data.as_slice()));
    }
}
