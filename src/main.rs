#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static MSSG: &[u8] = b"HELLO WORLD";

#[unsafe(no_mangle)]
extern "C" fn _start()-> ! {
    let vga_buf = 0xb8000 as *mut u8;

    for (i, &byte) in MSSG.iter().enumerate() {
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}
