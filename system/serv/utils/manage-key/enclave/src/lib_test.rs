use super::*;

#[test]
fn authenticated_key_transfer() {
    let keys = KeyBundle::random(&mut OsRng);
    let receiver = KeyReceiver::random();
    let envelope = keys.export(receiver.public_key(), &mut OsRng).unwrap();
    let received = receiver.receive(&envelope, keys.public_key()).unwrap();
    assert_eq!(received.enc_key, keys.enc_key);
    assert_eq!(received.hpke_receiver_key, keys.hpke_receiver_key);
}

#[test]
fn rejects_invalid_key_transfers() {
    let keys = KeyBundle::random(&mut OsRng);
    let receiver = KeyReceiver::random();
    let other = KeyReceiver::random();
    let mut envelope = keys.export(receiver.public_key(), &mut OsRng).unwrap();
    assert!(receiver.receive(&envelope, other.public_key()).is_err());
    assert!(other.receive(&envelope, keys.public_key()).is_err());
    assert!(receiver
        .receive(&envelope[..95], keys.public_key())
        .is_err());
    *envelope.last_mut().unwrap() ^= 1;
    assert!(receiver.receive(&envelope, keys.public_key()).is_err());
}

#[cfg(not(target_env = "sgx"))]
#[test]
fn unsealed_keys() {
    assert!(KeyManager::restore(&[0; 64]).is_err());
    assert!(KeyManager::restore(&[]).is_err());
}

#[test]
fn invalid_evidence() {
    let manager = KeyManager {
        keys: KeyBundle::random(&mut OsRng),
        recipient_policy: QuotePolicy {
            expected_mrenclave: Some([1; 32].into()),
            expected_mrsigner: None,
            allow_debug: false,
            allow_advisory: false,
        },
    };
    assert!(manager.export(&[]).is_err());
    let receiver = KeyReceiver::random();
    let envelope = manager
        .keys
        .export(receiver.public_key(), &mut OsRng)
        .unwrap();
    assert!(receiver.receive_attested(&[], &envelope).is_err());
    let evidence = Evidence {
        quote: &[],
        collateral: &[],
    }
    .encode();
    assert!(receiver.receive_attested(&evidence, &envelope).is_err());
}
