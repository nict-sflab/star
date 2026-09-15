#[cfg(target_env = "sgx")]
use sgx_isa::{AttributesFlags, Report};
#[cfg(target_env = "sgx")]
use star_attestation::Measurement;
use star_attestation::QuotePolicy;

#[cfg(target_env = "sgx")]
pub(crate) fn recipient_policy() -> Result<QuotePolicy, String> {
    let self_report = Report::for_self();
    let expected_mrenclave = option_env!("STAR_TRUSTED_ENCLAVE_MRENCLAVE")
        .map(|value| Measurement::parse_measurement_hex(value, "STAR_TRUSTED_ENCLAVE_MRENCLAVE"))
        .transpose()?;
    let mut expected_mrsigner = option_env!("STAR_TRUSTED_ENCLAVE_MRSIGNER")
        .map(|value| Measurement::parse_measurement_hex(value, "STAR_TRUSTED_ENCLAVE_MRSIGNER"))
        .transpose()?;
    if expected_mrenclave.is_none()
        && expected_mrsigner.is_none()
        && embedded_bool(
            option_env!("STAR_TRUST_SAME_SIGNER_ENCLAVE")
                .or(option_env!("STAR_TRUST_SAME_SIGNER_ENCLAVES")),
            false,
        )?
    {
        expected_mrsigner = Some(self_report.mrsigner.into());
    }
    if expected_mrenclave.is_none() && expected_mrsigner.is_none() {
        return Err(
            "a trusted recipient measurement or same-signer policy must be embedded".into(),
        );
    }
    Ok(QuotePolicy {
        expected_mrenclave,
        expected_mrsigner,
        allow_debug: embedded_bool(
            option_env!("STAR_ALLOW_DEBUG_ENCLAVE").or(option_env!("STAR_ALLOW_DEBUG_ENCLAVES")),
            self_report
                .attributes
                .flags
                .contains(AttributesFlags::DEBUG),
        )?,
        allow_advisory: embedded_bool(
            option_env!("STAR_ALLOW_ADVISORY_ENCLAVE")
                .or(option_env!("STAR_ALLOW_ADVISORY_ENCLAVES")),
            false,
        )?,
    })
}

#[cfg(not(target_env = "sgx"))]
pub(crate) fn recipient_policy() -> Result<QuotePolicy, String> {
    Err("recipient policy is only available inside SGX".into())
}

#[cfg(target_env = "sgx")]
fn embedded_bool(value: Option<&str>, default: bool) -> Result<bool, String> {
    match value {
        Some("1" | "true" | "TRUE" | "yes" | "YES") => Ok(true),
        Some("0" | "false" | "FALSE" | "no" | "NO") => Ok(false),
        Some(value) => Err(format!("invalid embedded policy boolean: {value}")),
        None => Ok(default),
    }
}
