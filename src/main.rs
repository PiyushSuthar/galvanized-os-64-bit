#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Imports
use core::panic::PanicInfo;

// Static variables
static HELLO: &[u8] = b"Hello World!";

// Start function
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // Get the VGA Buffer

    // Print the string.
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    // OS Loop
    loop {}
}

// Called On Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
