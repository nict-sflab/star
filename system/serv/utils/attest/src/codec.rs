use alloc::{format, string::ToString, vec::Vec};

use crate::Result;

pub(crate) fn append_blob(payload: &mut Vec<u8>, blob: &[u8]) {
    payload.extend_from_slice(&(blob.len() as u32).to_be_bytes());
    payload.extend_from_slice(blob);
}

pub(crate) fn take_blob<'a>(payload: &mut &'a [u8]) -> Result<&'a [u8]> {
    let len: [u8; 4] = payload
        .get(..4)
        .ok_or_else(|| "missing blob length".to_string())?
        .try_into()
        .map_err(|_| "bad blob length".to_string())?;

    let len = u32::from_be_bytes(len) as usize;
    let blob = payload
        .get(4..4 + len)
        .ok_or_else(|| "truncated blob".to_string())?;
    *payload = &payload[4 + len..];
    Ok(blob)
}

pub(crate) fn read_array<const N: usize>(
    bytes: &[u8],
    offset: usize,
    name: &str,
) -> Result<[u8; N]> {
    bytes
        .get(offset..offset + N)
        .ok_or_else(|| format!("quote is too short for {name}"))?
        .try_into()
        .map_err(|_| format!("quote is too short for {name}"))
}

pub(crate) fn read_u16_le(bytes: &[u8], offset: usize) -> Result<u16> {
    let bytes: [u8; 2] = read_array(bytes, offset, "u16")?;
    Ok(u16::from_le_bytes(bytes))
}

pub(crate) fn read_u32_le(bytes: &[u8], offset: usize) -> Result<u32> {
    let bytes: [u8; 4] = read_array(bytes, offset, "u32")?;
    Ok(u32::from_le_bytes(bytes))
}

pub(crate) fn read_u64_le(bytes: &[u8], offset: usize) -> Result<u64> {
    let bytes: [u8; 8] = read_array(bytes, offset, "u64")?;
    Ok(u64::from_le_bytes(bytes))
}
