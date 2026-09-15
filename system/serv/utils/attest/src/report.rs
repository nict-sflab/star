use alloc::format;

use crate::{
    Result,
    codec::{read_u16_le, read_u32_le},
};

const QUOTE_HEADER_LEN: usize = 48;
const SGX_REPORT_BODY_LEN: usize = 384;
const QUOTE_V5: u16 = 5;
const QUOTE_V5_BODY_TYPE_OFFSET: usize = 48;
const QUOTE_V5_BODY_SIZE_OFFSET: usize = 50;
const QUOTE_V5_BODY_OFFSET: usize = 54;
const SGX_QL_ALG_ECDSA_P256: u16 = 2;
const TEE_TYPE_SGX: u32 = 0;
const QUOTE_BODY_TYPE_SGX: u16 = 1;

pub(crate) fn sgx_report_body_offset(quote: &[u8]) -> Result<usize> {
    if quote.len() < QUOTE_HEADER_LEN {
        return Err("quote is too short".into());
    }

    let version = read_u16_le(quote, 0)?;
    let attestation_key_type = read_u16_le(quote, 2)?;
    if attestation_key_type != SGX_QL_ALG_ECDSA_P256 {
        return Err("quote attestation key is not ECDSA/P-256".into());
    }

    let report_body_offset = match version {
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
        _ => return Err(format!("unsupported quote version: {version}")),
    };

    if quote.len() < report_body_offset + SGX_REPORT_BODY_LEN {
        return Err("quote is too short".into());
    }

    Ok(report_body_offset)
}
