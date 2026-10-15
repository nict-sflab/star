use std::sync::{Arc, Mutex};

use simple_filter_box::core::{Entry, FilterBox};
use star_core::serv::core::token::Token;

use crate::enclave::EnclaveRuntime;

struct PeriodHls<T: FilterBox> {
    previous: Option<(u64, T)>,
    current: (u64, T),
    next: (u64, T),
}

impl<T: FilterBox> PeriodHls<T> {
    fn test_and_insert(
        &mut self,
        period: u64,
        token: &Entry,
    ) -> Result<bool, StarFilterErrorStatus> {
        let hl = if period == self.current.0 {
            &mut self.current.1
        } else if period == self.next.0 {
            &mut self.next.1
        } else {
            match &mut self.previous {
                Some((previous_period, hl)) if *previous_period == period => hl,
                _ => return Err(StarFilterErrorStatus::Internal),
            }
        };
        hl.test_and_insert(token)
            .map_err(|_| StarFilterErrorStatus::Internal)
    }
}

impl<T: FilterBox + Default> PeriodHls<T> {
    fn new(current: u64, filter: T) -> Self {
        Self {
            previous: None,
            current: (current, filter),
            next: (current.checked_add(1).expect("STAR period overflow"), T::default()),
        }
    }

    fn maintain(&mut self, current: u64) -> [Option<T>; 3] {
        if current <= self.current.0 {
            return [None, None, None];
        }

        let next_period = current.checked_add(1).expect("STAR period overflow");
        if current == self.next.0 {
            let expired = self.previous.take().map(|(_, hl)| hl);
            let next = std::mem::replace(&mut self.next, (next_period, T::default()));
            let previous = std::mem::replace(&mut self.current, next);
            self.previous = Some(previous);
            return [expired, None, None];
        }

        if self.next.0.checked_add(1) == Some(current) {
            let expired = self.previous.take().map(|(_, hl)| hl);
            let old_current = std::mem::replace(&mut self.current, (current, T::default()));
            self.previous = Some(std::mem::replace(
                &mut self.next,
                (next_period, T::default()),
            ));
            return [expired, Some(old_current.1), None];
        }
        let old = std::mem::replace(self, Self::new(current, T::default()));
        [
            old.previous.map(|(_, hl)| hl),
            Some(old.current.1),
            Some(old.next.1),
        ]
    }
}

pub struct StarFilter<T: FilterBox + Send + Sync> {
    filter: Mutex<PeriodHls<T>>,
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
    ) -> Self
    where
        T: Default,
    {
        let current = u64::from_be_bytes(constant_time_utils::time::current_period());
        Self {
            filter: Mutex::new(PeriodHls::new(current, filter)),
            prefixes,
            prefix_max,
            enclave,
        }
    }

    pub fn maintain_hls(&self) -> Result<(), StarFilterErrorStatus>
    where
        T: Default,
    {
        let current = u64::from_be_bytes(constant_time_utils::time::current_period());
        let expired = self
            .filter
            .lock()
            .map_err(|_| StarFilterErrorStatus::Internal)?
            .maintain(current);
        drop(expired);
        Ok(())
    }

    pub fn filter_client_hello(&self, request: &[u8]) -> Result<(), StarFilterErrorStatus> {
        let period = constant_time_utils::time::current_period();
        let (token, route) = self.issue_token_and_route_at_period(request, period)?;
        self.check_duplication_in_period(&token, route, u64::from_be_bytes(period))
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
        self.issue_token_and_route_at_period(request, constant_time_utils::time::current_period())
    }

    fn issue_token_and_route_at_period(
        &self,
        request: &[u8],
        period: [u8; 8],
    ) -> Result<(Entry, u32), StarFilterErrorStatus> {
        let token = self
            .enclave
            .issue_token(request, period)
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
        let current = u64::from_be_bytes(constant_time_utils::time::current_period());
        self.check_duplication_in_period(token, route, current)
    }

    fn check_duplication_in_period(
        &self,
        token: &Entry,
        route: u32,
        period: u64,
    ) -> Result<(), StarFilterErrorStatus> {
        if !self.prefixes.contains(&route) {
            return Err(StarFilterErrorStatus::InvalidRoute);
        }

        let mut filter = self
            .filter
            .lock()
            .map_err(|_| StarFilterErrorStatus::Internal)?;

        if filter.test_and_insert(period, token)? {
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
    fn rotates_hls_without_losing_replay_history() {
        let token = [7u8; 32];
        let mut hls = PeriodHls::new(41, HashMapFilterBox::default());
        assert!(hls.test_and_insert(41, &token).unwrap());
        assert!(matches!(
            hls.test_and_insert(43, &token),
            Err(StarFilterErrorStatus::Internal)
        ));

        drop(hls.maintain(42));
        assert!(!hls.test_and_insert(41, &token).unwrap());
        assert!(hls.test_and_insert(42, &token).unwrap());

        let expired = hls.maintain(43);
        assert!(expired[0].is_some());
        drop(expired);
        assert_eq!((hls.current.0, hls.next.0), (43, 44));
        assert!(!hls.test_and_insert(42, &token).unwrap());
        assert!(matches!(
            hls.test_and_insert(41, &token),
            Err(StarFilterErrorStatus::Internal)
        ));
    }

    #[test]
    fn skipped_periods_and_rollback_do_not_reset_history() {
        let token = [3u8; 32];
        let mut hls = PeriodHls::new(7, HashMapFilterBox::default());
        assert!(hls.test_and_insert(8, &token).unwrap());
        drop(hls.maintain(9));
        assert_eq!(hls.previous.as_ref().map(|(period, _)| *period), Some(8));
        assert!(!hls.test_and_insert(8, &token).unwrap());

        drop(hls.maintain(8));
        assert!(!hls.test_and_insert(8, &token).unwrap());

        drop(hls.maintain(15));
        assert!(hls.previous.is_none());
        assert_eq!((hls.current.0, hls.next.0), (15, 16));
        assert!(matches!(
            hls.test_and_insert(8, &token),
            Err(StarFilterErrorStatus::Internal)
        ));
    }

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
        assert!(filters[1].check_duplication(&token, 1).is_ok());
    }
}
