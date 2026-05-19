use std::ffi::{CString, c_void};
use std::os::raw::c_long;
use std::path::{Path, PathBuf};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

use libloading::{Library, Symbol};
use rand::RngCore;
use star_attestation::{
    QVE_REPORT_INFO_LEN, QVE_TARGET_INFO_LEN, QveEvidence, encode_qve_evidence,
};

use crate::enclave::EnclaveError;

const SGX_QL_SUCCESS: u32 = 0x0000;
const SGX_QL_QV_RESULT_UNSPECIFIED: u32 = 0x0000a006;
const SGX_QV_QPL_PATH: i32 = 1;
const QVE_NONCE_OFFSET: usize = 0;
const QVE_NONCE_LEN: usize = 16;
const QVE_TARGET_INFO_OFFSET: usize = QVE_NONCE_LEN;

pub fn evidence_for_verifier(
    quote: &[u8],
    verifier_target_info: &[u8],
) -> Result<Vec<u8>, EnclaveError> {
    if verifier_target_info.len() != QVE_TARGET_INFO_LEN {
        return Err(EnclaveError::Qve(format!(
            "verifier targetinfo must be {QVE_TARGET_INFO_LEN} bytes, got {}",
            verifier_target_info.len()
        )));
    }

    let library = load_qvl()?;
    configure_qpl(&library)?;

    let supplemental_data_size = supplemental_data_size(&library)?;
    let mut supplemental_data = vec![0u8; supplemental_data_size as usize];
    let mut report_info = vec![0u8; QVE_REPORT_INFO_LEN];

    rand::rng().fill_bytes(&mut report_info[QVE_NONCE_OFFSET..QVE_NONCE_OFFSET + QVE_NONCE_LEN]);
    report_info[QVE_TARGET_INFO_OFFSET..QVE_TARGET_INFO_OFFSET + QVE_TARGET_INFO_LEN]
        .copy_from_slice(verifier_target_info);

    let verify: Symbol<SgxQvVerifyQuote> = unsafe {
        library
            .get(b"sgx_qv_verify_quote")
            .map_err(|err| EnclaveError::Qve(format!("QVL does not export verify: {err}")))?
    };

    let expiration_check_date = now_unix_timestamp()?;
    let quote_size: u32 = quote
        .len()
        .try_into()
        .map_err(|_| EnclaveError::Qve("quote is too large for QVL".to_string()))?;
    let mut collateral_expiration_status = 1u32;
    let mut quote_verification_result = SGX_QL_QV_RESULT_UNSPECIFIED;

    let status = unsafe {
        verify(
            quote.as_ptr(),
            quote_size,
            ptr::null(),
            expiration_check_date as c_long,
            &mut collateral_expiration_status,
            &mut quote_verification_result,
            report_info.as_mut_ptr() as *mut c_void,
            supplemental_data_size,
            supplemental_data.as_mut_ptr(),
        )
    };

    if status != SGX_QL_SUCCESS {
        return Err(EnclaveError::Qve(format!(
            "sgx_qv_verify_quote with QvE failed: {} / 0x{status:04x}",
            quote3_error_name(status)
        )));
    }

    encode_qve_evidence(&QveEvidence {
        quote,
        report_info: &report_info,
        expiration_check_date,
        collateral_expiration_status,
        quote_verification_result,
        supplemental_data: &supplemental_data,
    })
    .map_err(EnclaveError::Qve)
}

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

fn supplemental_data_size(library: &Library) -> Result<u32, EnclaveError> {
    type SgxQvGetQuoteSupplementalDataSize = unsafe extern "C" fn(p_data_size: *mut u32) -> u32;

    let get_size: Symbol<SgxQvGetQuoteSupplementalDataSize> = unsafe {
        library
            .get(b"sgx_qv_get_quote_supplemental_data_size")
            .map_err(|err| {
                EnclaveError::Qve(format!("QVL does not export supplemental size: {err}"))
            })?
    };

    let mut size = 0u32;
    let status = unsafe { get_size(&mut size) };
    if status == SGX_QL_SUCCESS {
        Ok(size)
    } else {
        Err(EnclaveError::Qve(format!(
            "sgx_qv_get_quote_supplemental_data_size failed: {} / 0x{status:04x}",
            quote3_error_name(status)
        )))
    }
}

fn load_qvl() -> Result<Library, EnclaveError> {
    let path = std::env::var_os("SGX_DCAP_QVL").map(PathBuf::from);
    match path {
        Some(path) => unsafe { Library::new(&path) }
            .map_err(|err| EnclaveError::Qve(format!("load QVL {}: {err}", path.display()))),
        None => unsafe { Library::new("libsgx_dcap_quoteverify.so") }
            .or_else(|_| unsafe {
                Library::new("/run/current-system/sw/lib/libsgx_dcap_quoteverify.so")
            })
            .map_err(|err| {
                EnclaveError::Qve(format!(
                    "load QVL libsgx_dcap_quoteverify.so: {err}; set SGX_DCAP_QVL"
                ))
            }),
    }
}

fn configure_qpl(library: &Library) -> Result<(), EnclaveError> {
    type SgxQvSetPath = unsafe extern "C" fn(path_type: i32, p_path: *const i8) -> u32;

    let Some(path) = std::env::var_os("SGX_DCAP_QPL").map(PathBuf::from) else {
        return Ok(());
    };

    let set_path: Symbol<SgxQvSetPath> = unsafe {
        library
            .get(b"sgx_qv_set_path")
            .map_err(|err| EnclaveError::Qve(format!("QVL does not export set_path: {err}")))?
    };
    let path = cstring_path(&path)?;
    let status = unsafe { set_path(SGX_QV_QPL_PATH, path.as_ptr()) };

    if status == SGX_QL_SUCCESS {
        Ok(())
    } else {
        Err(EnclaveError::Qve(format!(
            "sgx_qv_set_path(SGX_QV_QPL_PATH) failed: {} / 0x{status:04x}",
            quote3_error_name(status)
        )))
    }
}

fn cstring_path(path: &Path) -> Result<CString, EnclaveError> {
    CString::new(path.to_string_lossy().as_bytes())
        .map_err(|_| EnclaveError::Qve("QPL path contains a NUL byte".to_string()))
}

fn now_unix_timestamp() -> Result<i64, EnclaveError> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| EnclaveError::Qve(format!("system clock is before UNIX_EPOCH: {err}")))?
        .as_secs();
    seconds
        .try_into()
        .map_err(|_| EnclaveError::Qve("UNIX timestamp is too large".to_string()))
}

fn quote3_error_name(value: u32) -> &'static str {
    match value {
        0x0000 => "SGX_QL_SUCCESS",
        0xe001 => "SGX_QL_ERROR_UNEXPECTED",
        0xe002 => "SGX_QL_ERROR_INVALID_PARAMETER",
        0xe003 => "SGX_QL_ERROR_OUT_OF_MEMORY",
        0xe00d => "SGX_QL_INTERFACE_UNAVAILABLE",
        0xe011 => "SGX_QL_NO_PLATFORM_CERT_DATA",
        0xe017 => "SGX_QL_UNABLE_TO_GENERATE_QE_REPORT",
        0xe019 => "SGX_QL_NETWORK_ERROR",
        0xe01b => "SGX_QL_NO_QUOTE_COLLATERAL_DATA",
        0xe01d => "SGX_QL_QUOTE_FORMAT_UNSUPPORTED",
        0xe034 => "SGX_QL_UNABLE_TO_GET_COLLATERAL",
        0xe03b => "SGX_QL_ERROR_QVL_QVE_MISMATCH",
        0xe064 => "SGX_QL_SUPPLEMENTAL_DATA_VERSION_NOT_SUPPORTED",
        _ => "UNKNOWN",
    }
}
