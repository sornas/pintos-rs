#![no_std]

#[link(name="pintos", kind="static")]
extern {
    pub fn exit(status: i32);
}
