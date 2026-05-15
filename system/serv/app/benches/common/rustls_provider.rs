use std::sync::Once;

pub fn ensure_rustls_ring_provider() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let mut provider = rustls::crypto::ring::default_provider();
        provider.kx_groups = vec![rustls::crypto::ring::kx_group::X25519];
        let _ = provider.install_default();
    });
}
