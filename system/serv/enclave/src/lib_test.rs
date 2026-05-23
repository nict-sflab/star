#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn init_rejects_raw_key_bundle() {
        let mut app = EnclaveApp::new(1);
        let bundle = KeyBundle {
            enc_key: [7u8; 32],
            hpke_receiver_key: [8u8; 32],
        };
        let payload = encode_key_bundle(bundle);

        let (status, payload) = app.handle(OP_INIT, &payload);

        assert_eq!(status, STATUS_BAD_REQUEST);
        assert_eq!(
            payload,
            b"init does not accept keys; install key-manager envelope with OP_INSTALL_KEYS"
        );

        let (status, payload) = app.handle(OP_PUBLIC_KEY, &[]);
        assert_eq!(status, STATUS_NOT_INITIALIZED);
        assert_eq!(payload, b"enclave is not initialized");
    }

    #[test]
    fn empty_init_only_prepares_key_manager_transfer() {
        let mut app = EnclaveApp::new(1);

        let (status, payload) = app.handle(OP_INIT, &[]);

        assert_eq!(status, STATUS_OK);
        assert!(payload.is_empty());

        let (status, payload) = app.handle(OP_PUBLIC_KEY, &[]);
        assert_eq!(status, STATUS_NOT_INITIALIZED);
        assert_eq!(payload, b"enclave is not initialized");

        let (status, payload) = app.handle(OP_TRANSFER_PUBLIC_KEY, &[]);
        assert_eq!(status, STATUS_OK);
        assert_eq!(payload.len(), PUBLIC_KEY_LEN);
    }

    #[test]
    fn key_bundle_round_trips() {
        let bundle = KeyBundle {
            enc_key: [7u8; 32],
            hpke_receiver_key: [8u8; 32],
        };

        let encoded = encode_key_bundle(bundle);
        let decoded = decode_key_bundle(&encoded).expect("decode");

        assert_eq!(decoded.enc_key, bundle.enc_key);
        assert_eq!(decoded.hpke_receiver_key, bundle.hpke_receiver_key);
    }

    fn encode_key_bundle(bundle: KeyBundle) -> Vec<u8> {
        let mut out = Vec::with_capacity(64);
        out.extend_from_slice(&bundle.enc_key);
        out.extend_from_slice(&bundle.hpke_receiver_key);
        out
    }

    #[test]
    fn attest_dispatch_rejects_non_64_byte_challenge() {
        let enclave = Enclave::new(1, [1u8; 32], [2u8; 32]).expect("enclave");
        let mut app = EnclaveApp::with_enclave(enclave);

        let (status, payload) = app.handle(OP_ATTEST, b"short");

        assert_eq!(status, STATUS_BAD_REQUEST);
        assert_eq!(payload, b"attest needs 64 bytes");
    }

    #[test]
    fn init_rejects_raw_key_bundle() {
        let mut app = EnclaveApp::new(1);
        let bundle = KeyBundle {
            enc_key: [7u8; 32],
            hpke_receiver_key: [8u8; 32],
        };
        let payload = encode_key_bundle(bundle);

        let (status, payload) = app.handle(OP_INIT, &payload);

        assert_eq!(status, STATUS_BAD_REQUEST);
        assert_eq!(
            payload,
            b"init does not accept keys; install key-manager envelope with OP_INSTALL_KEYS"
        );

        let (status, payload) = app.handle(OP_PUBLIC_KEY, &[]);
        assert_eq!(status, STATUS_NOT_INITIALIZED);
        assert_eq!(payload, b"enclave is not initialized");
    }

    #[test]
    fn empty_init_only_prepares_key_manager_transfer() {
        let mut app = EnclaveApp::new(1);

        let (status, payload) = app.handle(OP_INIT, &[]);

        assert_eq!(status, STATUS_OK);
        assert!(payload.is_empty());

        let (status, payload) = app.handle(OP_PUBLIC_KEY, &[]);
        assert_eq!(status, STATUS_NOT_INITIALIZED);
        assert_eq!(payload, b"enclave is not initialized");

        let (status, payload) = app.handle(OP_TRANSFER_PUBLIC_KEY, &[]);
        assert_eq!(status, STATUS_OK);
        assert_eq!(payload.len(), PUBLIC_KEY_LEN);
    }

    #[test]
    fn key_bundle_round_trips() {
        let bundle = KeyBundle {
            enc_key: [7u8; 32],
            hpke_receiver_key: [8u8; 32],
        };

        let encoded = encode_key_bundle(bundle);
        let decoded = decode_key_bundle(&encoded).expect("decode");

        assert_eq!(decoded.enc_key, bundle.enc_key);
        assert_eq!(decoded.hpke_receiver_key, bundle.hpke_receiver_key);
    }
}
