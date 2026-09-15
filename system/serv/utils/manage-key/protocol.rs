//! Length-delimited messages shared by the host and executable enclave.
use std::io::{self, Read, Write};

pub const INIT: u8 = 0;
pub const PUBLIC_KEY: u8 = 1;
pub const ATTEST: u8 = 4;
pub const EXPORT_KEYS: u8 = 5;
pub const OK: u8 = 0;
pub const ERROR: u8 = 2;
const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

pub fn read(reader: &mut impl Read) -> io::Result<Option<(u8, Vec<u8>)>> {
    let mut tag = [0];
    match reader.read_exact(&mut tag) {
        Ok(()) => {}
        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(err) => return Err(err),
    }
    let mut len = [0; 4];
    reader.read_exact(&mut len)?;
    let len = u32::from_le_bytes(len) as usize;
    check_len(len)?;
    let mut payload = vec![0; len];
    reader.read_exact(&mut payload)?;
    Ok(Some((tag[0], payload)))
}

pub fn write(writer: &mut impl Write, tag: u8, payload: &[u8]) -> io::Result<()> {
    check_len(payload.len())?;
    writer.write_all(&[tag])?;
    writer.write_all(&(payload.len() as u32).to_le_bytes())?;
    writer.write_all(payload)?;
    writer.flush()
}

fn check_len(len: usize) -> io::Result<()> {
    if len > MAX_PAYLOAD_LEN {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "key-manager message too large",
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_boundaries() {
        let mut bytes = Vec::new();
        write(&mut bytes, INIT, &[]).unwrap();
        write(&mut bytes, EXPORT_KEYS, &[0, 255, 42]).unwrap();
        let mut reader = bytes.as_slice();
        assert_eq!(read(&mut reader).unwrap(), Some((INIT, vec![])));
        assert_eq!(
            read(&mut reader).unwrap(),
            Some((EXPORT_KEYS, vec![0, 255, 42]))
        );
        assert_eq!(read(&mut reader).unwrap(), None);
    }

    #[test]
    fn invalid_frames() {
        for bytes in [&[INIT][..], &[INIT, 2, 0, 0, 0, 1][..]] {
            assert_eq!(
                read(&mut &*bytes).unwrap_err().kind(),
                io::ErrorKind::UnexpectedEof
            );
        }
        let mut bytes = vec![INIT];
        bytes.extend_from_slice(&((MAX_PAYLOAD_LEN + 1) as u32).to_le_bytes());
        assert_eq!(
            read(&mut bytes.as_slice()).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}
