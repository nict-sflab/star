#[cfg(test)]
mod tests {
    use super::super::{
        KeyManagerExport, decode_key_manager_export_response, encode_key_manager_export_response,
    };

    #[test]
    fn key_manager_export_response_round_trips() {
        let export = KeyManagerExport {
            key_manager_quote: b"manager quote".to_vec(),
            recipient_evidence: b"recipient evidence".to_vec(),
            envelope: b"encrypted envelope".to_vec(),
        };

        let encoded = encode_key_manager_export_response(&export);
        let decoded = decode_key_manager_export_response(&encoded).expect("decode export response");

        assert_eq!(decoded.key_manager_quote, export.key_manager_quote);
        assert_eq!(decoded.recipient_evidence, export.recipient_evidence);
        assert_eq!(decoded.envelope, export.envelope);
    }

    #[test]
    fn key_manager_export_response_rejects_trailing_bytes() {
        let export = KeyManagerExport {
            key_manager_quote: b"manager quote".to_vec(),
            recipient_evidence: b"recipient evidence".to_vec(),
            envelope: b"encrypted envelope".to_vec(),
        };
        let mut encoded = encode_key_manager_export_response(&export);
        encoded.push(0);

        let err = decode_key_manager_export_response(&encoded).expect_err("reject trailing byte");
        assert!(err.contains("trailing bytes"));
    }
}
