#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo, info::FrameBufferInfo};

entry_point!(kernel_main);



pub fn kernel_main(boot_info: &'static mut BootInfo) -> !{
    //Refer to the documentation for more useful infos
    let _kernel_len = boot_info.kernel_len;
    let _kernel_addr  = boot_info.kernel_addr;

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
       let _info = framebuffer.info();
        let mut buffer = framebuffer.buffer_mut();

        //As you wish then !
    }


    loop {}
}

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}