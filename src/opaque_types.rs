#[repr(C)]
pub struct Opaque<const SIZE: usize>(pub [u8; SIZE]);

pub type std_string = Opaque<32>;
pub type std_mutex = Opaque<40>;
