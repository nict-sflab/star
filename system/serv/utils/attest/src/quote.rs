use crate::{
    CHALLENGE_LEN, Measurement, PUBLIC_KEY_LEN, Result,
    codec::{read_array, read_u64_le},
    report::sgx_report_body_offset,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote {
    pub mrenclave: Measurement,
    pub mrsigner: Measurement,
    pub report_data: [u8; CHALLENGE_LEN],
    pub debug: bool,
}

impl Quote {
    pub fn parse(quote: &[u8]) -> Result<Self> {
        let report_body_offset = sgx_report_body_offset(quote)?;

        let attributes_offset = report_body_offset + 48;
        let mrenclave_offset = report_body_offset + 64;
        let mrsigner_offset = report_body_offset + 128;
        let report_data_offset = report_body_offset + 320;

        let attributes_flags = read_u64_le(quote, attributes_offset)?;

        Ok(Self {
            mrenclave: Measurement::new(read_array(quote, mrenclave_offset, "MRENCLAVE")?),
            mrsigner: Measurement::new(read_array(quote, mrsigner_offset, "MRSIGNER")?),
            report_data: read_array(quote, report_data_offset, "report_data")?,
            debug: attributes_flags & 0x2 != 0,
        })
    }

    pub fn report_data_for_public_key(public_key: [u8; PUBLIC_KEY_LEN]) -> [u8; CHALLENGE_LEN] {
        let mut report_data = [0u8; CHALLENGE_LEN];
        report_data[..PUBLIC_KEY_LEN].copy_from_slice(&public_key);
        report_data
    }

    pub fn public_key(&self) -> Result<[u8; PUBLIC_KEY_LEN]> {
        if self.report_data[PUBLIC_KEY_LEN..]
            .iter()
            .any(|byte| *byte != 0)
        {
            return Err("report_data is not public_key || zero padding".into());
        }

        self.report_data[..PUBLIC_KEY_LEN]
            .try_into()
            .map_err(|_| "report_data is too short for public key".into())
    }
}
