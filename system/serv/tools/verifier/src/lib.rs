use std::ffi::{CString, c_void};
use std::os::raw::c_long;
use std::path::{Path, PathBuf};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::{
    Engine as _,
    engine::general_purpose::{URL_SAFE, URL_SAFE_NO_PAD},
};
use libloading::{Library, Symbol};
use star_attestation::QuotePolicy;
pub use star_attestation::{
    CHALLENGE_LEN, MEASUREMENT_LEN, PUBLIC_KEY_LEN, ParsedQuote, parse_quote,
    public_key_from_report_data,
};

const SGX_QL_SUCCESS: u32 = 0x0000;
const SGX_QL_QV_RESULT_OK: u32 = 0x0000;
const SGX_QL_QV_RESULT_CONFIG_NEEDED: u32 = tee_qv_error(0x0001);
const SGX_QL_QV_RESULT_OUT_OF_DATE: u32 = tee_qv_error(0x0002);
const SGX_QL_QV_RESULT_OUT_OF_DATE_CONFIG_NEEDED: u32 = tee_qv_error(0x0003);
const SGX_QL_QV_RESULT_INVALID_SIGNATURE: u32 = tee_qv_error(0x0004);
const SGX_QL_QV_RESULT_REVOKED: u32 = tee_qv_error(0x0005);
const SGX_QL_QV_RESULT_UNSPECIFIED: u32 = tee_qv_error(0x0006);
const SGX_QL_QV_RESULT_SW_HARDENING_NEEDED: u32 = tee_qv_error(0x0007);
const SGX_QL_QV_RESULT_CONFIG_AND_SW_HARDENING_NEEDED: u32 = tee_qv_error(0x0008);
const SGX_QL_QV_RESULT_TD_RELAUNCH_ADVISED: u32 = tee_qv_error(0x0009);
const SGX_QL_QV_RESULT_TD_RELAUNCH_ADVISED_CONFIG_NEEDED: u32 = tee_qv_error(0x000a);
const SGX_QV_QPL_PATH: i32 = 1;

const fn tee_qv_error(value: u32) -> u32 {
    0x0000a000 | value
}

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DcapVerification {
    pub collateral_expiration_status: u32,
    pub quote_verification_result: u32,
}

#[derive(Debug, Clone)]
pub struct QuoteVerificationPolicy<'a> {
    pub expected_mrsigner: &'a [u8; MEASUREMENT_LEN],
    pub expected_mrenclave: Option<&'a [u8; MEASUREMENT_LEN]>,
    pub expected_public_key: Option<&'a [u8; PUBLIC_KEY_LEN]>,
    pub allow_debug: bool,
    pub allow_advisory: bool,
    pub qvl_library: Option<&'a Path>,
    pub qpl_library: Option<&'a Path>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedQuote {
    pub parsed: ParsedQuote,
    pub public_key: [u8; PUBLIC_KEY_LEN],
    pub dcap: DcapVerification,
}

pub fn verify_quote(quote: &[u8], policy: &QuoteVerificationPolicy<'_>) -> Result<VerifiedQuote> {
    let dcap = verify_quote_with_dcap_qvl(quote, policy.qvl_library, policy.qpl_library)?;
    validate_qv_result(dcap.quote_verification_result, policy.allow_advisory)?;

    let parsed = parse_quote(quote)?;
    let public_key = public_key_from_report_data(&parsed)?;
    if let Some(expected_public_key) = policy.expected_public_key {
        if &public_key != expected_public_key {
            return Err("attested public key does not match expected public key".to_string());
        }
    }
    let local_policy = quote_policy_from_verification_policy(policy);
    star_attestation::verify_policy(&parsed, &local_policy)?;

    Ok(VerifiedQuote {
        parsed,
        public_key,
        dcap,
    })
}

fn quote_policy_from_verification_policy(policy: &QuoteVerificationPolicy<'_>) -> QuotePolicy {
    QuotePolicy {
        expected_mrenclave: policy.expected_mrenclave.copied(),
        expected_mrsigner: Some(*policy.expected_mrsigner),
        allow_debug: policy.allow_debug,
        allow_advisory: policy.allow_advisory,
    }
}

pub fn verify_quote_with_dcap_qvl(
    quote: &[u8],
    qvl_library: Option<&Path>,
    qpl_library: Option<&Path>,
) -> Result<DcapVerification> {
    type SgxQvVerifyQuote = unsafe extern "C" fn(
        p_quote: *const u8,
        quote_size: u32,
        p_quote_collateral: *const c_void,
        expiration_check_date: c_long,
        p_collateral_expiration_status: *mut u32,
        p_quote_verification_result: *mut u32,
        p_qve_report_info: *mut c_void,
        supplemental_data_size: u32,
        p_supplemental_data: *mut u8,
    ) -> u32;

    let library = load_qvl(qvl_library)?;
    configure_qpl(&library, qpl_library)?;

    let verify: Symbol<SgxQvVerifyQuote> = unsafe {
        library
            .get(b"sgx_qv_verify_quote")
            .map_err(|err| format!("QVL does not export sgx_qv_verify_quote: {err}"))?
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| format!("system clock is before UNIX_EPOCH: {err}"))?
        .as_secs() as c_long;

    let quote_size: u32 = quote
        .len()
        .try_into()
        .map_err(|_| "quote is too large for sgx_qv_verify_quote".to_string())?;
    let mut collateral_expiration_status = 1u32;
    let mut quote_verification_result = SGX_QL_QV_RESULT_UNSPECIFIED;

    let status = unsafe {
        verify(
            quote.as_ptr(),
            quote_size,
            ptr::null(),
            now,
            &mut collateral_expiration_status,
            &mut quote_verification_result,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
        )
    };

    if status != SGX_QL_SUCCESS {
        return Err(format!(
            "sgx_qv_verify_quote failed: {} / 0x{status:04x}",
            quote3_error_name(status)
        ));
    }
    if collateral_expiration_status != 0 {
        return Err("DCAP collateral is expired".to_string());
    }
    Ok(DcapVerification {
        collateral_expiration_status,
        quote_verification_result,
    })
}

pub fn verify_policy(
    parsed: &ParsedQuote,
    expected_mrsigner: &[u8; MEASUREMENT_LEN],
    expected_mrenclave: Option<&[u8; MEASUREMENT_LEN]>,
    allow_debug: bool,
) -> Result<()> {
    star_attestation::verify_policy(
        parsed,
        &QuotePolicy {
            expected_mrenclave: expected_mrenclave.copied(),
            expected_mrsigner: Some(*expected_mrsigner),
            allow_debug,
            allow_advisory: false,
        },
    )
}

pub fn parse_hex_array<const N: usize>(input: &str, name: &str) -> Result<[u8; N]> {
    let mut cleaned = String::with_capacity(input.len());
    let mut input = input.trim();
    if let Some(stripped) = input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
    {
        input = stripped;
    }

    for ch in input.chars() {
        if ch == ':' || ch == '_' || ch.is_ascii_whitespace() {
            continue;
        }
        if !ch.is_ascii_hexdigit() {
            return Err(format!("{name} contains non-hex character {ch:?}"));
        }
        cleaned.push(ch);
    }

    if cleaned.len() != N * 2 {
        return Err(format!(
            "{name} must be {} hex characters for {N} bytes, got {}",
            N * 2,
            cleaned.len()
        ));
    }

    let mut out = [0u8; N];
    for (index, byte) in out.iter_mut().enumerate() {
        let offset = index * 2;
        *byte = u8::from_str_radix(&cleaned[offset..offset + 2], 16)
            .map_err(|err| format!("parse {name} hex: {err}"))?;
    }
    Ok(out)
}

pub fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>> {
    let input = input.trim();
    URL_SAFE
        .decode(input)
        .or_else(|_| URL_SAFE_NO_PAD.decode(input))
        .map_err(|err| format!("decode base64: {err}"))
}

pub fn decode_quote_input(input: &[u8]) -> Result<Vec<u8>> {
    if parse_quote(input).is_ok() {
        return Ok(input.to_vec());
    }

    let text = std::str::from_utf8(input)
        .map_err(|err| format!("stdin is neither a raw quote nor UTF-8 base64 text: {err}"))?;

    let trimmed = text.trim();
    if !trimmed.is_empty() {
        if let Ok(quote) = decode_base64(trimmed) {
            if parse_quote(&quote).is_ok() {
                return Ok(quote);
            }
        }
    }

    for token in text.split_whitespace().rev() {
        for candidate in quote_token_candidates(token) {
            if let Ok(quote) = decode_base64(candidate) {
                if parse_quote(&quote).is_ok() {
                    return Ok(quote);
                }
            }
        }
    }

    Err("could not find a parseable SGX quote in stdin".to_string())
}

fn quote_token_candidates(token: &str) -> impl Iterator<Item = &str> {
    let token = token.trim_matches(|ch: char| {
        ch == '"' || ch == '\'' || ch == ',' || ch == ';' || ch == '[' || ch == ']'
    });
    [
        Some(token),
        token.split_once('=').map(|(_, value)| value),
        token.rsplit_once(':').map(|(_, value)| value),
    ]
    .into_iter()
    .flatten()
    .filter(|candidate| !candidate.is_empty())
}

fn load_qvl(qvl_library: Option<&Path>) -> Result<Library> {
    let env_path = std::env::var_os("SGX_DCAP_QVL").map(PathBuf::from);
    let path = qvl_library.map(PathBuf::from).or(env_path);

    match path {
        Some(path) => unsafe { Library::new(&path) }
            .map_err(|err| format!("load QVL {}: {err}", path.display())),
        None => unsafe { Library::new("libsgx_dcap_quoteverify.so") }.map_err(|err| {
            format!("load QVL libsgx_dcap_quoteverify.so: {err}; set SGX_DCAP_QVL or pass --qvl")
        }),
    }
}

fn configure_qpl(library: &Library, qpl_library: Option<&Path>) -> Result<()> {
    type SgxQvSetPath = unsafe extern "C" fn(path_type: i32, p_path: *const i8) -> u32;

    let env_path = std::env::var_os("SGX_DCAP_QPL").map(PathBuf::from);
    let Some(path) = qpl_library.map(PathBuf::from).or(env_path) else {
        return Ok(());
    };

    let set_path: Symbol<SgxQvSetPath> = unsafe {
        library
            .get(b"sgx_qv_set_path")
            .map_err(|err| format!("QVL does not export sgx_qv_set_path: {err}"))?
    };
    let path = CString::new(path.to_string_lossy().as_bytes())
        .map_err(|_| "QPL path contains a NUL byte".to_string())?;
    let status = unsafe { set_path(SGX_QV_QPL_PATH, path.as_ptr()) };

    if status == SGX_QL_SUCCESS {
        Ok(())
    } else {
        Err(format!(
            "sgx_qv_set_path(SGX_QV_QPL_PATH) failed: {} / 0x{status:04x}",
            quote3_error_name(status)
        ))
    }
}

pub fn qv_result_name(value: u32) -> &'static str {
    match value {
        SGX_QL_QV_RESULT_OK => "OK",
        SGX_QL_QV_RESULT_CONFIG_NEEDED => "CONFIG_NEEDED",
        SGX_QL_QV_RESULT_OUT_OF_DATE => "OUT_OF_DATE",
        SGX_QL_QV_RESULT_OUT_OF_DATE_CONFIG_NEEDED => "OUT_OF_DATE_CONFIG_NEEDED",
        SGX_QL_QV_RESULT_INVALID_SIGNATURE => "INVALID_SIGNATURE",
        SGX_QL_QV_RESULT_REVOKED => "REVOKED",
        SGX_QL_QV_RESULT_UNSPECIFIED => "UNSPECIFIED",
        SGX_QL_QV_RESULT_SW_HARDENING_NEEDED => "SW_HARDENING_NEEDED",
        SGX_QL_QV_RESULT_CONFIG_AND_SW_HARDENING_NEEDED => "CONFIG_AND_SW_HARDENING_NEEDED",
        SGX_QL_QV_RESULT_TD_RELAUNCH_ADVISED => "TD_RELAUNCH_ADVISED",
        SGX_QL_QV_RESULT_TD_RELAUNCH_ADVISED_CONFIG_NEEDED => "TD_RELAUNCH_ADVISED_CONFIG_NEEDED",
        _ => "UNKNOWN",
    }
}

pub fn qv_result_is_ok(value: u32) -> bool {
    value == SGX_QL_QV_RESULT_OK
}

pub fn qv_result_is_advisory(value: u32) -> bool {
    matches!(
        value,
        SGX_QL_QV_RESULT_CONFIG_NEEDED
            | SGX_QL_QV_RESULT_OUT_OF_DATE
            | SGX_QL_QV_RESULT_OUT_OF_DATE_CONFIG_NEEDED
            | SGX_QL_QV_RESULT_SW_HARDENING_NEEDED
            | SGX_QL_QV_RESULT_CONFIG_AND_SW_HARDENING_NEEDED
    )
}

pub fn validate_qv_result(result: u32, allow_advisory: bool) -> Result<()> {
    if qv_result_is_ok(result) || (allow_advisory && qv_result_is_advisory(result)) {
        return Ok(());
    }

    Err(format!(
        "quote verification result is {} / 0x{result:08x}",
        qv_result_name(result)
    ))
}

pub fn quote3_error_name(value: u32) -> &'static str {
    match value {
        0x0000 => "SGX_QL_SUCCESS",
        0xe001 => "SGX_QL_ERROR_UNEXPECTED",
        0xe002 => "SGX_QL_ERROR_INVALID_PARAMETER",
        0xe003 => "SGX_QL_ERROR_OUT_OF_MEMORY",
        0xe00d => "SGX_QL_INTERFACE_UNAVAILABLE",
        0xe011 => "SGX_QL_NO_PLATFORM_CERT_DATA",
        0xe019 => "SGX_QL_NETWORK_ERROR",
        0xe01b => "SGX_QL_NO_QUOTE_COLLATERAL_DATA",
        0xe01d => "SGX_QL_QUOTE_FORMAT_UNSUPPORTED",
        0xe022 => "SGX_QL_PCK_CERT_CHAIN_ERROR",
        0xe023 => "SGX_QL_TCBINFO_UNSUPPORTED_FORMAT",
        0xe024 => "SGX_QL_TCBINFO_MISMATCH",
        0xe025 => "SGX_QL_QEIDENTITY_UNSUPPORTED_FORMAT",
        0xe026 => "SGX_QL_QEIDENTITY_MISMATCH",
        0xe034 => "SGX_QL_UNABLE_TO_GET_COLLATERAL",
        0xe03b => "SGX_QL_ERROR_QVL_QVE_MISMATCH",
        0xe052 => "SGX_QL_PSW_NOT_AVAILABLE",
        0xe053 => "SGX_QL_COLLATERAL_VERSION_NOT_SUPPORTED",
        0xe064 => "SGX_QL_SUPPLEMENTAL_DATA_VERSION_NOT_SUPPORTED",
        0xe065 => "SGX_QL_ROOT_CA_UNTRUSTED",
        0xe066 => "SGX_QL_TCB_NOT_SUPPORTED",
        0xe068 => "SGX_QL_RESULT_INVALID_SIGNATURE",
        _ => "UNKNOWN",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUOTE_HEADER_LEN: usize = 48;
    const SGX_REPORT_BODY_LEN: usize = 384;
    const QUOTE_V3: u16 = 3;
    const SGX_QL_ALG_ECDSA_P256: u16 = 2;

    fn fake_quote(
        mrenclave: [u8; MEASUREMENT_LEN],
        mrsigner: [u8; MEASUREMENT_LEN],
        public_key: [u8; PUBLIC_KEY_LEN],
        debug: bool,
    ) -> Vec<u8> {
        let mut quote = vec![0u8; QUOTE_HEADER_LEN + SGX_REPORT_BODY_LEN];
        quote[0..2].copy_from_slice(&QUOTE_V3.to_le_bytes());
        quote[2..4].copy_from_slice(&SGX_QL_ALG_ECDSA_P256.to_le_bytes());

        let report = QUOTE_HEADER_LEN;
        let flags = if debug { 0x2u64 } else { 0 };
        quote[report + 48..report + 56].copy_from_slice(&flags.to_le_bytes());
        quote[report + 64..report + 96].copy_from_slice(&mrenclave);
        quote[report + 128..report + 160].copy_from_slice(&mrsigner);
        quote[report + 320..report + 352].copy_from_slice(&public_key);
        quote
    }

    #[test]
    fn parses_quote_report_fields() {
        let mrenclave = [1u8; MEASUREMENT_LEN];
        let mrsigner = [2u8; MEASUREMENT_LEN];
        let public_key = [3u8; PUBLIC_KEY_LEN];

        let parsed =
            parse_quote(&fake_quote(mrenclave, mrsigner, public_key, true)).expect("parse quote");

        assert_eq!(parsed.mrenclave, mrenclave);
        assert_eq!(parsed.mrsigner, mrsigner);
        assert_eq!(
            public_key_from_report_data(&parsed).expect("public key"),
            public_key
        );
        assert!(parsed.debug);
    }

    #[test]
    fn policy_rejects_untrusted_debug_quote() {
        let mrenclave = [1u8; MEASUREMENT_LEN];
        let mrsigner = [2u8; MEASUREMENT_LEN];
        let public_key = [3u8; PUBLIC_KEY_LEN];
        let parsed =
            parse_quote(&fake_quote(mrenclave, mrsigner, public_key, true)).expect("parse quote");

        let err = verify_policy(&parsed, &mrsigner, Some(&mrenclave), false)
            .expect_err("debug quote should be rejected");

        assert!(err.contains("debug enclave"));
    }

    #[test]
    fn hex_round_trips_and_accepts_separators() {
        let parsed = parse_hex_array::<4>("0x12:34_56 78", "test").expect("parse hex");
        assert_eq!(parsed, [0x12, 0x34, 0x56, 0x78]);
        assert_eq!(hex(&parsed), "12345678");
    }

    #[test]
    fn names_dcap_qvl_results() {
        assert_eq!(qv_result_name(SGX_QL_QV_RESULT_OK), "OK");
        assert_eq!(qv_result_name(SGX_QL_QV_RESULT_OUT_OF_DATE), "OUT_OF_DATE");
        assert!(qv_result_is_advisory(SGX_QL_QV_RESULT_OUT_OF_DATE));
        assert!(!qv_result_is_advisory(SGX_QL_QV_RESULT_INVALID_SIGNATURE));
    }

    #[test]
    fn extracts_base64_quote_from_startup_log_line() {
        let quote = fake_quote(
            [1u8; MEASUREMENT_LEN],
            [2u8; MEASUREMENT_LEN],
            [3u8; PUBLIC_KEY_LEN],
            false,
        );
        let input = format!(
            "STAR_ATTESTATION_QUOTE_BASE64={}\n",
            URL_SAFE.encode(&quote)
        );

        let decoded = decode_quote_input(input.as_bytes()).expect("decode quote input");

        assert_eq!(decoded, quote);
    }

    #[test]
    fn extracts_padded_base64_quote_from_startup_log_line() {
        let mut quote = fake_quote(
            [1u8; MEASUREMENT_LEN],
            [2u8; MEASUREMENT_LEN],
            [3u8; PUBLIC_KEY_LEN],
            false,
        );
        quote.push(9);
        let input = format!(
            "STAR_ATTESTATION_QUOTE_BASE64={}\n",
            URL_SAFE.encode(&quote)
        );

        let decoded = decode_quote_input(input.as_bytes()).expect("decode quote input");

        assert_eq!(decoded, quote);
    }
}
