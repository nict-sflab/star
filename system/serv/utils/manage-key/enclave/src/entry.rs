use crate::KeyManager;
use std::{
    slice,
    sync::{OnceLock, RwLock},
};
const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

const OP_INIT: u8 = 0;
const OP_PUBLIC_KEY: u8 = 1;
const OP_ATTEST: u8 = 4;
const OP_EXPORT_KEYS: u8 = 5;

const STATUS_OK: u8 = 0;
const STATUS_BAD_REQUEST: u8 = 2;
const STATUS_PANIC: u8 = 4;
const STATUS_BUFFER_TOO_SMALL: u8 = 5;

static LIBRARY_APP: OnceLock<RwLock<Option<KeyManager>>> = OnceLock::new();

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn entry(
    op: u64,
    input_ptr: u64,
    input_len: u64,
    _ignore: u64,
    output_ptr: u64,
    output_len: u64,
) -> (u64, u64) {
    let op = match u8::try_from(op) {
        Ok(op) => op,
        Err(_) => return (STATUS_BAD_REQUEST as u64, 0),
    };

    let input_len = match checked_user_len(input_len) {
        Ok(len) => len,
        Err(status) => return (status as u64, 0),
    };
    let output_len = match checked_user_len(output_len) {
        Ok(len) => len,
        Err(status) => return (status as u64, 0),
    };
    let input_ptr = input_ptr as *const u8;
    let output_ptr = output_ptr as *mut u8;
    if (input_len != 0 && (input_ptr.is_null() || !is_user_range(input_ptr, input_len)))
        || (output_len != 0 && (output_ptr.is_null() || !is_user_range(output_ptr, output_len)))
    {
        return (STATUS_BAD_REQUEST as u64, 0);
    }
    let input = if input_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(input_ptr, input_len) }
    };
    let output = if output_len == 0 {
        &mut []
    } else {
        unsafe { slice::from_raw_parts_mut(output_ptr, output_len) }
    };

    let app = LIBRARY_APP.get_or_init(|| RwLock::new(None));
    let (status, response) = match app.write() {
        Ok(mut app) => {
            let result = match op {
                OP_INIT => KeyManager::restore(input).map(|(manager, sealed)| {
                    *app = Some(manager);
                    sealed
                }),
                _ => app
                    .as_ref()
                    .ok_or_else(|| "key manager is not initialized".to_string())
                    .and_then(|manager| match op {
                        OP_PUBLIC_KEY => Ok(manager.public_key().to_vec()),
                        OP_ATTEST => manager.attest(),
                        OP_EXPORT_KEYS => manager.export(input),
                        _ => Err("unknown operation".into()),
                    }),
            };
            match result {
                Ok(payload) => (STATUS_OK, payload),
                Err(err) => (STATUS_BAD_REQUEST, err.into_bytes()),
            }
        }
        Err(_) => (STATUS_PANIC, b"key-manager app lock poisoned".to_vec()),
    };

    if response.len() > output.len() {
        return (STATUS_BUFFER_TOO_SMALL as u64, response.len() as u64);
    }

    output[..response.len()].copy_from_slice(&response);
    (status as u64, response.len() as u64)
}

fn checked_user_len(len: u64) -> Result<usize, u8> {
    let len = usize::try_from(len).map_err(|_| STATUS_BAD_REQUEST)?;
    if len > MAX_PAYLOAD_LEN {
        return Err(STATUS_BAD_REQUEST);
    }
    Ok(len)
}

fn is_user_range(ptr: *const u8, len: usize) -> bool {
    if len == 0 {
        return true;
    }

    let start = ptr as usize;
    let Some(end) = start.checked_add(len - 1) else {
        return false;
    };

    let base: usize;
    unsafe {
        std::arch::asm!(
            "lea IMAGE_BASE(%rip), {}",
            lateout(reg) base,
            options(att_syntax, nostack, preserves_flags, nomem, pure),
        );
    }

    let enclave_size = unsafe { ENCLAVE_SIZE };
    let Some(enclave_end) = base.checked_add(enclave_size.saturating_sub(1)) else {
        return false;
    };

    end < base || start > enclave_end
}

unsafe extern "C" {
    static ENCLAVE_SIZE: usize;
}
