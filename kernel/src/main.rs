#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo, info::FrameBufferInfo};

entry_point!(kernel_main);

static HELLO: &[u8] = b"Hello World!";


pub fn kernel_main(boot_info: &'static mut BootInfo) -> !{
    //Refer to the documentation for more useful infos
    let _kernel_len = boot_info.kernel_len;
    let _kernel_addr  = boot_info.kernel_addr;

    if let Some(_framebuffer) = boot_info.framebuffer.as_mut() {
       // Do your magic
    }


    loop {}
}

use core::panic::PanicInfo;
use bootloader_api::info::{FrameBuffer, PixelFormat};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}