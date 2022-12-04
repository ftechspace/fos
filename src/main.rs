#![no_std] //telling rust not to use the standard lib (eg, println!)
#![no_main] //telling rust compiler  we don't want to use the normal entry point chain. 
            //we will rewrite the crtO entry point directly

/// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// end of function

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}