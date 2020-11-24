#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn entry() {
    exit(0x69);
}

#[link(name="pintos", kind="static")]
extern {
    fn exit(status: i32);
}
