use super::*;

const LABEL: [u8; 16] = *b"enc-key         ";

// Fixed bincode representation of the existing sealed-blob format.
fn encoded_blob() -> Vec<u8> {
    let mut bytes = vec![3; 16];
    bytes.extend_from_slice(&7u16.to_le_bytes());
    bytes.extend_from_slice(&[9; 16]);
    bytes.extend_from_slice(&[0; 20]); // attributes (16) and miscselect (4)
    bytes.extend_from_slice(&[0x24; 12]);
    bytes.extend_from_slice(&3u64.to_le_bytes());
    bytes.extend_from_slice(&[1, 2, 3]);
    bytes
}

#[test]
fn sealed_blob_preserves_existing_encoding() {
    let encoded = encoded_blob();
    let blob = SealedBlob::decode(&encoded).unwrap();
    assert_eq!(blob.seal_data.isvsvn, 7);
    assert_eq!(blob.nonce, [0x24; 12]);
    assert_eq!(blob.ciphertext, [1, 2, 3]);
    assert_eq!(blob.encode().unwrap(), encoded);
}

#[test]
fn sealed_blob_rejects_truncated_encoding() {
    let encoded = encoded_blob();
    for len in 0..encoded.len() {
        assert!(SealedBlob::decode(&encoded[..len]).is_err());
    }
}

#[cfg(not(target_env = "sgx"))]
#[test]
fn sealing_and_unsealing_require_sgx() {
    assert!(matches!(
        SealedBlob::seal(LABEL, b"payload"),
        Err(SealError::UnsupportedTarget)
    ));
    let blob = SealedBlob::decode(&encoded_blob()).unwrap();
    assert!(matches!(
        blob.unseal(LABEL),
        Err(SealError::UnsupportedTarget)
    ));
}

#[cfg(target_env = "sgx")]
#[test]
fn sealed_blob_round_trips_inside_sgx() {
    let plaintext = b"payload sealed by SGX";
    let blob = SealedBlob::seal(LABEL, plaintext).unwrap();
    assert_ne!(blob.ciphertext.as_slice(), plaintext);
    let restored = SealedBlob::decode(&blob.encode().unwrap()).unwrap();
    assert_eq!(restored.unseal(LABEL).unwrap(), plaintext);
}

#[cfg(target_env = "sgx")]
#[test]
fn sealed_blob_rejects_wrong_label_and_modified_ciphertext() {
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
fn sealed_blob_rejects_changed_enclave_metadata() {
    let mut blob = SealedBlob::seal(LABEL, b"payload").unwrap();
    blob.seal_data.miscselect =
        Miscselect::from_bits_truncate(blob.seal_data.miscselect.bits() ^ 1);
    assert!(matches!(
        blob.unseal(LABEL),
        Err(SealError::InvalidEnclaveAttributes)
    ));
}
