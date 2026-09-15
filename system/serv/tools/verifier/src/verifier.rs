use dcap_qvl::collateral::CollateralClient;
use parity_scale_codec::Encode;
use star_attestation::{Evidence, Quote, QuotePolicy, Result};

/// Verifies quotes using collateral fetched from PCCS.
pub struct Verifier {
    collateral: CollateralClient,
}

impl Verifier {
    pub fn new(pccs_url: Option<&str>) -> Result<Self> {
        let collateral = match pccs_url {
            Some(url) => CollateralClient::with_default_http(url),
            None => CollateralClient::from_env(),
        }
        .map_err(|err| format!("create PCCS client: {err}"))?;
        Ok(Self { collateral })
    }

    pub async fn verify(&self, quote: &[u8], policy: &QuotePolicy) -> Result<Quote> {
        Quote::parse(quote)?.verify(policy)?;
        let collateral = self
            .collateral
            .fetch(quote)
            .await
            .map_err(|err| format!("fetch DCAP collateral: {err}"))?
            .encode();
        Evidence {
            quote,
            collateral: &collateral,
        }
        .verify(policy)
    }
}
