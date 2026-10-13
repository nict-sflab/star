use rand::{rngs::OsRng, RngCore};
use star_core::client::User;
use star_core::secure_channel::{
    client::SecureChannelClient,
    server::SecureChannelServer,
};
use star_core::serv::enclave::Enclave;

mod rng;
use rng::PoisonedRng;

const PERIOD: u64 = 1;

fn main() {
    // PoisonedRng poisons the random values it generates.
    // Do not poison the RNG object itself.
    let mut rng = PoisonedRng::new();
    let max_tokens_per_credential = 3u32;

    // Target 1: Enclave::default(&mut rng)
    // The RNG output is poisoned by PoisonedRng.
    let enclave = Enclave::default(&mut rng);

    // User setup and credential issuance.
    let pk = enclave.public_key();
    timecop::unpoison(pk.as_slice());

    let sender = SecureChannelClient::new(pk)
        .expect("failed to initialize sender");
    let mut user = User::new(max_tokens_per_credential as u64, sender);
    let (cipher, ck) = user.request_credential(&mut rng);

    // Target 2: enclave.issue_cred(&cipher, &mut rng)
    // Poison the cipher bytes, not the enclave or RNG objects.
    timecop::poison(cipher.as_slice());

    let credential_bytes: Vec<u8> = enclave.issue_cred(&cipher, &mut rng);

    timecop::unpoison(cipher.as_slice());
    timecop::unpoison(credential_bytes.as_slice());
    user.receive_credential(&credential_bytes, &ck);

    let envelope = user
        .request_auth(1, PERIOD, &mut rng)
        .expect("failed to build token request");
    let period = PERIOD.to_be_bytes();

    // Target 3: enclave.issue_token(&envelope, period)
    // Poison both explicit byte-array inputs, not the enclave object.
    timecop::poison(period.as_slice());
    timecop::poison(envelope.as_slice());

    let token = enclave
        .issue_token(&envelope, period)
        .expect("token issuance failed");

    timecop::unpoison(period.as_slice());
    timecop::unpoison(envelope.as_slice());
    timecop::unpoison(token.as_slice());

    fail1();
    fail2();

    println!("done");
}

use hpke_ng::{
    ChaCha20Poly1305, DhKemX25519HkdfSha256, HkdfSha384, Hpke, HpkeError, Kem,
};

fn fail1() {
    type KemType = DhKemX25519HkdfSha256;
    type Suite = Hpke<KemType, HkdfSha384, ChaCha20Poly1305>;

    // Poison the source bytes before parsing them into a secret key.
    let sk_bytes = [0u8; 32];
    timecop::poison(sk_bytes.as_slice());
    let sk = KemType::sk_from_bytes(&sk_bytes).unwrap();

    // Poison the source bytes before parsing an encapsulated key.
    let plain = [0u8; 32];
    timecop::poison(plain.as_slice());
    let bad_encapped_key =
        KemType::enc_from_bytes(&plain).expect("bad enc parse failed");

    // Target: Suite::setup_receiver_base(&bad_encapped_key, &sk, info)
    // All three inputs are poisoned through their bytes/source values.
    let info = b"ct-test";
    timecop::poison(info.as_slice());

    let res = Suite::setup_receiver_base(&bad_encapped_key, &sk, info);

    timecop::unpoison(info.as_slice());
    timecop::unpoison(plain.as_slice());
    timecop::unpoison(sk_bytes.as_slice());

    assert!(matches!(res, Err(HpkeError::EncapError)));
}

fn fail2() {
    let mut rng = OsRng;
    let mut hpke_sk = [0u8; 32];
    rng.fill_bytes(&mut hpke_sk);

    // Poison the source key bytes, not the receiver object.
    timecop::poison(hpke_sk.as_slice());
    let receiver = SecureChannelServer::new(hpke_sk).unwrap();

    let receiver_pk = receiver.public_key();
    timecop::unpoison(receiver_pk.as_slice());

    let sender = SecureChannelClient::new(receiver_pk)
        .expect("sender init failed");
    let (envelope, _shared_key) = sender
        .send_shared_key(&mut rng)
        .expect("send shared key failed");

    let mut envelope = envelope.clone();
    const AAD_LEN: usize = 32;
    const INFO_LEN: usize = 32;
    const ENC_LEN: usize = 32;
    let min_len = AAD_LEN + INFO_LEN + ENC_LEN;
    assert!(envelope.len() >= min_len, "HPKE envelope too short");

    *envelope.last_mut().unwrap() ^= 1;

    let aad = &envelope[..AAD_LEN];
    let info = &envelope[AAD_LEN..AAD_LEN + INFO_LEN];
    let enc = &envelope[AAD_LEN + INFO_LEN..AAD_LEN + INFO_LEN + ENC_LEN];
    let ciphertext = &envelope[AAD_LEN + INFO_LEN + ENC_LEN..];

    // Target: receiver.receive_hpke(enc, info, aad, ciphertext)
    // Poison all four byte-slice arguments. The receiver key comes
    // from the already-poisoned hpke_sk bytes.
    timecop::poison(enc);
    timecop::poison(info);
    timecop::poison(aad);
    timecop::poison(ciphertext);

    let decrypted_key = receiver.receive_hpke(enc, info, aad, ciphertext);

    timecop::unpoison(enc);
    timecop::unpoison(info);
    timecop::unpoison(aad);
    timecop::unpoison(ciphertext);

    assert!(matches!(decrypted_key, Err(HpkeError::OpenError)));
}