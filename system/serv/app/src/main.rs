use enclave_runner::EnclaveBuilder;
use sgxs_loaders::isgx::Device;
use std::path::Path;

const LEN: usize = 32;
const FUNC_BYTES: u64 = 1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = EnclaveBuilder::new(Path::new("../enclave/target/x86_64-fortanix-unknown-sgx/debug/enclave.sgxs"));
    builder.dummy_signature();
    let mut device = Device::new()?.build();
    let enclave = builder.build_library(&mut device)?;
    
    let mut input = [0u8; LEN];
    for (i, b) in input.iter_mut().enumerate() {
        *b = i as u8;
    }
    let mut output = [0u8; LEN];

    let (status, _ret2) = unsafe {
        enclave.call(
            FUNC_BYTES,
            input.as_ptr() as u64,
            output.as_mut_ptr() as u64,
            LEN as u64,
            0,
        )?
    };

    if status != 0 {
        return Err(format!("enclave returned error status={status}").into());
    }

    println!("in : {:02x?}", input);
    println!("out: {:02x?}", output);
    Ok(())
}
