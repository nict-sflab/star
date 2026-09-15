use std::io::{self, Read};

use clap::Parser;
use hex::FromHex;
use star_attestation::{Measurement, PUBLIC_KEY_LEN, Quote, QuotePolicy};
use star_tools::{Verifier, decode_quote_input};

#[derive(Debug, Parser)]
#[command(
    name = "star_tools",
    about = "Inspect and verify SGX attestation quotes from stdin"
)]
enum Command {
    /// Display measurements and the public key without verifying authenticity.
    Inspect,
    /// Verify DCAP evidence and the trusted enclave identity.
    Verify {
        #[arg(long, required_unless_present = "mrenclave", value_parser = |value: &str| Measurement::parse_measurement_hex(value, "MRSIGNER"))]
        mrsigner: Option<Measurement>,
        #[arg(long, value_parser = |value: &str| Measurement::parse_measurement_hex(value, "MRENCLAVE"))]
        mrenclave: Option<Measurement>,
        #[arg(long, value_parser = |value: &str| <[u8; PUBLIC_KEY_LEN]>::from_hex(value))]
        public_key: Option<[u8; PUBLIC_KEY_LEN]>,
        /// Accept development enclaves with the debug attribute set.
        #[arg(long)]
        allow_debug: bool,
        /// Accept advisory TCB statuses. Revoked quotes are always rejected.
        #[arg(long)]
        allow_advisory: bool,
        /// Override PCCS_URL and the default PCCS endpoint.
        #[arg(long)]
        pccs_url: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::parse();
    let verifying = matches!(command, Command::Verify { .. });
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let bytes = decode_quote_input(&input)?;

    let quote = Quote::parse(&bytes)?;
    let public_key = quote.public_key()?;
    if let Command::Verify {
        mrsigner,
        mrenclave,
        public_key: expected_public_key,
        allow_debug,
        allow_advisory,
        pccs_url,
    } = command
    {
        if let Some(expected) = expected_public_key
            && public_key != expected
        {
            return Err("attested public key does not match expected public key".into());
        }
        let policy = QuotePolicy {
            expected_mrsigner: mrsigner,
            expected_mrenclave: mrenclave,
            allow_debug,
            allow_advisory,
        };
        Verifier::new(pccs_url.as_deref())?
            .verify(&bytes, &policy)
            .await?;
    }
    println!("quote size: {} bytes", bytes.len());
    println!("public key: {}", hex::encode(public_key));
    println!("mrenclave: {}", hex::encode(quote.mrenclave));
    println!("mrsigner : {}", hex::encode(quote.mrsigner));
    println!("debug    : {}", quote.debug);
    if verifying {
        println!("DCAP verification, enclave policy, and public key binding OK");
    }
    Ok(())
}

#[path = "main_test.rs"]
#[cfg(test)]
mod tests;
