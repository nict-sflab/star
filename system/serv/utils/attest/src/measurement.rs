use alloc::format;

use crate::{MEASUREMENT_LEN, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Measurement([u8; MEASUREMENT_LEN]);

impl Measurement {
    pub const fn new(bytes: [u8; MEASUREMENT_LEN]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; MEASUREMENT_LEN] {
        &self.0
    }

    pub const fn into_bytes(self) -> [u8; MEASUREMENT_LEN] {
        self.0
    }

    pub fn parse_measurement_hex(input: &str, name: &str) -> Result<Self> {
        fn hex_nibble(byte: u8) -> Option<u8> {
            match byte {
                b'0'..=b'9' => Some(byte - b'0'),
                b'a'..=b'f' => Some(byte - b'a' + 10),
                b'A'..=b'F' => Some(byte - b'A' + 10),
                _ => None,
            }
        }

        let mut bytes = [0u8; MEASUREMENT_LEN];
        let mut nibbles = 0usize;

        for byte in input.bytes() {
            if matches!(byte, b':' | b'_' | b' ' | b'\t' | b'\r' | b'\n') {
                continue;
            }
            if nibbles == MEASUREMENT_LEN * 2 {
                return Err(format!("{name} is longer than 32 bytes"));
            }
            let value = hex_nibble(byte).ok_or_else(|| format!("{name} contains non-hex data"))?;
            if nibbles % 2 == 0 {
                bytes[nibbles / 2] = value << 4;
            } else {
                bytes[nibbles / 2] |= value;
            }
            nibbles += 1;
        }

        if nibbles != MEASUREMENT_LEN * 2 {
            return Err(format!("{name} must be 32 bytes of hex"));
        }

        Ok(Self(bytes))
    }
}

impl From<[u8; MEASUREMENT_LEN]> for Measurement {
    fn from(bytes: [u8; MEASUREMENT_LEN]) -> Self {
        Self::new(bytes)
    }
}

impl AsRef<[u8]> for Measurement {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
