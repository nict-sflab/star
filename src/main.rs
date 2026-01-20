use constant_time_utils::time::current_period;
use rand::RngCore;
use star::enclave::core::{bytes::Short, cred::Credential, token::Token};
use star::enclave::enclave::Enclave;

fn main() {
    issue_credential_then_token();
}

fn issue_credential_then_token() {
    let mut rng = rand::thread_rng();

    let mut sk = [0u8; 32];
    rng.fill_bytes(&mut sk);

    let mut cipher_receiver = [0u8; 32];
    rng.fill_bytes(&mut cipher_receiver);

    // Allow up to 5 tokens per period in this demo.
    let enclave = Enclave::new(5, sk, cipher_receiver).expect("failed to create enclave");

    // Issue a credential inside the enclave.
    let credential_bytes = enclave.issue_cred_plain();
    let credential = Credential::from_bytes(&credential_bytes);

    // Build the plaintext token request: cmd + credential + count + period.
    let period: Short = current_period();
    let count: Short = 1u64.to_be_bytes();
    let mut token_request = Vec::with_capacity(1 + credential_bytes.len() + count.len() + period.len());
    token_request.push(0x01); // placeholder command byte
    token_request.extend_from_slice(&credential_bytes);
    token_request.extend_from_slice(&count);
    token_request.extend_from_slice(&period);

    let token_bytes = enclave.issue_token_plain(&token_request);
    let token = Token::from_bytes(&token_bytes);

    println!("issued credential uid     : {:02x?}", credential.uid);
    println!("issued token (period {:?}): {:02x?}", token.period, token.token);
}
