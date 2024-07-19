#![no_std]
#![no_main]

use core::panic::PanicInfo;

use io::vga_buffer::{self, num_to_ascii, print};

pub mod io;
pub mod utils;

static HELLO_WORLD: &[u8] = b"Hello from my custom kernel!";

#[no_mangle]
pub extern "C" fn _start() {
    print_boot_message();

    let message: &mut [u8; 10] = &mut [0; 10];
    for i in 0..10 {
        message[i] = num_to_ascii(i as u8);
    }
    print(message);

    loop {}
}

fn print_boot_message() {
    vga_buffer::print(HELLO_WORLD);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
