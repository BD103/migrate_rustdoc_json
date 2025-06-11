use std::any::Any;

/// A safe, untyped, heap-allocated `Crate`.
pub struct UntypedCrate(Box<dyn Any>);

impl UntypedCrate {
    /// Creates a new [`UntypedCrate`] from a `Crate`.
    pub fn new<T: Any + 'static>(crate_: T) -> Self {
        let crate_ = Box::new(crate_);
        Self(crate_)
    }

    /// Downcasts an [`UntypedCrate`] into a concrete `Crate`.
    ///
    /// # Panics
    ///
    /// If the format version of `Crate` is not equal to the format version of the
    /// [`UntypedCrate`].
    pub fn into_crate<T: 'static>(self) -> Box<T> {
        self.0.downcast().unwrap()
    }
}
