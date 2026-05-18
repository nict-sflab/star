use simple_filter_box::core::{Entry, FilterBox};
use star_core::serv::core::token::Token;
use std::sync::Mutex;

use crate::enclave::EnclaveRuntimes;

pub struct StarFilter<T: FilterBox + Send> {
    pub filter: Mutex<T>,
    pub prefix_max: u32,
    pub prefixes: Vec<u32>,
    pub enclaves: EnclaveRuntimes,
}

#[derive(Debug)]
pub enum StarFilterErrorStatus {
    InvalidToken,
    InvalidRoute,
    DuplicatedToken,
    Internal,
}

impl StarFilterErrorStatus {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::InvalidToken => 401,
            Self::InvalidRoute => 421,
            Self::DuplicatedToken => 429,
            Self::Internal => 500,
        }
    }
}

impl<T: FilterBox + Send> StarFilter<T> {
    pub fn new(filter: T, prefixes: Vec<u32>, prefix_max: u32, enclaves: EnclaveRuntimes) -> Self {
        Self {
            filter: Mutex::new(filter),
            prefixes,
            prefix_max,
            enclaves,
        }
    }

    pub async fn filter(&self, request: &[u8]) -> Result<(), StarFilterErrorStatus> {
        let (token, route) = self.issue_token_and_route(request).await?;
        self.check_duplication(&token, route)?;

        Ok(())
    }

    pub fn filter_client_hello(&self, request: &[u8]) -> Result<(), StarFilterErrorStatus> {
        let (token, route) = self.issue_token_and_route_sync(request)?;
        self.check_duplication(&token, route)?;
        Ok(())
    }

    pub async fn issue_token_and_route(
        &self,
        request: &[u8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let current_period = constant_time_utils::time::current_period();
        let prefix_max = self.prefix_max;

        let mut enclave = self
            .enclaves
            .pick()
            .await
            .ok_or(StarFilterErrorStatus::Internal)?;

        let token_bytes = enclave
            .issue_token(request, current_period)
            .map_err(|_| StarFilterErrorStatus::Internal)?;

        if token_bytes.len() < 32 {
            return Err(StarFilterErrorStatus::InvalidToken);
        }

        let token_entry: Entry = token_bytes[..32]
            .try_into()
            .map_err(|_| StarFilterErrorStatus::InvalidToken)?;

        let token = Token::new(token_entry);
        let route = token.route(prefix_max);

        Ok((token.token, route))
    }
    pub fn issue_token_and_route_sync(
        &self,
        request: &[u8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let current_period = constant_time_utils::time::current_period();

  
        let result = (|| {
            let enclave = self
                .enclaves
                .pick_blocking()
                .ok_or(StarFilterErrorStatus::Internal)?;

            let token_bytes = enclave
                .issue_token(request, current_period)
                .map_err(|_| StarFilterErrorStatus::Internal)?;

            if token_bytes.len() < 32 {
                return Err(StarFilterErrorStatus::InvalidToken);
            }

            let token_entry: Entry = token_bytes[..32]
                .try_into()
                .map_err(|_| StarFilterErrorStatus::InvalidToken)?;

            let token = Token::new(token_entry);
            let route = token.route(self.prefix_max);

            Ok((token.token, route))
        })();


        result
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

        let is_new = filter.test(token);
        if is_new {
            filter
                .insert(token)
                .map_err(|_| StarFilterErrorStatus::Internal)?;
        }

        if is_new {
            Ok(())
        } else {
            Err(StarFilterErrorStatus::DuplicatedToken)
        }
    }
}
