#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Integrate modules
mod vga_buffer;

// Imports
use core::panic::PanicInfo;

// Start function
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!"); // Print using VGA Buffer

    // OS Loop
    loop {}
}

// Called On Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
