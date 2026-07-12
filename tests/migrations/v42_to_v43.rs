#[repr(Rust)]
pub struct ReprRust {
    a: u8,
    b: u64,
}

#[repr(C)]
pub struct ReprC {
    a: u8,
    b: u64,
}

#[repr(packed)]
pub struct ReprPacked1 {
    a: u8,
    b: u64,
}

#[repr(packed(2))]
pub struct ReprPacked2 {
    a: u8,
    b: u64,
}

#[repr(C, align(8))]
pub struct ReprCAlign8 {
    a: u8,
    b: u64,
}

#[repr(i8)]
pub enum ReprI8 {
    First,
    Second,
}

#[repr(usize, C)]
pub enum ReprUsizeC {
    First,
    Second,
}

#[repr(transparent)]
pub struct TransparentPub(pub i64);

#[repr(transparent)]
pub struct TransparentPriv(i64);
