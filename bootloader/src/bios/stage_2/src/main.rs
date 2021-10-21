#![no_std]
#![no_main]
#![feature(asm)]

mod mode_switch;
mod display;
mod img_load;
mod a20;

use core::{marker::PhantomData, panic::PanicInfo, mem::transmute};
use a20::check_a20;
use display::display_real;
use img_load::{STAGE3_PTR, load_stage3};
use mode_switch::to_protect;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() -> Result<(), &'static str> {
    display_real("Stage 2 entered.");
    load_stage3()?;
    display_real("Stage 3 loaded.");

    if check_a20() {
        display_real("A20 enabled")
    }

    to_protect()?;
    Ok(())
}

/// Our entrypoiny of bootloader.
/// The loader will be loaded to 0x7c00 by BIOS, which has been considered by our linker script
#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    if let Err(msg) = main() {
        display_real(msg);
        unsafe { asm!("hlt;") };
    }
    
    let stage_3: fn() -> ! = unsafe { transmute(&STAGE3_PTR as *const PhantomData<()>) };
    stage_3()
}
