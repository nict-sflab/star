use super::*;

#[test]
fn response_buffers_retry_and_preserve_remote_errors() {
    let mut calls = 0;
    let output = call_library(7, b"request", 1, |args| {
        calls += 1;
        assert_eq!(args[0], 7);
        assert_eq!(args[2], 7);
        if args[4] < 32 {
            return Ok((STATUS_BUFFER_TOO_SMALL, 32));
        }
        // The helper supplies a live writable output of the advertised length.
        unsafe { std::ptr::write_bytes(args[3] as *mut u8, 1, 32) };
        Ok((STATUS_OK, 32))
    })
    .unwrap();
    assert_eq!(output, [1; 32]);
    assert_eq!(calls, 2);

    let error = call_library(99, &[], 4, |args| {
        unsafe { std::ptr::copy_nonoverlapping(b"fail".as_ptr(), args[3] as *mut u8, 4) };
        Ok((2, 4))
    })
    .unwrap_err();
    assert!(matches!(error, EnclaveError::Remote { status: 2, payload } if payload == b"fail"));
}

#[test]
fn invalid_response_lengths_do_not_retry() {
    for (status, len) in [
        (STATUS_OK, 33),
        (STATUS_BUFFER_TOO_SMALL, 32),
        (STATUS_BUFFER_TOO_SMALL, MAX_PAYLOAD_LEN as u64 + 1),
    ] {
        let mut calls = 0;
        let error = call_library(7, &[], 32, |_| {
            calls += 1;
            Ok((status, len))
        })
        .unwrap_err();
        assert!(matches!(error, EnclaveError::ResponseTooLarge { .. }));
        assert_eq!(calls, 1);
    }
}

#[test]
#[ignore = "requires SGX hardware and the Fortanix SGX build tools"]
fn direct_calls_share_state_with_provisioning_and_support_contention() {
    let sgxs = std::env::var_os("STAR_ENCLAVE_SGXS")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            build_sgxs(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../enclave/Cargo.toml"),
                "enclave",
                "33554432".into(),
                "131072".into(),
                "5".into(),
            )
            .unwrap()
        });
    for workers in [1, 4] {
        let runtime = LibraryRuntime::load(&sgxs, workers).unwrap();
        let tokio = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        tokio.block_on(async {
            runtime.call_provisioning(0, &[], 0).unwrap();
            assert_eq!(runtime.call(7, &[0; 200], 1).unwrap(), [1; 32]);
        });
        let key = runtime.call(5, &[], 32).unwrap();
        assert_eq!(key.len(), 32);
        assert_eq!(runtime.call_provisioning(5, &[], 32).unwrap(), key);
        let barrier = std::sync::Barrier::new(9);
        std::thread::scope(|scope| {
            for _ in 0..8 {
                let (runtime, key, barrier) = (&runtime, &key, &barrier);
                scope.spawn(move || {
                    barrier.wait();
                    for _ in 0..1000 {
                        assert_eq!(runtime.call(7, &[0; 200], 32).unwrap(), [1; 32]);
                        assert_eq!(&runtime.call(5, &[], 32).unwrap(), key);
                    }
                });
            }
            barrier.wait();
            assert_eq!(runtime.call_provisioning(5, &[], 32).unwrap(), key);
        });
        // An application error must not stop the caller or lose its TCS.
        assert!(matches!(
            runtime.call(99, &[], 0),
            Err(EnclaveError::Remote { .. })
        ));
        assert_eq!(runtime.call(7, &[], 32).unwrap(), [1; 32]);
        // Attestation requires I/O, which is deliberately unsupported on the
        // direct path. A usercall failure must stop every TCS, without replay.
        assert!(matches!(
            runtime.call(4, &[], 32),
            Err(EnclaveError::Library(_))
        ));
        assert!(runtime.call(7, &[], 32).is_err());
        assert!(runtime.call_provisioning(5, &[], 32).is_err());
    }
}
