#![allow(unused)]
use core::{ptr, slice};

const LEN: usize = 32;
const FUNC_ECHO: u64 = 0;


#[no_mangle]
pub extern "C" fn entry(p1: u64, p2: u64, p3: u64, p4: u64, _p5: u64, _p6: u64) -> u64 {
    if p4 != LEN as u64 {
        return 1; // bad length
    }
    if p1 != FUNC_ECHO {
        return 2; // unknown func
    }

    unsafe {
        let in_ptr = p2 as *const u8;
        let out_ptr = p3 as *mut u8;

        let input = slice::from_raw_parts(in_ptr, LEN);
        let output = slice::from_raw_parts_mut(out_ptr, LEN);
        output.copy_from_slice(input);
    }

    0
}

fn main() {}
