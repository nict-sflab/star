//! Synchronous wait/send for the direct TCSes (e.g. a contended enclave allocator).
//! Addresses and event bits are public ABI metadata, never enclave secrets.
use std::collections::BTreeMap;
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

use fast_rust_sgx_caller::Error;
use fortanix_sgx_abi::{
    EV_CANCELQ_NOT_FULL, EV_RETURNQ_NOT_EMPTY, EV_UNPARK, EV_USERCALLQ_NOT_FULL, Error as AbiError,
    WAIT_INDEFINITE, WAIT_NO,
};

const ALL_EVENTS: u64 =
    EV_USERCALLQ_NOT_FULL | EV_RETURNQ_NOT_EMPTY | EV_UNPARK | EV_CANCELQ_NOT_FULL;

pub(super) struct Events {
    state: Mutex<State>,
    changed: Condvar,
}

struct State {
    pending: BTreeMap<usize, [u64; 16]>,
    stopped: bool,
}

impl Events {
    pub(super) fn new(addresses: impl Iterator<Item = usize>) -> Self {
        Self {
            state: Mutex::new(State {
                pending: addresses.map(|address| (address, [0; 16])).collect(),
                stopped: false,
            }),
            changed: Condvar::new(),
        }
    }

    pub(super) fn stop(&self) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.stopped = true;
        self.changed.notify_all();
    }

    pub(super) fn send(&self, events: u64, address: usize) -> Result<(u64, u64), Error> {
        if events == 0 || events & !ALL_EVENTS != 0 {
            return Ok((AbiError::InvalidInput as u64, 0));
        }
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if state.stopped {
            return Err(Error::Stopped);
        }
        if address == 0 {
            for pending in state.pending.values_mut() {
                pending[events as usize] = pending[events as usize].saturating_add(1);
            }
        } else if let Some(pending) = state.pending.get_mut(&address) {
            pending[events as usize] = pending[events as usize].saturating_add(1);
        } else {
            return Ok((AbiError::InvalidInput as u64, 0));
        }
        self.changed.notify_all();
        Ok((0, 0))
    }

    pub(super) fn wait(
        &self,
        address: usize,
        mask: u64,
        timeout: u64,
    ) -> Result<(u64, u64), Error> {
        if mask & !ALL_EVENTS != 0 {
            return Ok((AbiError::InvalidInput as u64, 0));
        }
        let start = Instant::now();
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if state.stopped {
                return Err(Error::Stopped);
            }
            let Some(pending) = state.pending.get_mut(&address) else {
                return Ok((AbiError::InvalidInput as u64, 0));
            };
            // A queued event set must be a subset of the requested mask.
            // Keep counts so repeated sends are not lost.
            for event in (1..=ALL_EVENTS as usize).rev() {
                if event as u64 & mask == event as u64 && pending[event] != 0 {
                    pending[event] -= 1;
                    return Ok((0, event as u64));
                }
            }
            if timeout == WAIT_NO {
                return Ok((AbiError::WouldBlock as u64, 0));
            }
            if timeout == WAIT_INDEFINITE {
                state = self.changed.wait(state).unwrap_or_else(|e| e.into_inner());
            } else {
                let Some(remaining) = Duration::from_nanos(timeout).checked_sub(start.elapsed())
                else {
                    return Ok((AbiError::TimedOut as u64, 0));
                };
                state = self
                    .changed
                    .wait_timeout(state, remaining)
                    .unwrap_or_else(|e| e.into_inner())
                    .0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fortanix_sgx_abi::{EV_RETURNQ_NOT_EMPTY, EV_UNPARK};

    #[test]
    fn events_are_targeted_masked_consumed_and_broadcast() {
        let events = Events::new([1, 2].into_iter());
        assert_eq!(
            events.send(EV_UNPARK, 3).unwrap(),
            (AbiError::InvalidInput as u64, 0)
        );
        events.send(EV_UNPARK, 1).unwrap();
        events.send(EV_RETURNQ_NOT_EMPTY, 1).unwrap();
        assert_eq!(events.wait(1, EV_UNPARK, WAIT_NO).unwrap(), (0, EV_UNPARK));
        assert_eq!(
            events.wait(1, EV_UNPARK, WAIT_NO).unwrap(),
            (AbiError::WouldBlock as u64, 0)
        );
        assert_eq!(
            events.wait(1, EV_RETURNQ_NOT_EMPTY, WAIT_NO).unwrap(),
            (0, EV_RETURNQ_NOT_EMPTY)
        );
        assert_eq!(
            events.wait(2, EV_UNPARK, 1).unwrap(),
            (AbiError::TimedOut as u64, 0)
        );
        events.send(EV_UNPARK, 0).unwrap();
        for address in [1, 2] {
            assert_eq!(
                events.wait(address, EV_UNPARK, WAIT_NO).unwrap(),
                (0, EV_UNPARK)
            );
        }
    }

    #[test]
    fn queued_event_sets_keep_their_counts_and_masks() {
        let events = Events::new([1].into_iter());
        let both = EV_UNPARK | EV_RETURNQ_NOT_EMPTY;
        events.send(both, 1).unwrap();
        events.send(both, 1).unwrap();
        assert_eq!(
            events.wait(1, EV_UNPARK, WAIT_NO).unwrap(),
            (AbiError::WouldBlock as u64, 0)
        );
        for _ in 0..2 {
            assert_eq!(events.wait(1, both, WAIT_NO).unwrap(), (0, both));
        }
        assert_eq!(
            events.wait(1, both, WAIT_NO).unwrap(),
            (AbiError::WouldBlock as u64, 0)
        );
        for invalid in [0, ALL_EVENTS + 1] {
            assert_eq!(
                events.send(invalid, 1).unwrap(),
                (AbiError::InvalidInput as u64, 0)
            );
        }
        assert_eq!(
            events.wait(1, ALL_EVENTS + 1, WAIT_NO).unwrap(),
            (AbiError::InvalidInput as u64, 0)
        );
    }

    #[test]
    fn send_and_shutdown_wake_waiters() {
        let events = Events::new([1, 2].into_iter());
        std::thread::scope(|scope| {
            let first = scope.spawn(|| events.wait(1, EV_UNPARK, WAIT_INDEFINITE));
            events.send(EV_UNPARK, 1).unwrap();
            assert_eq!(first.join().unwrap().unwrap(), (0, EV_UNPARK));
            let second = scope.spawn(|| events.wait(2, EV_UNPARK, WAIT_INDEFINITE));
            events.stop();
            assert!(matches!(second.join().unwrap(), Err(Error::Stopped)));
            assert!(matches!(events.send(EV_UNPARK, 1), Err(Error::Stopped)));
        });
    }
}
