use constant_time_utils::time;
use rand::{rngs::OsRng, RngCore};
use star_core::secure_channel::client::SecureChannelClient;
use star_core::serv::core::token;
use star_core::serv::enclave::Enclave;
use star_core::client::User;

const period :u64 = 1;

#[cfg(feature = "timecop")]
fn main() {
    // End-to-end demo: issue a credential, request a token over HPKE, and inspect the token.
    let mut rng = OsRng;

    // Bootstrap enclave secrets.
    let mut token_sk = [0u8; 32];
    let mut hpke_sk = [0u8; 32];
    rng.fill_bytes(&mut token_sk);
    rng.fill_bytes(&mut hpke_sk);

    timecop::poison(&hpke_sk);
    timecop::poison(&token_sk);

    let max_tokens_per_credential = 3u32;
    let enclave =
        Enclave::new(max_tokens_per_credential, token_sk, hpke_sk)
            .expect("failed to initialize enclave");


    // User setup and credential issuance.
    let sender = SecureChannelClient::new(enclave.public_key())
        .expect("failed to initialize sender");
    let mut user = User::new(max_tokens_per_credential as u64, sender);
    let (cipher, ck) = user.request_credential(&mut rng);
    let credential_bytes = enclave.issue_cred(&cipher, &mut rng);
    user.receive_credential(&credential_bytes, &ck);

    let mut issue_for_period = |counter| {
        let envelope = user
            .request_auth(counter, period, &mut rng)
            .expect("failed to build token request");
        enclave.issue_token(&envelope, period.to_be_bytes()).expect("token issuance failed")
    };

    let token_bytes3 = issue_for_period(1);
    let token_bytes1 = issue_for_period(0);

    timecop::unpoison(&hpke_sk);
    timecop::unpoison(&token_sk);

    println!("done");
}
