#![feature(abi_c_cmse_nonsecure_call, cmse_nonsecure_entry)]
#![no_std]
#![no_main]

mod mpc;
mod spcb;

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::peripheral::sau::{SauRegion, SauRegionAttribute};
use cortex_m::peripheral::scb::Exception;
use cortex_m::peripheral::Peripherals;
use cortex_m::cmse::{AccessType, TestTarget};
use cortex_m_semihosting::hprintln;
use cortex_m_rt::{entry, exception};
use mpc::Mpc;

#[entry]
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
    // Allow to call NSC functions
    sau.set_region(
        2,
        SauRegion {
            base_address: 0x10000000,
            limit_address: 0x101FFFFF | 0x1F,
            attribute: SauRegionAttribute::NonSecureCallable,
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
    // Enable secure fault
    let mut scb = peripherals.SCB;
    scb.enable(Exception::SecureFault);
    // Allows SAU to define the code region as a NSC
    spcb::enable_idau_nsc_code();
    unsafe {
        let ns_vector_table_addr = 0x00200000;
        // Write the Non-Secure Main Stack Pointer before switching state. Its value is the first
        // entry of the Non Secure Vector Table.
        cortex_m::register::msp::write_ns(*(ns_vector_table_addr as *const u32));
        // Create a Non-Secure function pointer to the address of the second entry of the Non
        // Secure Vector Table.
        let ns_reset_vector: extern "C-cmse-nonsecure-call" fn() -> ! =
            core::mem::transmute::<u32, _>(ns_vector_table_addr + 4);
        ns_reset_vector()
    }
}

#[no_mangle]
#[cmse_nonsecure_entry]
pub extern "C" fn secure_function() {
    hprintln!("secure function called!").unwrap();
}

#[no_mangle]
#[cmse_nonsecure_entry]
pub extern "C" fn secure_function_pointers(
    input: *const u8,
    input_length: usize,
    output: *mut u8,
    output_length: usize,
) -> u32 {
    hprintln!("secure function with pointers called!").unwrap();
    // Is NS allowed to read at input and write at signature and signature_length?
    let input_check =
        TestTarget::check_range(input as *mut u32, input_length,
                                AccessType::NonSecure).unwrap();
    let output_check =
        TestTarget::check_range(output as *mut u32, output_length,
                                AccessType::NonSecure).unwrap();
    if !input_check.ns_readable() || !output_check.ns_read_and_writable() {
        hprintln!("Permission denied").unwrap();
        1
    } else {
        hprintln!("Permission accepted").unwrap();
        // Deal with the operation...
        0
    }
}

#[no_mangle]
#[cmse_nonsecure_entry]
pub extern "C" fn secure_callback(callback: unsafe extern "C" fn()) {
    let callback: unsafe extern "C-cmse-nonsecure-call" fn() = unsafe {
        core::mem::transmute(callback)
    };
    unsafe {
        callback();
    }
}

#[allow(non_snake_case)]
#[exception]
fn SecureFault() {
    hprintln!("Secure Fault!!!").unwrap();
    loop {}
}
