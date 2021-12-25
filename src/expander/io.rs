use core::marker::PhantomData;

use super::{Register, SyncExpander};

use super::Expander;
use crate::ExpanderMutex;

/// A wrapper struct to make an Expander Sync.
/// This Expander type can be used to generate [`crate::ExpanderInputPin`] or [`crate::ExpanderOutputPin`].
pub struct IoExpander<Em, Ex>
where
    Ex: Send,
    Em: ExpanderMutex<Ex>,
{
    expander_mutex: Em,
    phantom_data: PhantomData<Ex>,
}

impl<Em: ExpanderMutex<Ex>, Ex: Expander + Send> IoExpander<Em, Ex> {
    /// Creates a new IoExpander instance out of an Expander.
    pub fn new(expander: Ex) -> IoExpander<Em, Ex> {
        IoExpander {
            expander_mutex: Em::new(expander),
            phantom_data: PhantomData,
        }
    }
}

impl<Em: ExpanderMutex<Ex>, Ex: Expander + Send> SyncExpander for IoExpander<Em, Ex> {
    type Error = <Ex as Expander>::Error;

    fn write_byte(&self, register: Register, data: u8) -> Result<(), Self::Error> {
        self.expander_mutex.lock(|ex| ex.write_byte(register, data))
    }
    fn read_byte(&self, register: Register, buffer: &mut u8) -> Result<(), Self::Error> {
        self.expander_mutex
            .lock(|ex| ex.read_byte(register, buffer))
    }
    fn write_halfword(&self, register: Register, data: u16) -> Result<(), Self::Error> {
        self.expander_mutex
            .lock(|ex| ex.write_halfword(register, data))
    }
    fn read_halfword(&self, register: Register, buffer: &mut u16) -> Result<(), Self::Error> {
        self.expander_mutex
            .lock(|ex| ex.read_halfword(register, buffer))
    }
}
