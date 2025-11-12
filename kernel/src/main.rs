#![no_std]
#![no_main]

mod framebuffer;

use core::fmt::Write;
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use crate::framebuffer::FrameBufferWriter;

entry_point!(kernel_main);



pub fn kernel_main(boot_info: &'static mut BootInfo) -> !{
    //Refer to the documentation for more useful infos
    let _kernel_len = boot_info.kernel_len;
    let _kernel_addr  = boot_info.kernel_addr;

    let ( buffer,  frame_buffer_info) = {
        let framebuffer = boot_info.framebuffer.as_mut().unwrap();
        let info = framebuffer.info();
        (framebuffer.buffer_mut(),info)
    };
    let mut writer = FrameBufferWriter::new(buffer,frame_buffer_info);

   writer.clear();
    let _ =writer.write_str(" Hello, world!\n");



    loop {}
}



/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}