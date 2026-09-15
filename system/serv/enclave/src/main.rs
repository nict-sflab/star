#[cfg(any(target_env = "sgx", test))]
mod protocol;
#[cfg(any(target_env = "sgx", test))]
mod sgx;

fn main() {}
