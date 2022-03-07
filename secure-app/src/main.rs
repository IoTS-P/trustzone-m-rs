#![feature(abi_c_cmse_nonsecure_call)]
#![no_std]
#![no_main]

mod mpc;

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::peripheral::sau::{SauRegion, SauRegionAttribute};
use cortex_m::peripheral::Peripherals;
use cortex_m_semihosting::hprintln;
use mpc::Mpc;

#[cortex_m_rt::entry]
fn main() -> ! {
    hprintln!("Hello from Secure World!").unwrap();
    let peripherals = Peripherals::take().unwrap();
    let mut sau = peripherals.SAU;
    // Non-Secure Flash area
    sau.set_region(
        0,
        SauRegion {
            base_address: 0x00200000,
            limit_address: 0x00400000 - 1,
            attribute: SauRegionAttribute::NonSecure,
        },
    ).unwrap();
    // Non-Secure RAM area
    sau.set_region(
        1,
        SauRegion {
            base_address: 0x28200000,
            limit_address: 0x28400000 - 1,
            attribute: SauRegionAttribute::NonSecure,
        },
    ).unwrap();
    sau.enable();
    // Code in SSRAM1
    let mut ssram1_mpc = Mpc::new(0x58007000, 0x00000000);
    ssram1_mpc.set_non_secure(0x00200000, 0x003F7FFF);
    // Secure data in SSRAM2, Non-Secure in SSRAM3
    let mut ssram3_mpc = Mpc::new(0x58009000, 0x28200000);
    ssram3_mpc.set_non_secure(0x28200000, 0x283F7FFF);
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    unsafe {
        let ns_vector_table_addr = 0x00200000;
        // Write the Non-Secure Main Stack Pointer before switching state. Its value is the first
        // entry of the Non Secure Vector Table.
        cortex_m::register::msp::write_ns(*(ns_vector_table_addr as *const u32));
        // Create a Non-Secure function pointer to the address of the second entry of the Non
        // Secure Vector Table.
        let ns_reset_vector: extern "C-cmse-nonsecure-call" fn() -> u32 =
            core::mem::transmute::<u32, _>(ns_vector_table_addr + 4);
        ns_reset_vector();
    }
    loop {
        // your code goes here
    }
}
