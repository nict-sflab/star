use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
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

    pub(crate) fn token_request(&self) -> Result<Vec<u8>, BoxError> {
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

        let mut rng = OsRng;
        session
            .user
            .request_auth(counter, period, &mut rng)
            .map_err(|error| format!("failed to build STAR token request: {error:?}").into())
    }
}

fn current_period() -> u64 {
    u64::from_be_bytes(constant_time_utils::time::current_period())
}
