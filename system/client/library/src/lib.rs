use wasm_bindgen::prelude::*;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand::rngs::OsRng;
use star_core::{client::User, secure_channel::client::SecureChannelClient};
use std::sync::{Mutex, OnceLock};

const MAX_COUNT: u64 = 10;

static USER: OnceLock<Mutex<User>> = OnceLock::new();

fn with_user<F, R>(f: F) -> R
where
    F: FnOnce(&mut User) -> R,
{
    let user_lock = USER.get().expect("client not initialized");
    let mut user = user_lock.lock().expect("client mutex poisoned");
    f(&mut user)
}

#[wasm_bindgen]
pub fn init_user(public_key: &str) {
    let public_key_decoded = URL_SAFE.decode(public_key).unwrap();
    let public_key_bytes: [u8; 32] = public_key_decoded
        .as_slice()
        .try_into()
        .expect("public key must be 32 bytes");
    let c = SecureChannelClient::new(public_key_bytes).unwrap();
    let u = User::new(MAX_COUNT, c);

    if USER.set(Mutex::new(u)).is_err() {
        panic!("client already initialized");
    }
}

#[wasm_bindgen]
pub fn request_register() -> String {
    let mut rng = OsRng;
    let (request, shared_key) = with_user(|user| user.request_credential(&mut rng));

    let request_encoded = URL_SAFE.encode(request);
    let shared_key_encoded = URL_SAFE.encode(shared_key);

    request_encoded + ":" + &shared_key_encoded
}

#[wasm_bindgen]
pub fn receive_credential(cipher: &str, shared_key: &str) -> String {
    let shared_key_decoded = URL_SAFE.decode(shared_key).unwrap();
    let shared_key_bytes: [u8; 32] = shared_key_decoded
        .as_slice()
        .try_into()
        .expect("shared key must be 32 bytes");

    let cipher_decoded = URL_SAFE.decode(cipher).unwrap();
    let credential = with_user(|user| user.receive_credential(&cipher_decoded, &shared_key_bytes))
        .expect("error");

    URL_SAFE.encode(credential.to_bytes())
}

#[wasm_bindgen]
pub fn request_authenticate(counter: u64, cred: &str) -> String {
    let cred_decoded = URL_SAFE.decode(cred).unwrap();
    if cred_decoded.len() != 64 {
        panic!("credential must be 64 bytes");
    }
    if counter >= MAX_COUNT {
        panic!("exceeded max token requests for this credential");
    }

    let mut rng = OsRng;
    let credential = with_user(|user| {
        user.receive_credential_plain(&cred_decoded);
        user.request_auth(counter, &mut rng).unwrap()
    });

    URL_SAFE.encode(credential)
}
