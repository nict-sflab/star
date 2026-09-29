use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};

use star_core::secure_channel::server::SecureChannelServer;
use star_core::serv::core::cred::Credential;

#[no_mangle]
pub static mut BINSEC_SECRET_TOKEN_KEY: [u8; 32] = [0x11; 32];
#[no_mangle]
pub static mut BINSEC_SECRET_HPKE_KEY: [u8; 32] = [0x22; 32];

#[no_mangle]
pub static mut BINSEC_PUBLIC_CREDENTIAL: [u8; 64] = [0; 64];
#[no_mangle]
pub static mut BINSEC_PUBLIC_COUNT: [u8; 8] = [0; 8];
#[no_mangle]
pub static mut BINSEC_PUBLIC_PERIOD: [u8; 8] = [0; 8];
#[no_mangle]
pub static mut BINSEC_PUBLIC_CURRENT_PERIOD: [u8; 8] = [0; 8];
#[no_mangle]
pub static mut BINSEC_PUBLIC_MAX_COUNT: [u8; 8] = [0xff; 8];

#[no_mangle]
pub static mut BINSEC_PUBLIC_ENC: [u8; 32] = [0; 32];
#[no_mangle]
pub static mut BINSEC_PUBLIC_INFO: [u8; 32] = [0; 32];
#[no_mangle]
pub static mut BINSEC_PUBLIC_AAD: [u8; 32] = [0; 32];
#[no_mangle]
pub static mut BINSEC_PUBLIC_CIPHERTEXT: [u8; 48] = [0; 48];

#[no_mangle]
pub static mut BINSEC_RESULT: u8 = 0;

#[no_mangle]
#[inline(never)]
pub extern "C" fn binsec_halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn binsec_check_credential() -> ! {
    let secret_key = unsafe { read_volatile(addr_of!(BINSEC_SECRET_TOKEN_KEY)) };
    let credential_bytes = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_CREDENTIAL)) };
    let count = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_COUNT)) };
    let period = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_PERIOD)) };
    let current_period = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_CURRENT_PERIOD)) };
    let max_count = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_MAX_COUNT)) };

    let credential = Credential::from_bytes(&credential_bytes);
    let valid = credential.is_valid(period, count, current_period, max_count, secret_key);

    unsafe {
        write_volatile(addr_of_mut!(BINSEC_RESULT), u8::from(valid));
    }
    binsec_halt()
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn binsec_check_hpke() -> ! {
    let secret_key = unsafe { read_volatile(addr_of!(BINSEC_SECRET_HPKE_KEY)) };
    let enc = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_ENC)) };
    let info = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_INFO)) };
    let aad = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_AAD)) };
    let ciphertext = unsafe { read_volatile(addr_of!(BINSEC_PUBLIC_CIPHERTEXT)) };

    let receiver = match SecureChannelServer::new(secret_key) {
        Ok(receiver) => receiver,
        Err(_) => {
            unsafe {
                write_volatile(addr_of_mut!(BINSEC_RESULT), 0xff);
            }
            binsec_halt()
        }
    };

    let result = receiver.receive_hpke(&enc, &info, &aad, &ciphertext);
    let ok = result.is_ok();
    core::mem::forget(result);

    unsafe {
        write_volatile(addr_of_mut!(BINSEC_RESULT), u8::from(ok));
    }
    binsec_halt()
}

fn main() {
    binsec_check_credential()
}
