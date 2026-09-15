use std::{collections::HashMap, io, sync::Mutex};

use simple_filter_box::core::{Entry, FilterBox};
use star_core::serv::core::token::Token;

use crate::enclave::EnclaveRuntimes;


pub struct StarFilter<T: FilterBox + Send + Sync> {
    filter: T,
    prefixes: Vec<u32>,
    prefix_max: u32,
    enclaves: EnclaveRuntimes,
}

#[derive(Debug)]
pub enum StarFilterErrorStatus {
    InvalidToken,
    InvalidRoute,
    DuplicatedToken,
    Internal,
}

impl<T: FilterBox + Send + Sync> StarFilter<T> {
    pub fn new(filter: T, prefixes: Vec<u32>, prefix_max: u32, enclaves: EnclaveRuntimes) -> Self {
        Self {
            filter,
            prefixes,
            prefix_max,
            enclaves,
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
        let enclave = self
            .enclaves
            .pick()
            .await
            .ok_or(StarFilterErrorStatus::Internal)?;
        let token = enclave
            .issue_token(request, constant_time_utils::time::current_period())
            .map_err(|_| StarFilterErrorStatus::Internal)?;

        self.token_and_route(&token)
    }

    pub fn issue_token_and_route_sync(
        &self,
        request: &[u8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let enclave = self
            .enclaves
            .pick_blocking()
            .ok_or(StarFilterErrorStatus::Internal)?;
        let token = enclave
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

        if self
            .filter
            .test_and_insert(token)
            .map_err(|_| StarFilterErrorStatus::Internal)?
        {
            Ok(())
        } else {
            Err(StarFilterErrorStatus::DuplicatedToken)
        }
    }
}
