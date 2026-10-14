#[cfg(feature = "dcap-verify")]
use alloc::format;
#[cfg(feature = "dcap-verify")]
use dcap_qvl::tcb_info::TcbStatus;
#[cfg(feature = "dcap-verify")]
use parity_scale_codec::Decode;

#[cfg(feature = "dcap-verify")]
use crate::Evidence;
use crate::{Quote, QuotePolicy, Result};

impl Quote {
    pub fn verify(&self, policy: &QuotePolicy) -> Result<()> {
        let mut has_identity_policy = false;

        if let Some(expected_mrenclave) = policy.expected_mrenclave {
            has_identity_policy = true;
            if self.mrenclave != expected_mrenclave {
                return Err("MRENCLAVE does not match the trusted enclave measurement".into());
            }
        }

        if let Some(expected_mrsigner) = policy.expected_mrsigner {
            has_identity_policy = true;
            if self.mrsigner != expected_mrsigner {
                return Err("MRSIGNER does not match the trusted enclave signer".into());
            }
        }

        if !has_identity_policy {
            return Err("no trusted enclave identity policy is configured".into());
        }

        if self.debug && !policy.allow_debug {
            return Err("debug enclave is not trusted".into());
        }

        Ok(())
    }
}

#[cfg(feature = "dcap-verify")]
impl Evidence<'_> {
    pub fn verify(&self, policy: &QuotePolicy) -> Result<Quote> {
        let mut collateral_bytes = self.collateral;
        let collateral = dcap_qvl::QuoteCollateralV3::decode(&mut collateral_bytes)
            .map_err(|err| format!("parse DCAP collateral: {err}"))?;
        if !collateral_bytes.is_empty() {
            return Err("trailing DCAP collateral bytes".into());
        }

        let quote = Quote::parse(self.quote)?;
        quote.verify(policy)?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|err| format!("system clock is before UNIX_EPOCH: {err}"))?
            .as_secs();
        let verified = dcap_qvl::verify::QuoteVerifier::new_prod()
            .allow_debug(policy.allow_debug)
            .verify(self.quote, &collateral, now)
            .map_err(|err| format!("verify DCAP quote: {err}"))?;

        let status = verified
            .platform_status
            .clone()
            .merge(&verified.qe_status)
            .status;
        match status {
            TcbStatus::UpToDate => {}
            TcbStatus::Revoked => return Err("quote TCB status is Revoked".into()),
            _ if policy.allow_advisory => {}
            _ => {
                return Err(format!(
                    "quote TCB status is {status}; advisory status is not allowed"
                ));
            }
        }

        Ok(quote)
    }
}
