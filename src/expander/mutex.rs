pub trait ExpanderMutex<Ex> {
    fn lock<R, C: FnOnce(&mut Ex) -> R>(&self, c: C) -> R;

    fn new(ex: Ex) -> Self;
}

#[cfg(feature = "std")]
impl<Ex> ExpanderMutex<Ex> for std::sync::Mutex<Ex> {
    fn lock<R, C: FnOnce(&mut Ex) -> R>(&self, c: C) -> R {
        let mut expander = self.lock().unwrap();
        c(&mut expander)
    }

    fn new(ex: Ex) -> Self {
        std::sync::Mutex::new(ex)
    }
}
