#![no_std]
#![no_main]

use panic_halt as _; // put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_semihosting::hprintln;

// use all secure world functions
extern "C" {
    fn secure_function();
    fn secure_function_pointers(
        input: *const u8,
        input_length: usize,
        output: *mut u8,
        output_length: usize,
    ) -> u32;
    fn secure_callback(callback: unsafe extern "C" fn());
}

#[cortex_m_rt::entry]
fn main() -> ! {
    hprintln!("Hello from Non-Secure World!");
    unsafe {
        secure_function();
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
    unsafe {
        secure_callback(callback_function);
    }
    // let secure_value = unsafe {
    //     *(0x38000000 as *const u32)
    // };
    // drop(secure_value); // if uncomment, will raise SecureFault
    hprintln!("Program SUCCESS, exit Non-Secure World!");
    cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    unreachable!()
}

pub extern "C" fn callback_function() {
    hprintln!("Non-Secure World callback function called!");
}
