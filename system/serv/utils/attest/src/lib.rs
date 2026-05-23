#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

pub const CHALLENGE_LEN: usize = 64;
pub const MEASUREMENT_LEN: usize = 32;
pub const PUBLIC_KEY_LEN: usize = 32;
pub const QVE_TARGET_INFO_LEN: usize = 512;
pub const QVE_REPORT_INFO_LEN: usize = 16 + QVE_TARGET_INFO_LEN + 432;
pub const LOCAL_REPORT_LEN: usize = 432;

const QUOTE_HEADER_LEN: usize = 48;
const SGX_REPORT_BODY_LEN: usize = 384;
const QUOTE_V3: u16 = 3;
const QUOTE_V5: u16 = 5;
const QUOTE_V5_BODY_TYPE_OFFSET: usize = 48;
const QUOTE_V5_BODY_SIZE_OFFSET: usize = 50;
const QUOTE_V5_BODY_OFFSET: usize = 54;
const SGX_QL_ALG_ECDSA_P256: u16 = 2;
const TEE_TYPE_SGX: u32 = 0;
const QUOTE_BODY_TYPE_SGX: u16 = 1;
const QVE_EVIDENCE_MAGIC: &[u8; 8] = b"STARQVE1";
const SGX_QL_QV_RESULT_OK: u32 = 0x0000;
const SGX_QL_QV_RESULT_CONFIG_NEEDED: u32 = tee_qv_error(0x0001);
const SGX_QL_QV_RESULT_OUT_OF_DATE: u32 = tee_qv_error(0x0002);
const SGX_QL_QV_RESULT_OUT_OF_DATE_CONFIG_NEEDED: u32 = tee_qv_error(0x0003);
const SGX_QL_QV_RESULT_SW_HARDENING_NEEDED: u32 = tee_qv_error(0x0007);
const SGX_QL_QV_RESULT_CONFIG_AND_SW_HARDENING_NEEDED: u32 = tee_qv_error(0x0008);
#[cfg(target_env = "sgx")]
const QVE_REPORT_OFFSET: usize = 16 + QVE_TARGET_INFO_LEN;
#[cfg(target_env = "sgx")]
const QVE_REPORT_DATA_OFFSET: usize = QVE_REPORT_OFFSET + 320;
#[cfg(target_env = "sgx")]
const QVE_MISCSELECT_OFFSET: usize = QVE_REPORT_OFFSET + 16;
#[cfg(target_env = "sgx")]
const QVE_ATTRIBUTES_OFFSET: usize = QVE_REPORT_OFFSET + 48;
#[cfg(target_env = "sgx")]
const QVE_MRSIGNER_OFFSET: usize = QVE_REPORT_OFFSET + 128;
#[cfg(target_env = "sgx")]
const QVE_ISVPRODID_OFFSET: usize = QVE_REPORT_OFFSET + 256;
#[cfg(target_env = "sgx")]
const QVE_ISVSVN_OFFSET: usize = QVE_REPORT_OFFSET + 258;
#[cfg(target_env = "sgx")]
const QVE_MIN_ISVSVN: u16 = 7;
#[cfg(target_env = "sgx")]
const QVE_EXPECTED_MISCSELECT: u32 = 0;
#[cfg(target_env = "sgx")]
const QVE_MISCSELECT_MASK: u32 = 0xffff_ffff;
#[cfg(target_env = "sgx")]
const QVE_EXPECTED_ATTRIBUTES: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
#[cfg(target_env = "sgx")]
const QVE_ATTRIBUTES_MASK: [u8; 16] = [
    0xfb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[cfg(target_env = "sgx")]
const QVE_EXPECTED_MRSIGNER: [u8; MEASUREMENT_LEN] = [
    0x8c, 0x4f, 0x57, 0x75, 0xd7, 0x96, 0x50, 0x3e, 0x96, 0x13, 0x7f, 0x77, 0xc6, 0x8a, 0x82, 0x9a,
    0x00, 0x56, 0xac, 0x8d, 0xed, 0x70, 0x14, 0x0b, 0x08, 0x1b, 0x09, 0x44, 0x90, 0xc5, 0x7b, 0xff,
];

pub type Result<T> = core::result::Result<T, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedQuote {
    pub mrenclave: [u8; MEASUREMENT_LEN],
    pub mrsigner: [u8; MEASUREMENT_LEN],
    pub report_data: [u8; CHALLENGE_LEN],
    pub debug: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotePolicy {
    pub expected_mrenclave: Option<[u8; MEASUREMENT_LEN]>,
    pub expected_mrsigner: Option<[u8; MEASUREMENT_LEN]>,
    pub allow_debug: bool,
    pub allow_advisory: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QveEvidence<'a> {
    pub quote: &'a [u8],
    pub report_info: &'a [u8],
    pub expiration_check_date: i64,
    pub collateral_expiration_status: u32,
    pub quote_verification_result: u32,
    pub supplemental_data: &'a [u8],
}

pub fn report_data_for_public_key(public_key: [u8; PUBLIC_KEY_LEN]) -> [u8; CHALLENGE_LEN] {
    let mut report_data = [0u8; CHALLENGE_LEN];
    report_data[..PUBLIC_KEY_LEN].copy_from_slice(&public_key);
    report_data
}

pub fn public_key_from_report_data(parsed: &ParsedQuote) -> Result<[u8; PUBLIC_KEY_LEN]> {
    if parsed.report_data[PUBLIC_KEY_LEN..]
        .iter()
        .any(|byte| *byte != 0)
    {
        return Err("report_data is not public_key || zero padding".into());
    }

    parsed.report_data[..PUBLIC_KEY_LEN]
        .try_into()
        .map_err(|_| "report_data is too short for public key".into())
}

pub fn verify_quote_public_key(quote: &[u8], policy: &QuotePolicy) -> Result<[u8; PUBLIC_KEY_LEN]> {
    let parsed = parse_quote(quote)?;
    verify_policy(&parsed, policy)?;
    public_key_from_report_data(&parsed)
}

pub fn verify_qve_evidence_public_key(
    evidence: &[u8],
    policy: &QuotePolicy,
) -> Result<[u8; PUBLIC_KEY_LEN]> {
    if evidence.len() == local_report_len() && !evidence.starts_with(QVE_EVIDENCE_MAGIC) {
        return verify_local_report_public_key(evidence, policy);
    }

    let evidence = decode_qve_evidence(evidence)?;
    verify_qve_evidence(&evidence, policy.allow_advisory)?;
    verify_quote_public_key(evidence.quote, policy)
}

pub fn verify_qve_evidence(evidence: &QveEvidence<'_>, allow_advisory: bool) -> Result<()> {
    if evidence.collateral_expiration_status != 0 {
        return Err("QvE collateral is expired".into());
    }
    if !qv_result_is_ok(evidence.quote_verification_result)
        && !(allow_advisory && qv_result_is_advisory(evidence.quote_verification_result))
    {
        return Err(format!(
            "QvE quote verification result is {} / 0x{:08x}",
            qv_result_name(evidence.quote_verification_result),
            evidence.quote_verification_result,
        ));
    }
    if evidence.report_info.len() != QVE_REPORT_INFO_LEN {
        return Err("QvE report info has invalid length".into());
    }
    parse_quote(evidence.quote)?;

    trusted_verify_qve_evidence(evidence)
}

pub fn verify_policy(parsed: &ParsedQuote, policy: &QuotePolicy) -> Result<()> {
    let mut has_identity_policy = false;

    if let Some(expected_mrenclave) = policy.expected_mrenclave {
        has_identity_policy = true;
        if parsed.mrenclave != expected_mrenclave {
            return Err("MRENCLAVE does not match the trusted enclave measurement".into());
        }
    }

    if let Some(expected_mrsigner) = policy.expected_mrsigner {
        has_identity_policy = true;
        if parsed.mrsigner != expected_mrsigner {
            return Err("MRSIGNER does not match the trusted enclave signer".into());
        }
    }

    if !has_identity_policy {
        return Err("no trusted enclave identity policy is configured".into());
    }

    if parsed.debug && !policy.allow_debug {
        return Err("debug enclave is not trusted".into());
    }

    Ok(())
}

const fn tee_qv_error(value: u32) -> u32 {
    0x0000a000 | value
}

fn qv_result_is_ok(value: u32) -> bool {
    value == SGX_QL_QV_RESULT_OK
}

fn qv_result_is_advisory(value: u32) -> bool {
    matches!(
        value,
        SGX_QL_QV_RESULT_CONFIG_NEEDED
            | SGX_QL_QV_RESULT_OUT_OF_DATE
            | SGX_QL_QV_RESULT_OUT_OF_DATE_CONFIG_NEEDED
            | SGX_QL_QV_RESULT_SW_HARDENING_NEEDED
            | SGX_QL_QV_RESULT_CONFIG_AND_SW_HARDENING_NEEDED
    )
}

fn qv_result_name(value: u32) -> &'static str {
    match value {
        SGX_QL_QV_RESULT_OK => "OK",
        SGX_QL_QV_RESULT_CONFIG_NEEDED => "CONFIG_NEEDED",
        SGX_QL_QV_RESULT_OUT_OF_DATE => "OUT_OF_DATE",
        SGX_QL_QV_RESULT_OUT_OF_DATE_CONFIG_NEEDED => "OUT_OF_DATE_CONFIG_NEEDED",
        SGX_QL_QV_RESULT_SW_HARDENING_NEEDED => "SW_HARDENING_NEEDED",
        SGX_QL_QV_RESULT_CONFIG_AND_SW_HARDENING_NEEDED => "CONFIG_AND_SW_HARDENING_NEEDED",
        _ => "UNKNOWN",
    }
}

fn local_report_len() -> usize {
    LOCAL_REPORT_LEN
}

#[cfg(target_env = "sgx")]
fn verify_local_report_public_key(
    report_bytes: &[u8],
    policy: &QuotePolicy,
) -> Result<[u8; PUBLIC_KEY_LEN]> {
    use sgx_isa::{AttributesFlags, Report};

    let report = Report::try_copy_from(report_bytes)
        .ok_or_else(|| "local report has bad length".to_string())?;
    let report_mac_valid = report
        .verify(|key, data, mac| constant_time_eq(&aes_cmac_128::<aes::Aes128>(key, data), mac));
    if !report_mac_valid {
        return Err("local report MAC is invalid".into());
    }

    let parsed = ParsedQuote {
        mrenclave: report.mrenclave,
        mrsigner: report.mrsigner,
        report_data: report.reportdata,
        debug: report.attributes.flags.contains(AttributesFlags::DEBUG),
    };
    verify_policy(&parsed, policy)?;
    public_key_from_report_data(&parsed)
}

#[cfg(not(target_env = "sgx"))]
fn verify_local_report_public_key(_: &[u8], _: &QuotePolicy) -> Result<[u8; PUBLIC_KEY_LEN]> {
    Err("local report evidence is only verifiable inside SGX".into())
}

pub fn parse_measurement_hex(input: &str, name: &str) -> Result<[u8; MEASUREMENT_LEN]> {
    let mut out = [0u8; MEASUREMENT_LEN];
    let mut nibbles = 0usize;

    for byte in input.bytes() {
        if matches!(byte, b':' | b'_' | b' ' | b'\t' | b'\r' | b'\n') {
            continue;
        }
        if nibbles == MEASUREMENT_LEN * 2 {
            return Err(format!("{name} is longer than 32 bytes"));
        }
        let value = hex_nibble(byte).ok_or_else(|| format!("{name} contains non-hex data"))?;
        if nibbles % 2 == 0 {
            out[nibbles / 2] = value << 4;
        } else {
            out[nibbles / 2] |= value;
        }
        nibbles += 1;
    }

    if nibbles != MEASUREMENT_LEN * 2 {
        return Err(format!("{name} must be 32 bytes of hex"));
    }

    Ok(out)
}

fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

const POLICY_HAS_MRENCLAVE: u8 = 1;
const POLICY_HAS_MRSIGNER: u8 = 2;

pub fn encode_policy(policy: &QuotePolicy) -> Vec<u8> {
    let mut identity_flags = 0u8;
    let mut capacity = 3;
    if policy.expected_mrenclave.is_some() {
        identity_flags |= POLICY_HAS_MRENCLAVE;
        capacity += MEASUREMENT_LEN;
    }
    if policy.expected_mrsigner.is_some() {
        identity_flags |= POLICY_HAS_MRSIGNER;
        capacity += MEASUREMENT_LEN;
    }

    let mut out = Vec::with_capacity(capacity);
    out.push(identity_flags);
    out.push(u8::from(policy.allow_debug));
    out.push(u8::from(policy.allow_advisory));
    if let Some(expected_mrenclave) = policy.expected_mrenclave {
        out.extend_from_slice(&expected_mrenclave);
    }
    if let Some(expected_mrsigner) = policy.expected_mrsigner {
        out.extend_from_slice(&expected_mrsigner);
    }
    out
}

pub fn encode_policy_and_quote(policy: &QuotePolicy, quote: &[u8]) -> Vec<u8> {
    let mut out = encode_policy(policy);
    append_blob(&mut out, quote);
    out
}

pub fn decode_policy_and_quote(payload: &[u8]) -> Result<(QuotePolicy, &[u8])> {
    let mut remaining = payload;
    let policy = take_policy(&mut remaining)?;
    let quote = take_blob(&mut remaining)?;
    if !remaining.is_empty() {
        return Err("trailing policy payload bytes".into());
    }

    Ok((policy, quote))
}

pub fn encode_policy_and_qve_evidence(policy: &QuotePolicy, evidence: &[u8]) -> Vec<u8> {
    let mut out = encode_policy(policy);
    append_blob(&mut out, evidence);
    out
}

pub fn decode_policy_and_qve_evidence(payload: &[u8]) -> Result<(QuotePolicy, &[u8])> {
    let mut remaining = payload;
    let request = take_policy_and_qve_evidence(&mut remaining)?;
    if !remaining.is_empty() {
        return Err("trailing policy payload bytes".into());
    }
    Ok(request)
}

pub fn take_policy_and_qve_evidence<'a>(payload: &mut &'a [u8]) -> Result<(QuotePolicy, &'a [u8])> {
    let policy = take_policy(payload)?;
    let evidence = take_blob(payload)?;
    Ok((policy, evidence))
}

pub fn take_policy(payload: &mut &[u8]) -> Result<QuotePolicy> {
    let min_len = 3;
    if payload.len() < min_len {
        return Err("policy payload is too short".into());
    }

    let identity_flags = payload[0];
    if identity_flags & !(POLICY_HAS_MRENCLAVE | POLICY_HAS_MRSIGNER) != 0 {
        return Err("invalid policy identity flags".into());
    }
    let allow_debug = match payload[1] {
        0 => false,
        1 => true,
        _ => return Err("invalid debug policy flag".into()),
    };
    let allow_advisory = match payload[2] {
        0 => false,
        1 => true,
        _ => return Err("invalid advisory policy flag".into()),
    };

    let mut remaining = &payload[min_len..];
    let expected_mrenclave = if identity_flags & POLICY_HAS_MRENCLAVE != 0 {
        Some(take_measurement(&mut remaining, "MRENCLAVE")?)
    } else {
        None
    };
    let expected_mrsigner = if identity_flags & POLICY_HAS_MRSIGNER != 0 {
        Some(take_measurement(&mut remaining, "MRSIGNER")?)
    } else {
        None
    };

    *payload = remaining;

    Ok(QuotePolicy {
        expected_mrenclave,
        expected_mrsigner,
        allow_debug,
        allow_advisory,
    })
}

fn take_measurement(payload: &mut &[u8], name: &str) -> Result<[u8; MEASUREMENT_LEN]> {
    if payload.len() < MEASUREMENT_LEN {
        return Err(format!("policy payload is too short for {name}"));
    }
    let measurement = payload[..MEASUREMENT_LEN]
        .try_into()
        .map_err(|_| format!("bad {name} length"))?;
    *payload = &payload[MEASUREMENT_LEN..];
    Ok(measurement)
}

pub fn encode_qve_evidence(evidence: &QveEvidence<'_>) -> Result<Vec<u8>> {
    if evidence.report_info.len() != QVE_REPORT_INFO_LEN {
        return Err("QvE report info has invalid length".into());
    }

    let mut out = Vec::with_capacity(
        QVE_EVIDENCE_MAGIC.len()
            + 8
            + 4
            + 4
            + 12
            + evidence.report_info.len()
            + evidence.supplemental_data.len()
            + evidence.quote.len(),
    );
    out.extend_from_slice(QVE_EVIDENCE_MAGIC);
    out.extend_from_slice(&evidence.expiration_check_date.to_be_bytes());
    out.extend_from_slice(&evidence.collateral_expiration_status.to_be_bytes());
    out.extend_from_slice(&evidence.quote_verification_result.to_be_bytes());
    append_blob(&mut out, evidence.report_info);
    append_blob(&mut out, evidence.supplemental_data);
    append_blob(&mut out, evidence.quote);
    Ok(out)
}

pub fn decode_qve_evidence(payload: &[u8]) -> Result<QveEvidence<'_>> {
    if payload.len() < QVE_EVIDENCE_MAGIC.len() + 8 + 4 + 4 {
        return Err("QvE evidence is too short".into());
    }
    if &payload[..QVE_EVIDENCE_MAGIC.len()] != QVE_EVIDENCE_MAGIC {
        return Err("QvE evidence magic does not match".into());
    }

    let mut remaining = &payload[QVE_EVIDENCE_MAGIC.len()..];
    let expiration_check_date = take_i64_be(&mut remaining)?;
    let collateral_expiration_status = take_u32_be(&mut remaining)?;
    let quote_verification_result = take_u32_be(&mut remaining)?;
    let report_info = take_blob(&mut remaining)?;
    if report_info.len() != QVE_REPORT_INFO_LEN {
        return Err("QvE report info has invalid length".into());
    }
    let supplemental_data = take_blob(&mut remaining)?;
    let quote = take_blob(&mut remaining)?;
    if !remaining.is_empty() {
        return Err("trailing QvE evidence bytes".into());
    }

    Ok(QveEvidence {
        quote,
        report_info,
        expiration_check_date,
        collateral_expiration_status,
        quote_verification_result,
        supplemental_data,
    })
}

pub fn append_blob(payload: &mut Vec<u8>, blob: &[u8]) {
    payload.extend_from_slice(&(blob.len() as u32).to_be_bytes());
    payload.extend_from_slice(blob);
}

pub fn take_blob<'a>(payload: &mut &'a [u8]) -> Result<&'a [u8]> {
    let len: [u8; 4] = payload
        .get(..4)
        .ok_or_else(|| "missing blob length".to_string())?
        .try_into()
        .map_err(|_| "bad blob length".to_string())?;

    let len = u32::from_be_bytes(len) as usize;
    let blob = payload
        .get(4..4 + len)
        .ok_or_else(|| "truncated blob".to_string())?;
    *payload = &payload[4 + len..];
    Ok(blob)
}

fn take_i64_be(payload: &mut &[u8]) -> Result<i64> {
    let bytes: [u8; 8] = payload
        .get(..8)
        .ok_or_else(|| "missing i64".to_string())?
        .try_into()
        .map_err(|_| "bad i64 length".to_string())?;
    *payload = &payload[8..];
    Ok(i64::from_be_bytes(bytes))
}

pub fn take_u32_be(payload: &mut &[u8]) -> Result<u32> {
    let bytes: [u8; 4] = payload
        .get(..4)
        .ok_or_else(|| "missing u32".to_string())?
        .try_into()
        .map_err(|_| "bad u32 length".to_string())?;
    *payload = &payload[4..];
    Ok(u32::from_be_bytes(bytes))
}

#[cfg(feature = "std")]
pub mod qve;

pub fn parse_quote(quote: &[u8]) -> Result<ParsedQuote> {

    let report_body_offset = sgx_report_body_offset(quote)?;

    let attributes_offset = report_body_offset + 48;
    let mrenclave_offset = report_body_offset + 64;
    let mrsigner_offset = report_body_offset + 128;
    let report_data_offset = report_body_offset + 320;

    let flags = read_u64_le(quote, attributes_offset)?;

    Ok(ParsedQuote {
        mrenclave: read_array(quote, mrenclave_offset, "MRENCLAVE")?,
        mrsigner: read_array(quote, mrsigner_offset, "MRSIGNER")?,
        report_data: read_array(quote, report_data_offset, "report_data")?,
        debug: flags & 0x2 != 0,
    })
}

fn sgx_report_body_offset(quote: &[u8]) -> Result<usize> {
    if quote.len() < QUOTE_HEADER_LEN {
        return Err("quote is too short".into());
    }

    let version = read_u16_le(quote, 0)?;
    let attestation_key_type = read_u16_le(quote, 2)?;
    if attestation_key_type != SGX_QL_ALG_ECDSA_P256 {
        return Err("quote attestation key is not ECDSA/P-256".into());
    }

    let report_body_offset = match version {
        QUOTE_V3 => {
            let attestation_key_data = read_u32_le(quote, 4)?;
            if attestation_key_data != 0 {
                return Err("SGX quote v3 attestation key data must be zero".into());
            }
            QUOTE_HEADER_LEN
        }
        QUOTE_V5 => {
            let tee_type = read_u32_le(quote, 4)?;
            if tee_type != TEE_TYPE_SGX {
                return Err("quote is not for SGX".into());
            }
            let body_type = read_u16_le(quote, QUOTE_V5_BODY_TYPE_OFFSET)?;
            if body_type != QUOTE_BODY_TYPE_SGX {
                return Err("quote v5 body is not an SGX report".into());
            }
            let body_size = read_u32_le(quote, QUOTE_V5_BODY_SIZE_OFFSET)? as usize;
            if body_size < SGX_REPORT_BODY_LEN {
                return Err("quote v5 SGX body is too short".into());
            }
            if quote.len() < QUOTE_V5_BODY_OFFSET + body_size {
                return Err("quote v5 body size exceeds quote length".into());
            }
            QUOTE_V5_BODY_OFFSET
        }
        4 => return Err("TDX quote v4 is not an SGX enclave quote".into()),
        _ => return Err(format!("unsupported quote version: {version}")),
    };

    if quote.len() < report_body_offset + SGX_REPORT_BODY_LEN {
        return Err("quote is too short".into());
    }

    Ok(report_body_offset)
}

fn read_array<const N: usize>(bytes: &[u8], offset: usize, name: &str) -> Result<[u8; N]> {
    bytes
        .get(offset..offset + N)
        .ok_or_else(|| format!("quote is too short for {name}"))?
        .try_into()
        .map_err(|_| format!("quote is too short for {name}"))
}

fn read_u16_le(bytes: &[u8], offset: usize) -> Result<u16> {
    let bytes: [u8; 2] = read_array(bytes, offset, "u16")?;
    Ok(u16::from_le_bytes(bytes))
}

fn read_u32_le(bytes: &[u8], offset: usize) -> Result<u32> {
    let bytes: [u8; 4] = read_array(bytes, offset, "u32")?;
    Ok(u32::from_le_bytes(bytes))
}

fn read_u64_le(bytes: &[u8], offset: usize) -> Result<u64> {
    let bytes: [u8; 8] = read_array(bytes, offset, "u64")?;
    Ok(u64::from_le_bytes(bytes))
}

#[cfg(target_env = "sgx")]
fn trusted_verify_qve_evidence(evidence: &QveEvidence<'_>) -> Result<()> {
    use sgx_isa::Report;
    use sha2::{Digest, Sha256};

    let report_bytes = evidence
        .report_info
        .get(QVE_REPORT_OFFSET..QVE_REPORT_OFFSET + Report::UNPADDED_SIZE)
        .ok_or_else(|| "QvE report info is truncated".to_string())?;
    let report = Report::try_copy_from(report_bytes)
        .ok_or_else(|| "QvE report has bad length".to_string())?;

    let report_mac_valid = report
        .verify(|key, data, mac| constant_time_eq(&aes_cmac_128::<aes::Aes128>(key, data), mac));
    if !report_mac_valid {
        return Err("QvE report MAC is invalid".into());
    }

    let mut hasher = Sha256::new();
    hasher.update(&evidence.report_info[..16]);
    hasher.update(evidence.quote);
    hasher.update(evidence.expiration_check_date.to_le_bytes());
    hasher.update(evidence.collateral_expiration_status.to_le_bytes());
    hasher.update(evidence.quote_verification_result.to_le_bytes());
    hasher.update(evidence.supplemental_data);

    let digest = hasher.finalize();
    let report_data = evidence
        .report_info
        .get(QVE_REPORT_DATA_OFFSET..QVE_REPORT_DATA_OFFSET + CHALLENGE_LEN)
        .ok_or_else(|| "QvE report_data is truncated".to_string())?;
    if !constant_time_eq(&digest, &report_data[..32])
        || report_data[32..].iter().any(|byte| *byte != 0)
    {
        return Err("QvE report_data does not match verified quote data".into());
    }

    verify_qve_identity(evidence.report_info)
}

#[cfg(target_env = "sgx")]
fn verify_qve_identity(report_info: &[u8]) -> Result<()> {
    let miscselect = u32::from_le_bytes(read_array(
        report_info,
        QVE_MISCSELECT_OFFSET,
        "miscselect",
    )?);
    if miscselect & QVE_MISCSELECT_MASK != QVE_EXPECTED_MISCSELECT {
        return Err("QvE MISCSELECT does not match Intel QvE identity".into());
    }

    let attributes = report_info
        .get(QVE_ATTRIBUTES_OFFSET..QVE_ATTRIBUTES_OFFSET + 16)
        .ok_or_else(|| "QvE attributes are truncated".to_string())?;
    for index in 0..16 {
        if attributes[index] & QVE_ATTRIBUTES_MASK[index] != QVE_EXPECTED_ATTRIBUTES[index] {
            return Err("QvE ATTRIBUTES do not match Intel QvE identity".into());
        }
    }

    let mrsigner: [u8; MEASUREMENT_LEN] =
        read_array(report_info, QVE_MRSIGNER_OFFSET, "QvE MRSIGNER")?;
    if mrsigner != QVE_EXPECTED_MRSIGNER {
        return Err("QvE MRSIGNER does not match Intel QvE identity".into());
    }

    let isvprodid = u16::from_le_bytes(read_array(
        report_info,
        QVE_ISVPRODID_OFFSET,
        "QvE ISVPRODID",
    )?);
    if isvprodid != 2 {
        return Err("QvE ISVPRODID does not match Intel QvE identity".into());
    }

    let isvsvn = u16::from_le_bytes(read_array(report_info, QVE_ISVSVN_OFFSET, "QvE ISVSVN")?);
    if isvsvn < QVE_MIN_ISVSVN {
        return Err("QvE ISVSVN is below the trusted minimum".into());
    }

    Ok(())
}

#[cfg(target_env = "sgx")]
#[allow(deprecated)]
fn aes_cmac_128<C>(key: &[u8; 16], data: &[u8]) -> [u8; 16]
where
    C: aes::cipher::BlockEncrypt + aes::cipher::KeyInit,
{
    let cipher = C::new(aes::cipher::generic_array::GenericArray::from_slice(key));
    let mut zero = aes::cipher::generic_array::GenericArray::clone_from_slice(&[0u8; 16]);
    cipher.encrypt_block(&mut zero);

    let k1 = cmac_double(zero.as_slice().try_into().expect("AES block is 16 bytes"));
    let k2 = cmac_double(&k1);
    let complete = !data.is_empty() && data.len() % 16 == 0;
    let block_count = if data.is_empty() {
        1
    } else {
        data.len().div_ceil(16)
    };

    let mut last = [0u8; 16];
    if complete {
        let offset = (block_count - 1) * 16;
        last.copy_from_slice(&data[offset..offset + 16]);
        xor_in_place(&mut last, &k1);
    } else {
        let offset = (block_count - 1) * 16;
        let tail = &data[offset..];
        last[..tail.len()].copy_from_slice(tail);
        last[tail.len()] = 0x80;
        xor_in_place(&mut last, &k2);
    }

    let mut state = [0u8; 16];
    for block in data[..(block_count - 1) * 16].chunks_exact(16) {
        let mut input = state;
        xor_in_place(&mut input, block.try_into().expect("chunk is 16 bytes"));
        let mut input = aes::cipher::generic_array::GenericArray::clone_from_slice(&input);
        cipher.encrypt_block(&mut input);
        state.copy_from_slice(&input);
    }

    xor_in_place(&mut state, &last);
    let mut state = aes::cipher::generic_array::GenericArray::clone_from_slice(&state);
    cipher.encrypt_block(&mut state);
    state.as_slice().try_into().expect("AES block is 16 bytes")
}

#[cfg(target_env = "sgx")]
fn cmac_double(input: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut carry = 0u8;
    for index in (0..16).rev() {
        out[index] = (input[index] << 1) | carry;
        carry = input[index] >> 7;
    }
    if input[0] & 0x80 != 0 {
        out[15] ^= 0x87;
    }
    out
}

#[cfg(target_env = "sgx")]
fn xor_in_place(left: &mut [u8; 16], right: &[u8; 16]) {
    for index in 0..16 {
        left[index] ^= right[index];
    }
}

#[cfg(target_env = "sgx")]
fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut diff = 0u8;
    for index in 0..left.len() {
        diff |= left[index] ^ right[index];
    }
    diff == 0
}

#[cfg(not(target_env = "sgx"))]
fn trusted_verify_qve_evidence(_: &QveEvidence<'_>) -> Result<()> {
    Err("QvE evidence can only be trusted inside an SGX enclave".into())
}

#[path = "lib_test.rs"]
#[cfg(test)]
mod tests;
