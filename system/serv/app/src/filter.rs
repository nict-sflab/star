use simple_filter_box::core::{Entry, FilterBox};
use star_core::serv::core::token::Token;

use crate::enclave::EnclaveRuntimes;

pub struct StarFilter<T: FilterBox + Send> {
    pub filter: T,
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
            filter,
            prefixes,
            prefix_max,
            enclaves,
        }
    }

    pub async fn filter(&mut self, request: &[u8]) -> Result<(), StarFilterErrorStatus> {
        let (token, route) = self.issue_token_and_route(request).await?;
        self.check_duplication(&token, route).await?;

        Ok(())
    }

    pub async fn issue_token_and_route(
        &mut self,
        request: &[u8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let current_period = constant_time_utils::time::current_period();
        let request = request.to_vec();
        let prefix_max = self.prefix_max;

        let mut enclave = self
            .enclaves
            .pick()
            .await
            .ok_or(StarFilterErrorStatus::Internal)?;

        let result = tokio::task::spawn_blocking(move || {
            let token_bytes = match enclave.issue_token(&request, current_period) {
                Ok(token_bytes) => token_bytes,
                Err(_) => return (enclave, Err(StarFilterErrorStatus::Internal)),
            };

            if token_bytes.len() < 32 {
                return (enclave, Err(StarFilterErrorStatus::InvalidToken));
            }

            let token_entry: Entry = match token_bytes[..32].try_into() {
                Ok(token_entry) => token_entry,
                Err(_) => return (enclave, Err(StarFilterErrorStatus::InvalidToken)),
            };

            let token = Token::new(token_entry);
            let route = token.route(prefix_max);

            (enclave, Ok((token.token, route)))
        })
        .await;

        let (enclave, result) = match result {
            Ok(result) => result,
            Err(_) => return Err(StarFilterErrorStatus::Internal),
        };

        self.enclaves.release(enclave).await;
        result
    }

    pub async fn check_duplication(
        &mut self,
        token: &Entry,
        route: u32,
    ) -> Result<(), StarFilterErrorStatus> {
        if !self.prefixes.contains(&route) {
            return Err(StarFilterErrorStatus::InvalidRoute);
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
