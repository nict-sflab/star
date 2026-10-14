pub fn fake_quote() -> Vec<u8> {
    // A v5 SGX report with matching measurements and a public key, but no signature.
    let mut quote = vec![0u8; 54 + 384];
    quote[..2].copy_from_slice(&5u16.to_le_bytes());
    quote[2..4].copy_from_slice(&2u16.to_le_bytes());
    quote[48..50].copy_from_slice(&1u16.to_le_bytes());
    quote[50..54].copy_from_slice(&384u32.to_le_bytes());
    quote[54 + 64..54 + 96].fill(1);
    quote[54 + 128..54 + 160].fill(2);
    quote[54 + 320..54 + 352].fill(3);
    quote
}
