#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_semihosting::hprintln;

#[cortex_m_rt::entry]
fn main() -> ! {
    hprintln!("Hello from Non-Secure World!");
    // let secure_value = unsafe {
    //     *(0x38000000 as *const u32)
    // };
    // drop(secure_value);
    extern "C" {
        fn secure_function();
    }
    unsafe {
        secure_function();
    }
    extern "C" {
        fn secure_function_pointers(
            input: *const u8,
            input_length: usize,
            output: *mut u8,
            output_length: usize,
        ) -> u32;
    }
    let mut output: [u8; 32] = [0; 32];
    let _ans1 = unsafe {
        secure_function_pointers(
            0x28000000 as *const u8,
            16,
            output.as_mut_ptr(),
            output.len(),
        )
    };
    let ans2 =
        unsafe { secure_function_pointers(output.as_ptr(), 16, output.as_mut_ptr(), output.len()) };
    hprintln!("Return value: {}", ans2);
    extern "C" {
        fn secure_callback(callback: unsafe extern "C" fn());
    }
    unsafe {
        secure_callback(callback_function);
    }
    hprintln!("Program SUCCESS, exit Non-Secure World!");
    cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    unreachable!()
}

pub extern "C" fn callback_function() {
    hprintln!("Non-Secure World callback function called!");
}
