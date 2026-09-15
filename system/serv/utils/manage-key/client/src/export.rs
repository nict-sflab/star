#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyManagerExport {
    pub key_manager_quote: Vec<u8>,
    pub recipient_evidence: Vec<u8>,
    pub envelope: Vec<u8>,
}

impl KeyManagerExport {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for blob in [
            &self.key_manager_quote,
            &self.recipient_evidence,
            &self.envelope,
        ] {
            out.extend_from_slice(&(blob.len() as u32).to_be_bytes());
            out.extend_from_slice(blob);
        }
        out
    }

    pub fn decode(mut payload: &[u8]) -> Result<Self, String> {
        let mut blobs = [Vec::new(), Vec::new(), Vec::new()];
        for blob in &mut blobs {
            let length = payload.get(..4).ok_or("missing blob length")?;
            let len = u32::from_be_bytes(length.try_into().unwrap()) as usize;
            payload = &payload[4..];
            *blob = payload.get(..len).ok_or("truncated blob")?.to_vec();
            payload = &payload[len..];
        }
        if !payload.is_empty() {
            return Err("trailing bytes after key-manager export response".into());
        }
        let [key_manager_quote, recipient_evidence, envelope] = blobs;
        Ok(Self {
            key_manager_quote,
            recipient_evidence,
            envelope,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_encoding() {
        let export = KeyManagerExport {
            key_manager_quote: vec![1, 2],
            recipient_evidence: vec![3],
            envelope: vec![4, 5],
        };
        let encoded = export.encode();
        assert_eq!(KeyManagerExport::decode(&encoded).unwrap(), export);
        for end in 0..encoded.len() {
            assert!(KeyManagerExport::decode(&encoded[..end]).is_err());
        }
        let mut trailing = encoded;
        trailing.push(0);
        assert!(KeyManagerExport::decode(&trailing).is_err());
    }
}
