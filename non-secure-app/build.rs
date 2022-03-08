// Modified from cortex-m-quickstart

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/* ARM AN505 System */
const AN505_MEMORY_NON_SECURE: &'static [u8] = b"
MEMORY
{
    FLASH : ORIGIN = 0x00200000, LENGTH = 2M
    RAM : ORIGIN = 0x28200000, LENGTH = 2M
}";

fn main() {
    // Put memory configuration in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(AN505_MEMORY_NON_SECURE)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
