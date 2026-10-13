use constant_time_utils::time;
use rand::{rngs::OsRng, RngCore};
use star_core::client::User;
use star_core::secure_channel::{client::SecureChannelClient, server::SecureChannelServer};
use star_core::serv::core::token;
use star_core::serv::enclave::Enclave;

mod rng;
use rng::PoisonedRng;

const PERIOD: u64 = 1;

fn main() {
    let mut rng: PoisonedRng = PoisonedRng::new();

    // arg of target 1
    let max_tokens_per_credential = 3u32;

    // target 1
    let enclave = Enclave::default(&mut rng);

    // User setup and credential issuance.
    let pk = enclave.public_key();
    timecop::unpoison(pk.as_slice());
    let sender = SecureChannelClient::new(pk).expect("failed to initialize sender");
    let mut user = User::new(max_tokens_per_credential as u64, sender);
    let (cipher, ck) = user.request_credential(&mut rng);

    // arg of target 2
    timecop::poison(cipher.as_slice());

    // target 2
    let credential_bytes: Vec<u8> = enclave.issue_cred(&cipher, &mut rng);

    timecop::unpoison(credential_bytes.as_slice());
    user.receive_credential(&credential_bytes, &ck);

    let envelope = user
        .request_auth(1, PERIOD, &mut rng)
        .expect("failed to build token request");

    let period = PERIOD.to_be_bytes();

    // arg of target 3
    timecop::poison(period.as_slice());
    timecop::poison(envelope.as_slice());

    // target 3
    let token = enclave
        .issue_token(&envelope, period)
        .expect("token issuance failed");
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

    let sk_bytes = [0u8; 32];
    timecop::poison(sk_bytes.as_slice());

    let sk = KemType::sk_from_bytes(&sk_bytes).unwrap();

    let plain = [0u8; 32];
    let bad_encapped_key = KemType::enc_from_bytes(&plain).expect("bad enc parse failed");

    let info = b"ct-test";

    let res = Suite::setup_receiver_base(&bad_encapped_key, &sk, info);
    assert!(matches!(res, Err(HpkeError::EncapError)));
}

fn fail2() {
    let mut rng = OsRng;
    let mut hpke_sk = [0u8; 32];
    rng.fill_bytes(&mut hpke_sk);

    timecop::poison(hpke_sk.as_slice());
    let receiver = SecureChannelServer::new(hpke_sk).unwrap();

    let receiver_pk = receiver.public_key();
    timecop::unpoison(receiver_pk.as_slice());

    let sender = SecureChannelClient::new(receiver_pk).expect("sender init failed");
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

    // this is target
    let decrypted_key = receiver.receive_hpke(enc, info, aad, ciphertext);
    assert!(matches!(decrypted_key, Err(HpkeError::OpenError)));
}
