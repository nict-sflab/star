use std::io::{self, ErrorKind, Read, Write};

pub const MAGIC: &[u8; 8] = b"STARIPC1";
pub const OP_INIT: u8 = 0;
pub const OP_PUBLIC_KEY: u8 = 1;
pub const OP_ISSUE_CRED: u8 = 2;
pub const OP_ISSUE_TOKEN: u8 = 3;
pub const OP_ATTEST: u8 = 4;

pub const STATUS_OK: u8 = 0;
pub const STATUS_NOT_INITIALIZED: u8 = 1;
pub const STATUS_BAD_REQUEST: u8 = 2;
pub const STATUS_ENCLAVE_ERROR: u8 = 3;
pub const STATUS_PANIC: u8 = 4;

pub const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub op: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: u8,
    pub payload: Vec<u8>,
}

pub fn read_request(input: &mut impl Read) -> io::Result<Option<Request>> {
    let mut magic = [0u8; MAGIC.len()];
    match input.read_exact(&mut magic) {
        Ok(()) => {}
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => return Ok(None),
        Err(err) => return Err(err),
    }

    if &magic != MAGIC {
        return Err(io::Error::new(ErrorKind::InvalidData, "bad request magic"));
    }

    let mut op = [0u8; 1];
    input.read_exact(&mut op)?;

    let payload = read_payload(input)?;
    Ok(Some(Request { op: op[0], payload }))
}

pub fn write_request(output: &mut impl Write, op: u8, payload: &[u8]) -> io::Result<()> {
    output.write_all(MAGIC)?;
    output.write_all(&[op])?;
    write_payload(output, payload)?;
    output.flush()
}

pub fn read_response(input: &mut impl Read) -> io::Result<Response> {
    let mut status = [0u8; 1];
    input.read_exact(&mut status)?;

    let payload = read_payload(input)?;
    Ok(Response {
        status: status[0],
        payload,
    })
}

pub fn write_response(output: &mut impl Write, status: u8, payload: &[u8]) -> io::Result<()> {
    output.write_all(&[status])?;
    write_payload(output, payload)?;
    output.flush()
}

fn read_payload(input: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut len = [0u8; 4];
    input.read_exact(&mut len)?;
    let len = u32::from_be_bytes(len) as usize;
    if len > MAX_PAYLOAD_LEN {
        return Err(io::Error::new(ErrorKind::InvalidData, "payload too large"));
    }

    let mut payload = vec![0u8; len];
    input.read_exact(&mut payload)?;
    Ok(payload)
}

fn write_payload(output: &mut impl Write, payload: &[u8]) -> io::Result<()> {
    if payload.len() > MAX_PAYLOAD_LEN {
        return Err(io::Error::new(ErrorKind::InvalidInput, "payload too large"));
    }

    output.write_all(&(payload.len() as u32).to_be_bytes())?;
    output.write_all(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn request_round_trip() {
        let mut bytes = Vec::new();
        write_request(&mut bytes, OP_ISSUE_CRED, b"cipher").expect("write request");

        let mut cursor = Cursor::new(bytes);
        let request = read_request(&mut cursor)
            .expect("read request")
            .expect("request");

        assert_eq!(request.op, OP_ISSUE_CRED);
        assert_eq!(request.payload, b"cipher");
    }

    #[test]
    fn response_round_trip() {
        let mut bytes = Vec::new();
        write_response(&mut bytes, STATUS_ENCLAVE_ERROR, &[7]).expect("write response");

        let mut cursor = Cursor::new(bytes);
        let response = read_response(&mut cursor).expect("read response");

        assert_eq!(response.status, STATUS_ENCLAVE_ERROR);
        assert_eq!(response.payload, vec![7]);
    }

    #[test]
    fn empty_request_stream_is_none() {
        let mut cursor = Cursor::new(Vec::new());
        assert_eq!(read_request(&mut cursor).expect("read request"), None);
    }

    #[test]
    fn bad_request_magic_is_invalid_data() {
        let mut cursor = Cursor::new(*b"NOTSTAR1");
        let err = read_request(&mut cursor).expect_err("bad magic");
        assert_eq!(err.kind(), ErrorKind::InvalidData);
    }
}
