use base64::{
    Engine as _,
    engine::general_purpose::{URL_SAFE, URL_SAFE_NO_PAD},
};
use star_attestation::QuotePolicy;

use super::*;

#[path = "../tests/common/mod.rs"]
mod common;
use common::fake_quote;

#[test]
fn quote_input() {
    // Include URL-safe '-' and '_' characters, plus trailing '=' padding.
    let mut quote = fake_quote();
    quote.extend_from_slice(&[0xfb, 0xff, 0xff, 0]);
    for input in [
        quote.clone(),
        format!("  {}\n", URL_SAFE.encode(&quote)).into_bytes(),
        URL_SAFE_NO_PAD.encode(&quote).into_bytes(),
    ] {
        assert_eq!(decode_quote_input(&input).unwrap(), quote);
    }
}

#[test]
fn startup_log() {
    let quote = fake_quote();
    let input = format!(
        "starting\nSTAR_ATTESTATION_QUOTE_BASE64=old\n\
         STAR_ATTESTATION_PUBLIC_KEY_BASE64={}\n\
         STAR_ATTESTATION_QUOTE_BASE64={}\nready\n",
        URL_SAFE.encode([3; 32]),
        URL_SAFE.encode(&quote),
    );
    assert_eq!(decode_quote_input(input.as_bytes()).unwrap(), quote);
}

#[test]
fn rejects_invalid_quotes() {
    let mut old_quote = fake_quote();
    old_quote[..2].copy_from_slice(&3u16.to_le_bytes());
    for input in [
        Vec::new(),
        vec![0xff],
        b"invalid base64!".to_vec(),
        URL_SAFE.encode(b"not a quote").into_bytes(),
        URL_SAFE.encode(&old_quote).into_bytes(),
        URL_SAFE.encode(&fake_quote()[..100]).into_bytes(),
    ] {
        assert!(decode_quote_input(&input).is_err());
    }
}

#[test]
fn invalid_latest_quote() {
    let input = format!(
        "STAR_ATTESTATION_QUOTE_BASE64={}\nSTAR_ATTESTATION_QUOTE_BASE64=invalid!\n",
        URL_SAFE.encode(fake_quote()),
    );
    assert!(decode_quote_input(input.as_bytes()).is_err());
}

#[tokio::test]
async fn untrusted_quote() {
    let verifier = Verifier::new(Some("http://127.0.0.1:1")).unwrap();
    let trusted = QuotePolicy {
        expected_mrenclave: Some([1; 32].into()),
        expected_mrsigner: Some([2; 32].into()),
        allow_debug: false,
        allow_advisory: false,
    };
    for (policy, error) in [
        (
            QuotePolicy {
                expected_mrenclave: Some([9; 32].into()),
                ..trusted.clone()
            },
            "MRENCLAVE does not match",
        ),
        (
            QuotePolicy {
                expected_mrsigner: Some([9; 32].into()),
                ..trusted.clone()
            },
            "MRSIGNER does not match",
        ),
        (
            QuotePolicy {
                expected_mrenclave: None,
                expected_mrsigner: None,
                ..trusted.clone()
            },
            "no trusted enclave identity",
        ),
    ] {
        let err = verifier.verify(&fake_quote(), &policy).await.unwrap_err();
        assert!(err.contains(error), "{err}");
    }

    let mut quote = fake_quote();
    quote[54 + 48] = 2;
    let err = verifier.verify(&quote, &trusted).await.unwrap_err();
    assert!(err.contains("debug enclave"), "{err}");
}

#[tokio::test]
async fn unsigned_quote() {
    let verifier = Verifier::new(Some("http://127.0.0.1:1")).unwrap();
    let policy = QuotePolicy {
        expected_mrenclave: Some([1; 32].into()),
        expected_mrsigner: Some([2; 32].into()),
        allow_debug: false,
        allow_advisory: false,
    };

    // The local policy passes; DCAP must still reject the missing signature data.
    let err = verifier.verify(&fake_quote(), &policy).await.unwrap_err();
    assert!(err.contains("fetch DCAP collateral"), "{err}");
}
