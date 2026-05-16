#[allow(dead_code)]
const CHALLENGE_LEN: usize = 64;

#[cfg(target_env = "sgx")]
const SGX_QL_ALG_ECDSA_P256: u32 = 2;
#[cfg(target_env = "sgx")]
const ATT_KEY_ALGORITHM_OFFSET: usize = 154;

#[cfg(target_env = "sgx")]
#[allow(dead_code)]
pub fn attest(report_data: [u8; CHALLENGE_LEN]) -> Result<Vec<u8>, String> {
    use aesm_client::sgx::AesmClientExt;
    use aesm_client::AesmClient;
    use sgx_isa::{Report, Targetinfo};

    let aesm = AesmClient::new();
    let att_key_id = select_ecdsa_p256_key(&aesm)?;

    let qe_info = aesm
        .init_quote_ex(att_key_id.clone())
        .map_err(|e| format!("initialize ECDSA quote: {e:?}"))?;

    let target_info = Targetinfo::try_copy_from(qe_info.target_info())
        .ok_or_else(|| "parse QE target info".to_string())?;

    let report = Report::for_target(&target_info, &report_data);

    let quote = aesm
        .get_quote_ex(att_key_id, report.as_ref().to_vec(), None, vec![0u8; 16])
        .map_err(|e| format!("get ECDSA quote from AESM: {e:?}"))?;

    Ok(quote.quote().to_vec())
}

#[cfg(not(target_env = "sgx"))]
#[allow(dead_code)]
pub fn attest(_: [u8; CHALLENGE_LEN]) -> Result<Vec<u8>, String> {
    Err("SGX attestation is only available for target_env=sgx".to_string())
}

#[cfg(target_env = "sgx")]
fn select_ecdsa_p256_key(aesm: &aesm_client::AesmClient) -> Result<Vec<u8>, String> {
    let key_ids = aesm
        .get_supported_att_key_ids()
        .map_err(|e| format!("get supported AESM attestation key IDs: {e:?}"))?;

    key_ids
        .into_iter()
        .find(|id| read_le_u32(id, ATT_KEY_ALGORITHM_OFFSET) == Some(SGX_QL_ALG_ECDSA_P256))
        .ok_or_else(|| "ECDSA/P-256 attestation key is not available".to_string())
}

#[cfg(target_env = "sgx")]
fn read_le_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = bytes.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}
