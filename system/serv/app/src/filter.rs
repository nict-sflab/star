use std::collections::HashMap;

use simple_filter_box::core::{Entry, FilterBox};
use star_core::serv::core::token::Token;
use tokio::sync::Mutex;

use crate::core::ENCLAVE;

pub struct StarFilter<T: FilterBox + Send> {
    pub filters: HashMap<u32, Mutex<T>>,
    pub prefix_max: u32,
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
    pub fn new(filters: HashMap<u32, Mutex<T>>, prefix_max: u32) -> Self {
        Self {
            filters,
            prefix_max,
        }
    }

    pub async fn filter(&self, request: &[u8]) -> Result<(), StarFilterErrorStatus> {
        let (token, route) = self.issue_token_and_route(request)?;
        self.check_duplication(&token, route).await?;

        Ok(())
    }

    pub fn issue_token_and_route(
        &self,
        request: &[u8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let enclave = ENCLAVE.get().ok_or(StarFilterErrorStatus::Internal)?;
        let current_period = constant_time_utils::time::current_period();

        let token_bytes = enclave
            .issue_token(request, current_period)
            .map_err(|_| StarFilterErrorStatus::InvalidToken)?;
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
        &self,
        token: &Entry,
        route: u32,
    ) -> Result<(), StarFilterErrorStatus> {
        let filter = self
            .filters
            .get(&route)
            .ok_or(StarFilterErrorStatus::InvalidRoute)?;

        let mut filter_box = filter.lock().await;
        let is_new = filter_box.test(token);
        if is_new {
            filter_box
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
