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
fn independent_shards_share_provisioning_state_and_isolate_failures() {
    let sgxs = crate::enclave::EnclaveRuntime::prepare_image().unwrap();
    let runtimes: Vec<_> = (0..2)
        .map(|_| LibraryRuntime::load(&sgxs).unwrap())
        .collect();
    let tokio = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let keys: Vec<_> = runtimes
        .iter()
        .map(|runtime| {
            tokio.block_on(async {
                runtime.call_provisioning(0, &[], 0).unwrap();
            });
            let key = runtime.call(5, &[], 32).unwrap();
            assert_eq!(key.len(), 32);
            assert_eq!(runtime.call_provisioning(5, &[], 32).unwrap(), key);
            key
        })
        .collect();
    assert_ne!(
        keys[0], keys[1],
        "each shard must own a separate enclave mapping"
    );
    std::thread::scope(|scope| {
        for (runtime, key) in runtimes.iter().zip(&keys) {
            scope.spawn(move || {
                for _ in 0..1000 {
                    assert_eq!(runtime.call(7, &[0; 200], 1).unwrap(), [1; 32]);
                    assert_eq!(&runtime.call(5, &[], 32).unwrap(), key);
                }
            });
        }
    });
    assert!(matches!(
        runtimes[0].call(99, &[], 0),
        Err(EnclaveError::Remote { .. })
    ));
    assert_eq!(runtimes[0].call(7, &[], 32).unwrap(), [1; 32]);
    // Attestation I/O is unsupported on the direct path. Fail closed for this
    // mapping, without replaying the operation or stopping another shard.
    assert!(matches!(
        runtimes[0].call(4, &[], 32),
        Err(EnclaveError::Library(_))
    ));
    assert!(runtimes[0].call(7, &[], 32).is_err());
    assert!(runtimes[0].call_provisioning(5, &[], 32).is_err());
    assert_eq!(runtimes[1].call(7, &[], 32).unwrap(), [1; 32]);
}
