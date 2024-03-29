//! Contains the ExpanderMutex Trait to use an Expander accross threads.

/// Each type that can implement this trait can be used as a synchronization type for the [`crate::IoExpander`], which in turn is utilized to generate the [`hal`] pins. Due to this trait, the pins are synced and can be used across threads.
///
/// This trait can be implemented on all kinds of types, which ensure exclusive access to the contained data. For `std` environments, this trait is already implemented. It can be enabled by enabling the "std" feature of this library.
pub trait ExpanderMutex<Ex>
where
    Ex: Send,
{
    fn lock<R, C: FnOnce(&mut Ex) -> R>(&self, c: C) -> R;

    fn new(ex: Ex) -> Self;
}

#[cfg(feature = "std")]
impl<Ex> ExpanderMutex<Ex> for std::sync::Mutex<Ex>
where
    Ex: Send,
{
    fn lock<R, C: FnOnce(&mut Ex) -> R>(&self, c: C) -> R {
        let mut expander = self.lock().unwrap();
        c(&mut expander)
    }

    fn new(ex: Ex) -> Self {
        std::sync::Mutex::new(ex)
    }
}
