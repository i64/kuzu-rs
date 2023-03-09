#[repr(C)]
pub struct Opaque<const SIZE: usize>(
    pub [u8; SIZE],
    ::std::marker::PhantomData<(*mut u8, ::std::marker::PhantomPinned)>,
);

pub type std_string = Opaque<32>;
pub type std_mutex = Opaque<40>;
