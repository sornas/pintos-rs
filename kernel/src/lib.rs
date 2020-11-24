#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn syscall_handler() {
    printf("c suger lmao xd\n".as_ptr());
    thread_exit();
}

extern {
    fn thread_exit();
    fn printf(format: *const u8, ...);
}
