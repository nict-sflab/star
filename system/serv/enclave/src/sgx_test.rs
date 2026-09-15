use super::*;

// Stand in for the image bounds supplied by the SGX linker.
#[no_mangle]
static IMAGE_BASE: [u8; 64] = [0; 64];
#[export_name = "ENCLAVE_SIZE"]
static TEST_ENCLAVE_SIZE: usize = IMAGE_BASE.len();

#[test]
fn validates_host_buffer_bounds() {
    let base = IMAGE_BASE.as_ptr() as u64;
    assert!(UserBuffer::new(0, 0).is_some());
    assert!(UserBuffer::new(0, 1).is_none());
    assert!(UserBuffer::new(1, MAX_PAYLOAD_LEN as u64 + 1).is_none());
    assert!(UserBuffer::new(u64::MAX, 2).is_none());
    assert!(UserBuffer::new(base, 1).is_none());
    assert!(UserBuffer::new(base - 1, 2).is_none());
    assert!(UserBuffer::new(base + 63, 2).is_none());
    assert!(UserBuffer::new(base - 1, 1).is_some());
    assert!(UserBuffer::new(base + 64, 1).is_some());
}

#[test]
fn library_entry_copies_responses_and_reports_errors() {
    let mut output = [0; 128];
    let ptr = output.as_mut_ptr() as u64;
    assert_eq!(entry(0, 0, 0, 0, 0, 0), (OK, 0));
    assert_eq!(entry(5, 0, 0, 0, ptr, 1), (BUFFER_TOO_SMALL, 32));
    assert_eq!(entry(5, 0, 0, 0, ptr, 32), (OK, 32));
    assert_ne!(&output[..32], &[0; 32]);
    let (status, len) = entry(1, 0, 0, 0, ptr, output.len() as u64);
    assert_eq!(status, ERROR);
    assert_eq!(&output[..len as usize], b"enclave is not initialized");

    // The request and response may use the same host allocation.
    let (status, len) = entry(0, ptr, 1, 0, ptr, output.len() as u64);
    assert_eq!(status, ERROR);
    assert_eq!(
        &output[..len as usize],
        b"init does not accept keys; install a key-manager envelope"
    );
    assert_eq!(entry(7, 0, 0, 0, ptr, 32), (OK, 32));
    assert_eq!(&output[..32], &[1; 32]);
    assert_eq!(entry(7, 0, 1, 0, ptr, 32), (ERROR, 0));
    assert_eq!(
        entry(7, 0, 0, 0, IMAGE_BASE.as_ptr() as u64, 32),
        (ERROR, 0)
    );
}
