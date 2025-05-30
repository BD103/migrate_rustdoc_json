#[inline]
pub fn foo() {}

#[derive(Default)]
pub enum Bar {
    #[default]
    Baz,
    Frob,
}
