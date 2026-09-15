use foreign_types::ForeignTypeRef;
use openssl::error::ErrorStack;
use openssl::ex_data::Index;
use openssl::ssl::{ClientHelloResponse, Ssl};
use pingora::listeners::tls::TlsSettings;
use pingora::tls::ssl::{ExtensionContext, SslAlert, SslRef};
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

fn star_tls_extension_from_client_hello<'a>(ssl: &'a SslRef) -> Option<&'a [u8]> {
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

pub fn configure_tls_token_extension<T>(
    tls_settings: &mut TlsSettings,
    filter: Arc<StarFilter<T>>,
) -> Result<(), ErrorStack>
where
    T: FilterBox + Send + Sync + 'static,
{
    tls_settings.set_groups_list("X25519")?;
    let token_index = if let Some(index) = STAR_TLS_TOKEN_EX_INDEX.get() {
        *index
    } else {
        let index = Ssl::new_ex_index::<Vec<u8>>()?;
        let _ = STAR_TLS_TOKEN_EX_INDEX.set(index);
        *STAR_TLS_TOKEN_EX_INDEX.get().expect("TLS token index set")
    };

    tls_settings.set_client_hello_callback(|ssl, alert| {
        if matches!(
            star_tls_extension_from_client_hello(ssl),
            Some(extension) if !extension.is_empty()
        ) {
            return Ok(ClientHelloResponse::SUCCESS);
        }

        *alert = SslAlert::DECODE_ERROR;
        Err(ErrorStack::get())
    });

    tls_settings.add_custom_ext(
        STAR_TLS_EXTENSION_TYPE,
        ExtensionContext::CLIENT_HELLO,
        |_ssl, _context, _cert| Ok(None::<Vec<u8>>),
        move |ssl, _context, extension, _cert| {
            if extension.is_empty() {
                return Err(SslAlert::DECODE_ERROR);
            }

            if let Some(existing) = ssl.ex_data(token_index) {
                return if existing.as_slice() == extension {
                    Ok(())
                } else {
                    Err(SslAlert::ILLEGAL_PARAMETER)
                };
            }

            if let Err(error) = filter.filter_client_hello(extension) {
                return Err(match error {
                    StarFilterErrorStatus::InvalidRoute
                    | StarFilterErrorStatus::DuplicatedToken => SslAlert::ILLEGAL_PARAMETER,
                    StarFilterErrorStatus::InvalidToken | StarFilterErrorStatus::Internal => {
                        SslAlert::DECODE_ERROR
                    }
                });
            }

            ssl.set_ex_data(token_index, extension.to_vec());
            Ok(())
        },
    )?;

    Ok(())
}
