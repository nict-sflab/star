use std::sync::{Arc, Mutex};

use simple_filter_box::core::{Entry, FilterBox};
use star_core::serv::core::token::Token;

use crate::enclave::EnclaveRuntime;

pub struct StarFilter<T: FilterBox + Send + Sync> {
    filter: Mutex<T>,
    prefixes: Vec<u32>,
    prefix_max: u32,
    enclave: Arc<EnclaveRuntime>,
}

#[derive(Debug)]
pub enum StarFilterErrorStatus {
    InvalidToken,
    InvalidRoute,
    DuplicatedToken,
    Internal,
}

impl<T: FilterBox + Send + Sync> StarFilter<T> {
    pub fn new(
        filter: T,
        prefixes: Vec<u32>,
        prefix_max: u32,
        enclave: Arc<EnclaveRuntime>,
    ) -> Self {
        Self {
            filter: Mutex::new(filter),
            prefixes,
            prefix_max,
            enclave,
        }
    }

    pub fn filter_client_hello(&self, request: &[u8]) -> Result<(), StarFilterErrorStatus> {
        let (token, route) = self.issue_token_and_route_sync(request)?;
        self.check_duplication(&token, route)
    }

    pub async fn issue_token_and_route(
        &self,
        request: &[u8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        self.issue_token_and_route_sync(request)
    }

    pub fn issue_token_and_route_sync(
        &self,
        request: &[u8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let token = self
            .enclave
            .issue_token(request, constant_time_utils::time::current_period())
            .map_err(|_| StarFilterErrorStatus::Internal)?;

        self.token_and_route(&token)
    }

    fn token_and_route(&self, token_bytes: &[u8]) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let token_entry: Entry = token_bytes
            .get(..32)
            .ok_or(StarFilterErrorStatus::InvalidToken)?
            .try_into()
            .map_err(|_| StarFilterErrorStatus::InvalidToken)?;
        let token = Token::new(token_entry);

        Ok((token.token, token.route(self.prefix_max)))
    }

    pub fn check_duplication(
        &self,
        token: &Entry,
        route: u32,
    ) -> Result<(), StarFilterErrorStatus> {
        if !self.prefixes.contains(&route) {
            return Err(StarFilterErrorStatus::InvalidRoute);
        }

        let mut filter = self
            .filter
            .lock()
            .map_err(|_| StarFilterErrorStatus::Internal)?;

        if filter
            .test_and_insert(token)
            .map_err(|_| StarFilterErrorStatus::Internal)?
        {
            Ok(())
        } else {
            Err(StarFilterErrorStatus::DuplicatedToken)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_filter_box::hashmap::HashMapFilterBox;

    #[test]
    #[ignore = "requires SGX hardware and the Fortanix SGX build tools"]
    fn shard_lists_are_independent_and_reject_wrong_routes() {
        let filters: Vec<_> = (0..2)
            .map(|shard| {
                StarFilter::new(
                    HashMapFilterBox::default(),
                    vec![shard],
                    2,
                    Arc::new(EnclaveRuntime::from_env().unwrap()),
                )
            })
            .collect();
        let token = [0u8; 32];
        assert!(matches!(
            filters[1].check_duplication(&token, 0),
            Err(StarFilterErrorStatus::InvalidRoute)
        ));
        assert!(filters[0].check_duplication(&token, 0).is_ok());
        assert!(matches!(
            filters[0].check_duplication(&token, 0),
            Err(StarFilterErrorStatus::DuplicatedToken)
        ));
        // The rejected request did not populate shard 1's list, and shard 0's
        // insertion did not affect it. Routing was already checked above.
        assert!(filters[1].check_duplication(&token, 1).is_ok());
    }
}
