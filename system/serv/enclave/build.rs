use std::{env, fs, path::PathBuf};

const DEFAULT_MAX_COUNT: u32 = 100;
const STAR_MAX_COUNT: &str = "STAR_MAX_COUNT";

fn main() {
    println!("cargo:rerun-if-env-changed={STAR_MAX_COUNT}");
    let max_count = env::var_os(STAR_MAX_COUNT).map_or(DEFAULT_MAX_COUNT, |value| {
        value
            .to_str()
            .expect("STAR_MAX_COUNT must be UTF-8")
            .trim()
            .parse::<u32>()
            .expect("STAR_MAX_COUNT must be a decimal u32")
    });
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by cargo"));
    let config = format!("pub(crate) const MAX_COUNT: u32 = {max_count};\n");
    fs::write(out_dir.join("max_count.rs"), config).expect("write max count config");

    if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("sgx") {
        println!("cargo:rustc-link-arg=--allow-multiple-definition");
    }
}
