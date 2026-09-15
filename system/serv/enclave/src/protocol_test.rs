use super::*;

#[test]
fn init_only_prepares_key_transfer() {
    let enclave = RwLock::new(Enclave::default());
    assert!(dispatch(&enclave, INIT, &[7; 64]).is_err());
    let previous_key = dispatch(&enclave, TRANSFER_PUBLIC_KEY, &[]).unwrap();
    assert!(dispatch(&enclave, INIT, &[]).unwrap().is_empty());
    let transfer_key = dispatch(&enclave, TRANSFER_PUBLIC_KEY, &[]).unwrap();
    assert_eq!(transfer_key.len(), 32);
    assert_ne!(transfer_key, previous_key);
    assert!(dispatch(&enclave, PUBLIC_KEY, &[]).is_err());
    assert!(dispatch(&enclave, ISSUE_CRED, &[]).is_err());
    assert!(dispatch(&enclave, ISSUE_TOKEN, &[0; 8]).is_err());
}

#[test]
fn rejects_malformed_requests() {
    let enclave = RwLock::new(Enclave::default());
    assert!(dispatch(&enclave, ATTEST, &[0; 64]).is_err());
    assert!(dispatch(&enclave, ISSUE_TOKEN, &[0; 7]).is_err());
    for op in [8, 256, u64::MAX] {
        assert_eq!(
            dispatch(&enclave, op, &[]).unwrap_err(),
            "unknown operation"
        );
    }
    for payload in [&[][..], &[0, 0, 0, 1], &[0, 0, 0, 0], &[255; 4]] {
        assert!(dispatch(&enclave, INSTALL_KEYS, payload).is_err());
    }
    assert_eq!(
        dispatch(&enclave, INSTALL_KEYS, &[0; 9]).unwrap_err(),
        "trailing install request bytes"
    );
    assert_eq!(
        dispatch(&enclave, INSTALL_KEYS, &[0; 8]).unwrap_err(),
        "missing blob length"
    );
    assert_eq!(dispatch(&enclave, ONE, &[]).unwrap(), vec![1; 32]);
}
