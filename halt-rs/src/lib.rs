#![no_std]

use libpintos::exit;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn entry() {
    exit(0x42);
}
