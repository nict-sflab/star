use super::*;
use star_core::secure_channel::client::SecureChannelClient;

#[test]
fn key_transfer_does_not_initialize_issuer() {
    let mut enclave = Enclave::default();
    let previous_key = enclave.transfer_public_key();
    enclave.prepare_key_transfer().unwrap();
    assert_ne!(enclave.transfer_public_key(), previous_key);
    assert!(enclave.public_key().is_err());
    assert!(enclave.issue_cred(&[]).is_err());
    assert!(enclave.issue_token(&[], [0; 8]).is_err());
}

#[test]
fn failed_install_leaves_enclave_uninitialized() {
    let mut enclave = Enclave::default();
    let transfer_key = enclave.transfer_public_key();
    assert!(enclave.install_keys(&[0; 64], &[0; 64]).is_err());
    assert!(enclave.public_key().is_err());
    assert_eq!(enclave.transfer_public_key(), transfer_key);
}

#[test]
fn initialized_enclave_rejects_key_replacement() {
    let mut enclave = Enclave {
        issuer: Some(CredentialIssuer::new(MAX_COUNT, [7; 32], [8; 32]).unwrap()),
        ..Enclave::default()
    };
    let public_key = enclave.public_key().unwrap();
    let transfer_key = enclave.transfer_public_key();
    assert!(enclave.prepare_key_transfer().is_err());
    assert!(enclave.install_keys(&[], &[]).is_err());
    assert_eq!(enclave.public_key().unwrap(), public_key);
    assert_eq!(enclave.transfer_public_key(), transfer_key);
}

#[test]
fn invalid_issuance_requests_are_rejected() {
    let enclave = Enclave {
        issuer: Some(CredentialIssuer::new(MAX_COUNT, [7; 32], [8; 32]).unwrap()),
        ..Enclave::default()
    };
    assert!(enclave.issue_cred(&[]).is_err());
    assert!(enclave.issue_token(&[], [0; 8]).is_err());
}

#[test]
fn issues_credentials_and_tokens_with_the_measured_limit() {
    let enclave = Enclave {
        issuer: Some(
            CredentialIssuer::new(u32::from_be_bytes(MEASURED_MAX_COUNT_BE), [7; 32], [8; 32])
                .unwrap(),
        ),
        ..Enclave::default()
    };
    let client = SecureChannelClient::new(enclave.public_key().unwrap()).unwrap();
    let (request, shared_key) = client.send_shared_key(&mut OsRng).unwrap();
    let response = enclave.issue_cred(&request).unwrap();
    let credential = client
        .decrypt_with_shared_key(&response, &shared_key)
        .unwrap();
    assert_eq!(credential.len(), 64);

    let period = 42u64.to_be_bytes();
    for count in [0, MAX_COUNT as u64] {
        let mut request = credential.clone();
        request.extend_from_slice(&count.to_be_bytes());
        request.extend_from_slice(&period);
        let encrypted = client.send_hpke(&request, &mut OsRng).unwrap();
        let token = enclave.issue_token(&encrypted, period).unwrap();
        if count < MAX_COUNT as u64 {
            assert_eq!(token.len(), 32);
        } else {
            assert_eq!(token, [0]);
        }
    }
}

#[test]
fn measured_max_count_matches_policy() {
    assert_eq!(u32::from_be_bytes(MEASURED_MAX_COUNT_BE), MAX_COUNT);
}
