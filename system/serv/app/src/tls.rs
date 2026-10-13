use foreign_types::ForeignTypeRef;
use openssl::error::ErrorStack;
use openssl::ex_data::Index;
use openssl::ssl::{ClientHelloResponse, Ssl};
use pingora::tls::ssl::{SslAlert, SslRef};
use simple_filter_box::core::FilterBox;
use std::os::raw::{c_int, c_uchar, c_uint};
use std::sync::{Arc, OnceLock};
use std::{ptr, slice};

use crate::filter::{StarFilter, StarFilterErrorStatus};

static STAR_TLS_TOKEN_EX_INDEX: OnceLock<Index<Ssl, Vec<u8>>> = OnceLock::new();

pub const STAR_TLS_EXTENSION_TYPE: u16 = 65280;

unsafe extern "C" {
    fn SSL_client_hello_get0_ext(
        s: *mut openssl_sys::SSL,
        type_: c_uint,
        out: *mut *const c_uchar,
        outlen: *mut usize,
    ) -> c_int;
}

pub fn star_tls_extension_from_client_hello<'a>(ssl: &'a SslRef) -> Option<&'a [u8]> {
    let mut out = ptr::null();
    let mut out_len = 0usize;

    let found = unsafe {
        SSL_client_hello_get0_ext(
            ssl.as_ptr(),
            STAR_TLS_EXTENSION_TYPE as c_uint,
            &mut out,
            &mut out_len,
        )
    };

    if found != 1 {
        return None;
    }
    if out_len == 0 {
        return Some(&[]);
    }
    if out.is_null() {
        return None;
    }

    Some(unsafe { slice::from_raw_parts(out as *const u8, out_len) })
}

/// Creates the callback used by both the server and the rejection benchmarks.
pub fn star_tls_client_hello_callback<T>(
    filter: Arc<StarFilter<T>>,
) -> Result<
    impl Fn(&mut SslRef, &mut SslAlert) -> Result<ClientHelloResponse, ErrorStack>
        + Send
        + Sync
        + 'static,
    ErrorStack,
>
where
    T: FilterBox + Send + Sync + 'static,
{
    let token_index = if let Some(index) = STAR_TLS_TOKEN_EX_INDEX.get() {
        *index
    } else {
        let index = Ssl::new_ex_index::<Vec<u8>>()?;
        let _ = STAR_TLS_TOKEN_EX_INDEX.set(index);
        *STAR_TLS_TOKEN_EX_INDEX.get().expect("TLS token index set")
    };

    Ok(move |ssl, alert| {
        let extension = match star_tls_extension_from_client_hello(ssl) {
            Some(extension) if !extension.is_empty() => extension,
            _ => {
                *alert = SslAlert::DECODE_ERROR;
                return Err(ErrorStack::get());
            }
        };

        if let Some(existing) = ssl.ex_data(token_index) {
            if existing.as_slice() == extension {
                return Ok(ClientHelloResponse::SUCCESS);
            }
            *alert = SslAlert::ILLEGAL_PARAMETER;
            return Err(ErrorStack::get());
        }

        if let Err(error) = filter.filter_client_hello(extension) {
            *alert = match error {
                StarFilterErrorStatus::InvalidRoute
                | StarFilterErrorStatus::DuplicatedToken => SslAlert::ILLEGAL_PARAMETER,
                StarFilterErrorStatus::InvalidToken | StarFilterErrorStatus::Internal => {
                    SslAlert::DECODE_ERROR
                }
            };
            return Err(ErrorStack::get());
        }

        ssl.set_ex_data(token_index, extension.to_vec());
        Ok(ClientHelloResponse::SUCCESS)
    })
}
