use super::*;
use hpke::{
    aead::ChaCha20Poly1305, kdf::HkdfSha256, kem::X25519HkdfSha256, Deserializable, OpModeR,
    Serializable,
};


#[test]
fn authenticated_export() {
    type Kem = X25519HkdfSha256;
    let keys = KeyBundle::random(&mut OsRng);
    let (secret, public) = <Kem as hpke::Kem>::gen_keypair(&mut OsRng);
    let envelope = keys.export(public.to_bytes().into(), &mut OsRng).unwrap();
    let sender = <Kem as hpke::Kem>::PublicKey::from_bytes(&keys.public_key()).unwrap();
    let enc = <Kem as hpke::Kem>::EncappedKey::from_bytes(&envelope[64..96]).unwrap();
    let mut receiver = hpke::setup_receiver::<ChaCha20Poly1305, HkdfSha256, Kem>(
        &OpModeR::Auth(sender),
        &secret,
        &enc,
        &envelope[32..64],
    )
    .unwrap();
    assert_eq!(
        receiver.open(&envelope[96..], &envelope[..32]).unwrap(),
        keys.encode()
    );
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
}
