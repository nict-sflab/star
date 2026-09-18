use std::sync::{Arc, Mutex};

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use rand::rngs::OsRng;
use star_core::{client::User, secure_channel::client::SecureChannelClient};

use crate::BoxError;

#[derive(Clone)]
pub(crate) struct StarClient {
    session: Arc<Mutex<CredentialSession>>,
}

struct CredentialSession {
    user: User,
    period: u64,
    next_counter: u64,
}

impl StarClient {
    pub(crate) fn register(api_base: &str, max_requests_per_period: u64) -> Result<Self, BoxError> {
        let api_base = api_base.trim_end_matches('/');
        let public_key = ureq::get(&format!("{api_base}/star/public_key"))
            .call()?
            .into_string()?;
        let public_key = URL_SAFE.decode(public_key.trim())?;
        let public_key: [u8; 32] = public_key
            .try_into()
            .map_err(|_| "STAR public key must be 32 bytes")?;

        let channel = SecureChannelClient::new(public_key)
            .map_err(|error| format!("failed to initialize secure channel: {error:?}"))?;
        let mut user = User::new(max_requests_per_period, channel);
        let mut rng = OsRng;
        let (request, shared_key) = user.request_credential(&mut rng);
        let request = URL_SAFE.encode(request);

        let credential = ureq::post(&format!("{api_base}/star/register"))
            .send_bytes(request.as_bytes())?
            .into_string()?;
        let credential = URL_SAFE.decode(credential.trim())?;
        user.receive_credential(&credential, &shared_key)
            .ok_or("failed to receive STAR credential")?;

        Ok(Self {
            session: Arc::new(Mutex::new(CredentialSession {
                user,
                period: current_period(),
                next_counter: 0,
            })),
        })
    }

    pub(crate) fn token_request(&self, shard_count: u32) -> Result<(Vec<u8>, u32), BoxError> {
        if shard_count == 0 {
            return Err("STAR shard count must be positive".into());
        }
        let mut session = self.session.lock().unwrap();
        let period = current_period();

        if session.period != period {
            session.period = period;
            session.next_counter = 0;
        }

        let counter = session.next_counter;
        if counter >= session.user.max {
            return Err(format!(
                "exceeded {} STAR token requests for period {period}",
                session.user.max
            )
            .into());
        }
        session.next_counter += 1;

        let route = session.user.route(counter, period, shard_count);
        let mut rng = OsRng;
        session
            .user
            .request_auth(counter, period, &mut rng)
            .map(|request| (request, route))
            .map_err(|error| format!("failed to build STAR token request: {error:?}").into())
    }
}

fn current_period() -> u64 {
    u64::from_be_bytes(constant_time_utils::time::current_period())
}

#[cfg(test)]
mod tests {
    use super::*;
    use star_core::serv::{core::token::Token, enclave::Enclave};

    #[test]
    fn encrypted_requests_and_client_routes_agree() {
        // Exercise the actual client counter/period handling and server token
        // derivation without SGX, attestation, or an external key manager.
        let enclave = Enclave::new(64, [17; 32], [29; 32]).unwrap();
        for shard_count in [1, 2, 4, 7] {
            let mut user = User::new(64, SecureChannelClient::new(enclave.public_key()).unwrap());
            user.receive_credential_plain(&enclave.issue_cred_plain(&mut OsRng));
            let client = StarClient {
                session: Arc::new(Mutex::new(CredentialSession {
                    user,
                    period: current_period(),
                    next_counter: 0,
                })),
            };
            assert!(client.token_request(0).is_err());
            for _ in 0..64 {
                let (request, route) = client.token_request(shard_count).unwrap();
                let period = client.session.lock().unwrap().period;
                let token = enclave.issue_token(&request, period.to_be_bytes()).unwrap();
                let token = Token::new(token.try_into().unwrap());
                assert_eq!(route, token.route(shard_count));
            }
        }
    }
}
