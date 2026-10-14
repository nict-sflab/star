//! Fortanix SGX library ABI and access to host buffers.
//! Returns 0 on success, 2 on operation errors, and 5 if the output buffer is too small.
use enclave::Enclave;
use std::{
    ptr, slice,
    sync::{OnceLock, RwLock},
};

const OK: u64 = 0;
const ERROR: u64 = 2;
const BUFFER_TOO_SMALL: u64 = 5;
const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

static ENCLAVE: OnceLock<RwLock<Enclave>> = OnceLock::new();

#[allow(improper_ctypes_definitions)]
#[no_mangle]
// The unused fourth register is reserved by the Fortanix library ABI.
pub extern "C" fn entry(
    op: u64,
    input_ptr: u64,
    input_len: u64,
    _: u64,
    output_ptr: u64,
    output_len: u64,
) -> (u64, u64) {
    let Some(input) = UserBuffer::new(input_ptr, input_len) else {
        return (ERROR, 0);
    };
    let Some(output) = UserBuffer::new(output_ptr, output_len) else {
        return (ERROR, 0);
    };
    // Own the request before writing a response, even if the host buffers overlap.
    let request = if input.len == 0 {
        Vec::new()
    } else {
        unsafe { slice::from_raw_parts(input.ptr, input.len) }.to_vec()
    };
    let enclave = ENCLAVE.get_or_init(|| RwLock::new(Enclave::default()));
    let (status, response) = match crate::protocol::dispatch(enclave, op, &request) {
        Ok(response) => (OK, response),
        Err(err) => (ERROR, err.into_bytes()),
    };
    if response.len() > output.len {
        return (BUFFER_TOO_SMALL, response.len() as u64);
    }
    if !response.is_empty() {
        unsafe { ptr::copy_nonoverlapping(response.as_ptr(), output.ptr, response.len()) };
    }
    (status, response.len() as u64)
}

struct UserBuffer {
    ptr: *mut u8,
    len: usize,
}

impl UserBuffer {
    fn new(ptr: u64, len: u64) -> Option<Self> {
        let len = usize::try_from(len).ok()?;
        if len > MAX_PAYLOAD_LEN {
            return None;
        }
        if len != 0 {
            if ptr == 0 {
                return None;
            }
            let start = usize::try_from(ptr).ok()?;
            let end = start.checked_add(len)?;
            let base: usize;
            unsafe {
                std::arch::asm!(
                    "lea IMAGE_BASE(%rip), {}",
                    lateout(reg) base,
                    options(att_syntax, nostack, preserves_flags, nomem, pure),
                );
            }
            let enclave_end = base.checked_add(unsafe { ENCLAVE_SIZE })?;
            if start < enclave_end && end > base {
                return None;
            }
        }
        Some(Self {
            ptr: ptr as *mut u8,
            len,
        })
    }
}

unsafe extern "C" {
    static ENCLAVE_SIZE: usize;
}

#[path = "sgx_test.rs"]
#[cfg(all(test, not(target_env = "sgx")))]
mod tests;
