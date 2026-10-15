use rand::{rand_core::UnwrapErr, rngs::SysRng};
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;
use star_core::serv::enclave::{Enclave, DEFAULT_MAX_TICKET};

mod rng;
use rng::PoisonedRng;

const PERIOD: u64 = 1;

fn main() {
    // PoisonedRng poisons the random values it generates.
    // Do not poison the RNG object itself.
    let mut enclave_rng = PoisonedRng::new();
    let mut client_rng = UnwrapErr(SysRng);
    let max_tokens_per_credential = DEFAULT_MAX_TICKET;

    // Target 1: Enclave::default(&mut enclave_rng)
    // The RNG output is poisoned by PoisonedRng.
    let enclave = Enclave::default(&mut enclave_rng);

    // User setup and credential issuance.
    let pk = enclave.public_key();
    timecop::unpoison(pk.as_slice());

    let sender = SecureChannelClient::new(pk).expect("failed to initialize sender");
    let mut user = User::new(max_tokens_per_credential as u64, sender);
    let (cipher, ck) = user.request_credential(&mut client_rng);

    // Target 2: enclave.issue_cred(&cipher, &mut enclave_rng)
    // Poison the cipher bytes, not the enclave or RNG objects.
    timecop::poison(cipher.as_slice());

    let credential_bytes: Vec<u8> = enclave.issue_cred(&cipher, &mut enclave_rng);

    timecop::unpoison(cipher.as_slice());
    timecop::unpoison(credential_bytes.as_slice());
    user.receive_credential(&credential_bytes, &ck);

    let envelope = user
        .request_auth(1, PERIOD, &mut client_rng)
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

    fail1(&enclave, &envelope, period);
    fail2(&enclave, &envelope, period);

    println!("done");
}

fn fail1(enclave: &Enclave, valid_envelope: &[u8], period: [u8; 8]) {
    // Replace the HPKE encapsulated key with an invalid X25519 point.
    let mut envelope = valid_envelope.to_vec();
    envelope[64..96].fill(0);
    timecop::poison(envelope.as_slice());

    let response = enclave.issue_token(&envelope, period);

    timecop::unpoison(envelope.as_slice());
    assert_eq!(response, Err(1));
}

fn fail2(enclave: &Enclave, valid_envelope: &[u8], period: [u8; 8]) {
    // Corrupt the authentication tag of an otherwise valid HPKE envelope.
    let mut envelope = valid_envelope.to_vec();
    *envelope.last_mut().unwrap() ^= 1;
    timecop::poison(envelope.as_slice());

    let response = enclave.issue_token(&envelope, period);

    timecop::unpoison(envelope.as_slice());
    assert_eq!(response, Err(1));
}
