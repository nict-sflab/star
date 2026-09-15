use base64::{
    Engine as _,
    engine::general_purpose::{URL_SAFE, URL_SAFE_NO_PAD},
};
use star_attestation::{Quote, Result};

/// Reads a raw quote, URL-safe base64, or a service startup log.
pub fn decode_quote_input(input: &[u8]) -> Result<Vec<u8>> {
    if Quote::parse(input).is_ok() {
        return Ok(input.to_vec());
    }

    let text = std::str::from_utf8(input)
        .map_err(|err| format!("stdin is neither an SGX quote nor UTF-8 base64: {err}"))?;
    let encoded = text
        .lines()
        .rev()
        .find_map(|line| line.trim().strip_prefix("STAR_ATTESTATION_QUOTE_BASE64="))
        .unwrap_or(text)
        .trim();
    let quote = URL_SAFE
        .decode(encoded)
        .or_else(|_| URL_SAFE_NO_PAD.decode(encoded))
        .map_err(|err| format!("decode quote base64: {err}"))?;
    Quote::parse(&quote)?;
    Ok(quote)
}
