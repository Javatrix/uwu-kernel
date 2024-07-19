#![no_std]
#![no_main]

use core::panic::PanicInfo;
use io::vga_buffer;

pub mod io;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("YAY, custom {} of values: {}!", "printing", 43.0 / 24.0);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
