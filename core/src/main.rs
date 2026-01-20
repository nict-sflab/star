use rand::RngCore;
use star_core::serv::core::token::Token;
use star_core::serv::enclave::Enclave;
use star_core::client::User;

fn main() {
    // End-to-end demo: issue a credential, request a token over HPKE, and inspect the token.
    let mut rng = rand::thread_rng();

    // Bootstrap enclave secrets.
    let mut token_sk = [0u8; 32];
    let mut hpke_sk = [0u8; 32];
    rng.fill_bytes(&mut token_sk);
    rng.fill_bytes(&mut hpke_sk);

    let max_tokens_per_credential = 3u32;
    let enclave =
        Enclave::new(max_tokens_per_credential, token_sk, hpke_sk)
            .expect("failed to initialize enclave");

    // User setup and credential issuance.
    let mut user = User::new(max_tokens_per_credential as u64);
    let credential_bytes = enclave.issue_cred_plain();
    user.receive_credential(&credential_bytes);
    println!(
        "User received credential ({} bytes): {}",
        credential_bytes.len(),
        hex(&credential_bytes)
    );

    // User asks for a token using an HPKE-wrapped request.
    let envelope1 = user
        .request_auth(0, &enclave.public_key())
        .expect("failed to build token request");
    let token_bytes1 = enclave
        .issue_token(&envelope1)
        .expect("token issuance failed");


    let envelope2 = user
        .request_auth(0, &enclave.public_key())
        .expect("failed to build token request");
    let token_bytes2 = enclave
        .issue_token(&envelope2)
        .expect("token issuance failed");

    let envelope3 = user
        .request_auth(1, &enclave.public_key())
        .expect("failed to build token request");
    let token_bytes3 = enclave
        .issue_token(&envelope3)
        .expect("token issuance failed");


    println!(
        "Issued token for period ({} bytes): {}",
        token_bytes1.len(),
        hex(&token_bytes1)
    );


    println!(
        "Issued token for period ({} bytes): {}",
        token_bytes2.len(),
        hex(&token_bytes2)
    );
    
    println!(
        "Issued token for period ({} bytes): {}",
        token_bytes3.len(),
        hex(&token_bytes3)
    );

    assert!(token_bytes1 == token_bytes2);
    assert!(token_bytes1 != token_bytes3);

}

/// Hex formatter to avoid pulling an extra dependency for the example.
fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{:02x}", byte));
    }
    out
}
