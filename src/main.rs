#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points


// Handle things called on panic
use core::panic::PanicInfo;


#[panic_handler] // don't mangle the name of this function
fn panic(_info: &PanicInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {} // An infinite loop
}


// This function is called on panic
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {} // An infinite loop
}

//Next: https://os.phil-opp.com/minimal-rust-kernel/