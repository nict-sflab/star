use core::ptr::{addr_of, read_volatile};

use star_core::serv::enclave::Enclave;

const HPKE_ENVELOPE_LEN: usize = 32 + 32 + 32 + 96;

#[no_mangle]
pub static mut BINSEC_SECRET_TOKEN_KEY: [u8; 32] = [0x11; 32];
#[no_mangle]
pub static mut BINSEC_SECRET_HPKE_KEY: [u8; 32] = [0x22; 32];

#[no_mangle]
pub static mut BINSEC_PUBLIC_ENVELOPE: [u8; HPKE_ENVELOPE_LEN] = [0; HPKE_ENVELOPE_LEN];
#[no_mangle]
pub static mut BINSEC_PUBLIC_CURRENT_PERIOD: [u8; 8] = [0; 8];
#[no_mangle]
pub static mut BINSEC_PUBLIC_MAX_TICKETS: u32 = 3;

#[no_mangle]
#[inline(never)]
pub extern "C" fn binsec_halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn binsec_check_issue_token() -> ! {
    let token_key = unsafe { read_volatile(addr_of!(BINSEC_SECRET_TOKEN_KEY)) };
    let hpke_key = unsafe { read_volatile(addr_of!(BINSEC_SECRET_HPKE_KEY)) };
    let envelope = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_ENVELOPE)) };
    let current_period = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_CURRENT_PERIOD)) };
    let max_tickets = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_MAX_TICKETS)) };

    let enclave = Enclave::new(max_tickets, token_key, hpke_key)
        .expect("fixed-size X25519 secret key must be accepted");

    let result = enclave.issue_token(&envelope, current_period);

    // Keep the full request path alive without adding a harness-level branch on
    // the result. Binsec follows dependencies from the two secret keys through
    // HPKE decryption into credential/count/period processing.
    core::hint::black_box(&result);
    core::mem::forget(result);

    binsec_halt()
}

fn main() {
    binsec_check_issue_token()
}
