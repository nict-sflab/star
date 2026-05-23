#![cfg(any(unix, windows, libloading_docs))]

use std::ffi::c_void;
use std::path::PathBuf;
use std::ptr;
use std::vec::Vec;
use std::string::ToString;

#[cfg(unix)]
use libloading::os::unix::{Library as UnixLibrary, RTLD_GLOBAL, RTLD_NOW};
#[cfg(any(unix, windows, libloading_docs))]
use libloading::{Library, Symbol};

use crate::Result;

pub fn raw_evidence_for_verifier(quote: &[u8]) -> Result<Vec<u8>> {
    let _sgx_runtime = preload_sgx_runtime().map_err(|e| { eprintln!("preload_sgx_runtime failed: {}", e); e })?;
    let library = load_qvl().map_err(|e| { eprintln!("load_qvl failed: {}", e); e })?;

    let quote_size: u32 = quote
        .len()
        .try_into()
        .map_err(|_| "quote is too large for QVL".to_string())?;

    let mut collateral = ptr::null_mut();
    let mut collateral_size = 0u32;
    if let Ok(get_collateral) = unsafe { library.get::<Symbol<TeeQvGetCollateral>>(b"tee_qv_get_collateral") } {
        unsafe {
            get_collateral(
                quote.as_ptr(),
                quote_size,
                &mut collateral,
                &mut collateral_size,
            );
        }
    }

    if collateral.is_null() {
        return Err("failed to fetch collateral using QPL".to_string());
    }

    let collateral_slice = unsafe { std::slice::from_raw_parts(collateral as *const u8, collateral_size as usize) };
    
    let encoded = crate::encode_raw_quote_evidence(&crate::RawQuoteEvidence {
        quote,
        collateral: collateral_slice,
    });

    if let Ok(free_collateral) = unsafe { library.get::<Symbol<TeeQvFreeCollateral>>(b"tee_qv_free_collateral") } {
        unsafe { free_collateral(collateral) };
    }

    Ok(encoded)
}

type TeeQvGetCollateral = unsafe extern "C" fn(
    p_quote: *const u8,
    quote_size: u32,
    pp_quote_collateral: *mut *mut c_void,
    p_collateral_size: *mut u32,
) -> u32;

type TeeQvFreeCollateral = unsafe extern "C" fn(p_quote_collateral: *mut c_void) -> u32;

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
        }).inspect(|_| eprintln!("DEBUG: Loaded {} from {}", name, path.display()));
    }

    for path in candidate_library_paths(name) {
        if !path.is_file() {
            continue;
        }

        if let Ok(library) = unsafe { UnixLibrary::open(Some(path.as_os_str()), flags) } {
            eprintln!("DEBUG: Loaded {} from candidate path {}", name, path.display());
            return Ok(library);
        }
    }

    if let Ok(library) = unsafe { UnixLibrary::open(Some(name), flags) } {
        eprintln!("DEBUG: Loaded {} from system path", name);
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
