#![no_std]
#![no_main]

use core::panic::PanicInfo;

use io::vga_buffer;

pub mod io;

static HELLO_WORLD: &[u8] = b"Hello from my custom kernel!";

#[no_mangle]
pub extern "C" fn _start() {
    print_boot_message();

    loop {}
}

fn print_boot_message() {
    vga_buffer::print(HELLO_WORLD);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
