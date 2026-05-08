use std::collections::HashMap;

use simple_filter_box::core::{Entry, FilterBox};
use star_core::serv::core::token::Token;
use tokio::sync::Mutex;

use crate::core::EnclaveRuntime;

pub struct StarFilter<T: FilterBox + Send> {
    pub filter: T,
    pub prefix_max: u32,
    pub prefixes: Vec<u32>,
    pub enclave: EnclaveRuntime
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
    pub fn new(filter: T, prefixes: Vec<u32>, prefix_max: u32, enclave: EnclaveRuntime) -> Self {
        Self {
            filter,
            prefixes,
            prefix_max,
            enclave
        }
    }

    pub async fn filter(&mut self, request: &[u8]) -> Result<(), StarFilterErrorStatus> {
        let (token, route) = self.issue_token_and_route(request)?;
        self.check_duplication(&token, route).await?;

        Ok(())
    }

    pub fn issue_token_and_route(
        &mut self,
        request: &[u8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let current_period = constant_time_utils::time::current_period();

        let token_bytes = self.enclave
            .issue_token(request, current_period);

        if token_bytes.len() < 32 {
            return Err(StarFilterErrorStatus::InvalidToken);
        }

        let token_entry: Entry = token_bytes[..32]
            .try_into()
            .map_err(|_| StarFilterErrorStatus::InvalidToken)?;

        let token = Token::new(token_entry);
        let route = token.route(self.prefix_max);

        Ok((token.token, route))
    }

    pub async fn check_duplication(
        &mut self,
        token: &Entry,
        route: u32,
    ) -> Result<(), StarFilterErrorStatus> {
        if !self.prefixes.contains(&route) {
            return Err(StarFilterErrorStatus::InvalidRoute)
        }

        let is_new = self.filter.test(token);
        if is_new {
            self.filter
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
