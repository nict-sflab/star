use super::*;

const LABEL: [u8; 16] = *b"enc-key         ";



#[cfg(target_env = "sgx")]
#[test]
fn seal_round_trip() {
    let plaintext = b"payload sealed by SGX";
    let blob = SealedBlob::seal(LABEL, plaintext).unwrap();
    assert_ne!(blob.ciphertext.as_slice(), plaintext);
    let restored = SealedBlob::decode(&blob.encode().unwrap()).unwrap();
    assert_eq!(restored.unseal(LABEL).unwrap(), plaintext);
}

#[cfg(target_env = "sgx")]
#[test]
fn invalid_label_or_ciphertext() {
    let mut blob = SealedBlob::seal(LABEL, b"payload").unwrap();
    assert!(matches!(
        blob.unseal(*b"prf-key         "),
        Err(SealError::Crypto)
    ));
    *blob.ciphertext.last_mut().unwrap() ^= 1;
    assert!(matches!(blob.unseal(LABEL), Err(SealError::Crypto)));
}

#[cfg(target_env = "sgx")]
#[test]
fn modified_metadata() {
    let mut blob = SealedBlob::seal(LABEL, b"payload").unwrap();
    blob.seal_data.miscselect =
        Miscselect::from_bits_truncate(blob.seal_data.miscselect.bits() ^ 1);
    assert!(matches!(
        blob.unseal(LABEL),
        Err(SealError::InvalidEnclaveAttributes)
    ));
}
