use crate::traits::Crate;

/// A safe, untyped, heap-allocated `Crate` with an associated format version.
///
/// Note that dropping this type without calling [`UntypedCrate::into_crate()`] will result in a
/// memory leak.
pub struct UntypedCrate {
    crate_: *mut (),
    format_version: u32,
}

impl UntypedCrate {
    /// Creates a new [`UntypedCrate`] from a `Crate`.
    pub fn new<T: Crate>(crate_: T) -> Self {
        let crate_ = Box::new(crate_);

        Self {
            crate_: Box::into_raw(crate_).cast::<()>(),
            format_version: T::FORMAT_VERSION,
        }
    }

    /// Downcasts an [`UntypedCrate`] into a concrete `Crate`.
    ///
    /// # Panics
    ///
    /// If the format version of `Crate` is not equal to the format version of the
    /// [`UntypedCrate`].
    pub fn into_crate<T: Crate>(self) -> Box<T> {
        assert_eq!(
            self.format_version,
            T::FORMAT_VERSION,
            "attempted to convert an untyped `Crate` with format version {} to typed `Crate` with format version {}",
            self.format_version,
            T::FORMAT_VERSION,
        );

        unsafe { Box::from_raw(self.crate_.cast::<T>()) }
    }
}
