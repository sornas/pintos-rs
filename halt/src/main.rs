#![no_std]
#![no_main]
#![feature(start)]
//#![feature(asm)]
//#![link_args = "-L -lmain"]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    exit(0x69);
}

#[link(name="pintos", kind="static")]
extern {
    fn exit(status: i32);
}

//#[no_mangle]
//#[start]
//unsafe extern "C" fn _start() {
//    exit(main());
//}
