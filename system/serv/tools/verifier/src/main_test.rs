use super::*;

#[test]
fn required_identity() {
    assert!(Command::try_parse_from(["star_tools", "verify"]).is_err());
    assert!(Command::try_parse_from(["star_tools", "verify", "--allow-debug"]).is_err());
    assert!(matches!(
        Command::try_parse_from(["star_tools", "inspect"]).unwrap(),
        Command::Inspect
    ));
}

#[test]
fn measurement_options() {
    let signer = "02:".repeat(32);
    let enclave = "01_".repeat(32);
    for (args, expected_signer, expected_enclave) in [
        (vec!["--mrsigner", signer.as_str()], Some([2; 32]), None),
        (vec!["--mrenclave", enclave.as_str()], None, Some([1; 32])),
        (
            vec!["--mrsigner", &signer, "--mrenclave", &enclave],
            Some([2; 32]),
            Some([1; 32]),
        ),
    ] {
        let command =
            Command::try_parse_from(["star_tools", "verify"].into_iter().chain(args)).unwrap();
        let Command::Verify {
            mrsigner,
            mrenclave,
            allow_debug,
            allow_advisory,
            ..
        } = command
        else {
            panic!("expected verify command");
        };
        assert_eq!(mrsigner, expected_signer.map(Measurement::new));
        assert_eq!(mrenclave, expected_enclave.map(Measurement::new));
        assert!(!allow_debug);
        assert!(!allow_advisory);
    }
}

#[test]
fn verification_options() {
    let command = Command::try_parse_from([
        "star_tools",
        "verify",
        &format!("--mrsigner={}", "02".repeat(32)),
        "--public-key",
        &"03".repeat(32),
        "--allow-debug",
        "--allow-advisory",
        "--pccs-url=http://127.0.0.1:1",
    ])
    .unwrap();
    let Command::Verify {
        public_key,
        allow_debug,
        allow_advisory,
        pccs_url,
        ..
    } = command
    else {
        panic!("expected verify command");
    };
    assert_eq!(public_key, Some([3; 32]));
    assert!(allow_debug);
    assert!(allow_advisory);
    assert_eq!(pccs_url.as_deref(), Some("http://127.0.0.1:1"));
}

#[test]
fn invalid_hex() {
    let signer = "02".repeat(32);
    for flag in ["--mrsigner", "--mrenclave", "--public-key"] {
        for value in ["01", &"gg".repeat(32), &format!("0x{signer}")] {
            let mut args = vec!["star_tools", "verify", flag, value];
            if flag == "--public-key" {
                args.extend(["--mrsigner", &signer]);
            }
            assert!(Command::try_parse_from(args).is_err());
        }
    }
}
