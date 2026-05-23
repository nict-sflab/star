#![cfg(any(unix, windows, libloading_docs))]

use std::ffi::{CStr, CString, c_void};
use std::os::raw::{c_char, c_int, c_long};
use std::path::{Path, PathBuf};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;
use std::string::{String, ToString};

#[cfg(unix)]
use libloading::os::unix::{Library as UnixLibrary, RTLD_GLOBAL, RTLD_NOW};
#[cfg(any(unix, windows, libloading_docs))]
use libloading::{Library, Symbol};
use rand::{TryRngCore, rngs::OsRng};

use crate::{
    QVE_REPORT_INFO_LEN, QVE_TARGET_INFO_LEN, QveEvidence, Result, encode_qve_evidence,
};

const SGX_QL_SUCCESS: u32 = 0x0000;
const SGX_QL_QV_RESULT_UNSPECIFIED: u32 = 0x0000a006;
const SGX_QV_QVE_PATH: i32 = 0;
const SGX_QV_QPL_PATH: i32 = 1;
const QVE_NONCE_OFFSET: usize = 0;
const QVE_NONCE_LEN: usize = 16;
const QVE_TARGET_INFO_OFFSET: usize = QVE_NONCE_LEN;

pub fn evidence_for_verifier(
    quote: &[u8],
    verifier_target_info: &[u8],
) -> Result<Vec<u8>> {
    if verifier_target_info.len() != QVE_TARGET_INFO_LEN {
        return Err(format!(
            "verifier targetinfo must be {QVE_TARGET_INFO_LEN} bytes, got {}",
            verifier_target_info.len()
        ));
    }

    let _sgx_runtime = preload_sgx_runtime().map_err(|e| { eprintln!("preload_sgx_runtime failed: {}", e); e })?;
    let library = load_qvl().map_err(|e| { eprintln!("load_qvl failed: {}", e); e })?;
    configure_qve_trace(&library);
    configure_qve(&library).map_err(|e| { eprintln!("configure_qve failed: {}", e); e })?;
    configure_qpl(&library).map_err(|e| { eprintln!("configure_qpl failed: {}", e); e })?;

    let mut supplemental_data_size = 0u32;
    if let Ok(get_size) = unsafe { library.get::<Symbol<unsafe extern "C" fn(*mut u32) -> u32>>(b"sgx_qv_get_quote_supplemental_data_size") } {
        unsafe { get_size(&mut supplemental_data_size) };
    }
    if supplemental_data_size == 0 {
        supplemental_data_size = 2048; // fallback size
    }
    let mut supplemental_data = vec![0u8; supplemental_data_size as usize];
    let mut report_info = vec![0u8; QVE_REPORT_INFO_LEN];

    OsRng
        .try_fill_bytes(&mut report_info[QVE_NONCE_OFFSET..QVE_NONCE_OFFSET + QVE_NONCE_LEN])
        .map_err(|err| format!("generate QvE nonce: {err}"))?;
    report_info[QVE_TARGET_INFO_OFFSET..QVE_TARGET_INFO_OFFSET + QVE_TARGET_INFO_LEN]
        .copy_from_slice(verifier_target_info);

    let verify: Symbol<SgxQvVerifyQuote> = unsafe {
        library
            .get(b"sgx_qv_verify_quote")
            .map_err(|err| format!("QVL does not export verify: {err}"))?
    };

    let expiration_check_date = now_unix_timestamp()?;
    let quote_size: u32 = quote
        .len()
        .try_into()
        .map_err(|_| "quote is too large for QVL".to_string())?;
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
        let untrusted_status = verify_without_qve_report(&library, &verify, quote, quote_size);
        let msg = format!(
            "sgx_qv_verify_quote with QvE failed: {} / 0x{status:04x}; untrusted QVL status: {untrusted_status}",
            quote3_error_name(status),
        );
        eprintln!("ERROR: {}", msg);
        return Err(msg);
    }

    encode_qve_evidence(&QveEvidence {
        quote,
        report_info: &report_info,
        expiration_check_date,
        collateral_expiration_status,
        quote_verification_result,
        supplemental_data: &supplemental_data,
    })
}

pub fn evidence_for_untrusted_verifier(quote: &[u8]) -> Vec<u8> {
    panic!("Fallback triggered!");
}

fn verify_without_qve_report(
    library: &Library,
    verify: &Symbol<SgxQvVerifyQuote>,
    quote: &[u8],
    quote_size: u32,
) -> String {
    let expiration_check_date = match now_unix_timestamp() {
        Ok(timestamp) => timestamp,
        Err(err) => return format!("not run ({err})"),
    };
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
            ptr::null_mut(),
            0,
            ptr::null_mut(),
        )
    };

    format!(
        "{} / 0x{status:04x}, {}, {}, collateral_expiration_status={collateral_expiration_status}, quote_verification_result=0x{quote_verification_result:08x}",
        quote3_error_name(status),
        describe_quote(quote),
        describe_collateral(library, quote, quote_size),
    )
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

type TeeQvGetCollateral = unsafe extern "C" fn(
    p_quote: *const u8,
    quote_size: u32,
    pp_quote_collateral: *mut *mut c_void,
    p_collateral_size: *mut u32,
) -> u32;

type TeeQvFreeCollateral = unsafe extern "C" fn(p_quote_collateral: *mut c_void) -> u32;

type TraceLoggerCallback = Option<unsafe extern "C" fn(c_int, *const c_char)>;

fn configure_qve_trace(library: &Library) {
    if !truthy_env("STAR_QVE_TRACE") {
        return;
    }

    unsafe {
        if let Ok(loglevel) = library.get::<*mut c_int>(b"sgx_trace_loglevel") {
            **loglevel = 4;
        }
        if let Ok(callback) = library.get::<*mut TraceLoggerCallback>(b"sgx_trace_logger_callback")
        {
            **callback = Some(qve_trace_logger);
        }
    }
}

unsafe extern "C" fn qve_trace_logger(level: c_int, message: *const c_char) {
    if message.is_null() {
        eprintln!("[qve:{level}] <null>");
        return;
    }

    let message = unsafe { CStr::from_ptr(message) }.to_string_lossy();
    eprint!("[qve:{level}] {message}");
}

fn truthy_env(name: &str) -> bool {
    matches!(
        std::env::var(name).as_deref(),
        Ok("1" | "true" | "TRUE" | "yes" | "YES")
    )
}

fn describe_quote(quote: &[u8]) -> String {
    let Some(version) = read_u16_le(quote, 0) else {
        return "quote=truncated".to_string();
    };
    let Some(attestation_key_type) = read_u16_le(quote, 2) else {
        return format!("quote_version={version}, quote=truncated");
    };
    let Some(tee_type) = read_u32_le(quote, 4) else {
        return format!(
            "quote_version={version}, attestation_key_type={attestation_key_type}, quote=truncated"
        );
    };

    format!(
        "quote_version={version}, attestation_key_type={attestation_key_type}, tee_type={tee_type}"
    )
}

fn describe_collateral(library: &Library, quote: &[u8], quote_size: u32) -> String {
    let get_collateral: Symbol<TeeQvGetCollateral> =
        match unsafe { library.get(b"tee_qv_get_collateral") } {
            Ok(symbol) => symbol,
            Err(err) => return format!("collateral=unavailable({err})"),
        };
    let free_collateral: Symbol<TeeQvFreeCollateral> =
        match unsafe { library.get(b"tee_qv_free_collateral") } {
            Ok(symbol) => symbol,
            Err(err) => return format!("collateral=unavailable({err})"),
        };

    let mut collateral = ptr::null_mut();
    let mut collateral_size = 0u32;
    let status = unsafe {
        get_collateral(
            quote.as_ptr(),
            quote_size,
            &mut collateral,
            &mut collateral_size,
        )
    };
    if status != SGX_QL_SUCCESS {
        return format!(
            "collateral_status={} / 0x{status:04x}",
            quote3_error_name(status)
        );
    }
    if collateral.is_null() || collateral_size < 8 {
        return format!("collateral_size={collateral_size}");
    }

    let version = unsafe { ptr::read_unaligned(collateral.cast::<u32>()) };
    let tee_type = unsafe { ptr::read_unaligned(collateral.cast::<u32>().add(1)) };
    let free_status = unsafe { free_collateral(collateral) };

    format!(
        "collateral_version={version}, collateral_tee_type={tee_type}, collateral_size={collateral_size}, collateral_free_status={} / 0x{free_status:04x}",
        quote3_error_name(free_status)
    )
}

fn read_u16_le(bytes: &[u8], offset: usize) -> Option<u16> {
    bytes
        .get(offset..offset + 2)
        .and_then(|value| value.try_into().ok())
        .map(u16::from_le_bytes)
}

fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    bytes
        .get(offset..offset + 4)
        .and_then(|value| value.try_into().ok())
        .map(u32::from_le_bytes)
}

#[cfg(unix)]
fn preload_sgx_runtime() -> Result<Vec<UnixLibrary>> {
    Ok(vec![load_global_library("libsgx_urts.so.2", "SGX_URTS")?])
}

#[cfg(not(unix))]
fn preload_sgx_runtime() -> Result<Vec<()>> {
    Ok(Vec::new())
}

#[cfg(unix)]
fn load_global_library(name: &str, env_name: &str) -> Result<UnixLibrary> {
    let flags = RTLD_NOW | RTLD_GLOBAL;

    if let Some(path) = std::env::var_os(env_name).map(PathBuf::from) {
        return unsafe { UnixLibrary::open(Some(path.as_os_str()), flags) }.map_err(|err| {
            format!(
                "load {name} from {env_name}={}: {err}",
                path.display()
            )
        });
    }

    for path in candidate_library_paths(name) {
        if !path.is_file() {
            continue;
        }

        if let Ok(library) = unsafe { UnixLibrary::open(Some(path.as_os_str()), flags) } {
            return Ok(library);
        }
    }

    if let Ok(library) = unsafe { UnixLibrary::open(Some(name), flags) } {
        return Ok(library);
    }

    Err(format!(
        "load {name}: set {env_name} or make the SGX runtime library visible"
    ))
}

#[cfg(unix)]
fn candidate_library_paths(name: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for env_name in ["SGX_DCAP_QVL", "SGX_DCAP_QPL"] {
        if let Some(path) = std::env::var_os(env_name).map(PathBuf::from) {
            if let Some(parent) = path.parent() {
                paths.push(parent.join(name));
            }
        }
    }

    paths.extend(nix_sgx_psw_library_paths(name));

    for dir in [
        "/run/current-system/sw/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib64",
        "/usr/lib",
    ] {
        paths.push(PathBuf::from(dir).join(name));
    }

    paths
}

#[cfg(unix)]
fn nix_sgx_psw_library_paths(name: &str) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir("/nix/store") else {
        return Vec::new();
    };

    let mut paths = Vec::new();
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.contains("sgx-psw-") {
            paths.push(entry.path().join("lib").join(name));
        }
    }

    paths.sort();
    paths.reverse();
    paths
}

fn load_qvl() -> Result<Library> {
    let path = std::env::var_os("SGX_DCAP_QVL").map(PathBuf::from);
    match path {
        Some(path) => unsafe { Library::new(&path) }
            .map_err(|err| format!("load QVL {}: {err}", path.display())),
        None => unsafe { Library::new("libsgx_dcap_quoteverify.so") }
            .or_else(|_| unsafe {
                Library::new("/run/current-system/sw/lib/libsgx_dcap_quoteverify.so")
            })
            .map_err(|err| {
                format!(
                    "load QVL libsgx_dcap_quoteverify.so: {err}; set SGX_DCAP_QVL"
                )
            }),
    }
}

fn configure_qve(library: &Library) -> Result<()> {
    let Some(path) = qve_enclave_path() else {
        return Err(
            "Intel QvE enclave libsgx_qve.signed.so is not installed; set SGX_DCAP_QVE".to_string(),
        );
    };

    configure_qv_path(library, SGX_QV_QVE_PATH, &path, "SGX_QV_QVE_PATH")
}

fn configure_qpl(library: &Library) -> Result<()> {
    let Some(path) = std::env::var_os("SGX_DCAP_QPL").map(PathBuf::from) else {
        return Ok(());
    };

    configure_qv_path(library, SGX_QV_QPL_PATH, &path, "SGX_QV_QPL_PATH")
}

fn configure_qv_path(
    library: &Library,
    path_type: i32,
    path: &Path,
    path_type_name: &str,
) -> Result<()> {
    type SgxQvSetPath = unsafe extern "C" fn(path_type: i32, p_path: *const i8) -> u32;

    let set_path: Symbol<SgxQvSetPath> = unsafe {
        library
            .get(b"sgx_qv_set_path")
            .map_err(|err| format!("QVL does not export set_path: {err}"))?
    };
    let path = cstring_path(path)?;
    let status = unsafe { set_path(path_type, path.as_ptr()) };

    if status == SGX_QL_SUCCESS {
        Ok(())
    } else {
        Err(format!(
            "sgx_qv_set_path({path_type_name}) failed: {} / 0x{status:04x}",
            quote3_error_name(status)
        ))
    }
}

pub fn qve_enclave_path() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("SGX_DCAP_QVE").map(PathBuf::from) {
        if path.is_file() {
            return Some(path);
        }
    }

    for name in ["libsgx_qve.signed.so.1", "libsgx_qve.signed.so"] {
        for dir in library_search_dirs() {
            let path = dir.join(name);
            if path.is_file() {
                return Some(path);
            }
        }
    }

    None
}

fn library_search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    for env_name in ["SGX_DCAP_QVL", "SGX_DCAP_QPL"] {
        if let Some(path) = std::env::var_os(env_name).map(PathBuf::from) {
            if let Some(parent) = path.parent() {
                dirs.push(parent.to_path_buf());
            }
        }
    }

    if let Some(ld_library_path) = std::env::var_os("LD_LIBRARY_PATH") {
        dirs.extend(std::env::split_paths(&ld_library_path));
    }

    for dir in [
        "/run/current-system/sw/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib64",
        "/usr/lib",
    ] {
        dirs.push(PathBuf::from(dir));
    }

    dirs
}

fn cstring_path(path: &Path) -> Result<CString> {
    CString::new(path.to_string_lossy().as_bytes())
        .map_err(|_| "QPL path contains a NUL byte".to_string())
}

fn now_unix_timestamp() -> Result<i64> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| format!("system clock is before UNIX_EPOCH: {err}"))?
        .as_secs();
    seconds
        .try_into()
        .map_err(|_| "UNIX timestamp is too large".to_string())
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
