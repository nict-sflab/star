use super::*;

const QUOTE_V5_BODY_OFFSET: usize = 54;
const SGX_REPORT_BODY_LEN: usize = 384;
const QUOTE_V5: u16 = 5;
const SGX_QL_ALG_ECDSA_P256: u16 = 2;
const QUOTE_BODY_TYPE_SGX: u16 = 1;

fn fake_quote(
    mrenclave: [u8; MEASUREMENT_LEN],
    mrsigner: [u8; MEASUREMENT_LEN],
    public_key: [u8; PUBLIC_KEY_LEN],
    debug: bool,
) -> alloc::vec::Vec<u8> {
    let mut quote = alloc::vec![0u8; QUOTE_V5_BODY_OFFSET + SGX_REPORT_BODY_LEN];
    quote[0..2].copy_from_slice(&QUOTE_V5.to_le_bytes());
    quote[2..4].copy_from_slice(&SGX_QL_ALG_ECDSA_P256.to_le_bytes());
    quote[48..50].copy_from_slice(&QUOTE_BODY_TYPE_SGX.to_le_bytes());
    quote[50..54].copy_from_slice(&(SGX_REPORT_BODY_LEN as u32).to_le_bytes());
    let report = QUOTE_V5_BODY_OFFSET;
    let flags = if debug { 0x2u64 } else { 0 };
    quote[report + 48..report + 56].copy_from_slice(&flags.to_le_bytes());
    quote[report + 64..report + 96].copy_from_slice(&mrenclave);
    quote[report + 128..report + 160].copy_from_slice(&mrsigner);
    quote[report + 320..report + 352].copy_from_slice(&public_key);
    quote
}

#[test]
fn quote_verification() {
    let mrenclave = [1u8; MEASUREMENT_LEN];
    let mrsigner = [2u8; MEASUREMENT_LEN];
    let public_key = [3u8; PUBLIC_KEY_LEN];
    let quote = fake_quote(mrenclave, mrsigner, public_key, true);

    let policy = QuotePolicy {
        expected_mrenclave: Some(mrenclave.into()),
        expected_mrsigner: None,
        allow_debug: true,
        allow_advisory: false,
    };

    let quote = Quote::parse(&quote).expect("parse quote");
    quote.verify(&policy).expect("verify quote");
    assert_eq!(quote.public_key().expect("extract public key"), public_key);
}

#[test]
fn mrenclave_mismatch() {
    let quote = fake_quote(
        [1u8; MEASUREMENT_LEN],
        [2u8; MEASUREMENT_LEN],
        [3u8; PUBLIC_KEY_LEN],
        false,
    );
    let policy = QuotePolicy {
        expected_mrenclave: Some([9u8; MEASUREMENT_LEN].into()),
        expected_mrsigner: None,
        allow_debug: false,
        allow_advisory: false,
    };

    let quote = Quote::parse(&quote).expect("parse quote");
    assert!(quote.verify(&policy).is_err());
}

#[test]
fn evidence_roundtrip() {
    let quote = [1u8; 64];
    let collateral = [2u8; 96];
    let evidence = Evidence {
        quote: &quote,
        collateral: &collateral,
    };

    let encoded = evidence.encode();
    let decoded = Evidence::decode(&encoded).expect("decode evidence");

    assert_eq!(decoded, evidence);
}

#[test]
fn measurement_hex() {
    let measurement = "ab:".repeat(MEASUREMENT_LEN);
    assert_eq!(
        Measurement::parse_measurement_hex(&measurement, "MRENCLAVE").expect("parse measurement"),
        Measurement::new([0xabu8; MEASUREMENT_LEN])
    );
}
