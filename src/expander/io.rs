//! Contains the implementation to make an [`Expander`] Sync.

use core::marker::PhantomData;

use hal::i2c::blocking::{Write, WriteRead};

use super::{Expander, ExpanderError, Register, SyncExpander};
use crate::ExpanderMutex;

/// A wrapper struct to make an Expander Sync.
/// This Expander type can be used to generate [`crate::ExpanderInputPin`] or [`crate::ExpanderOutputPin`].
pub struct IoExpander<I2C, Ex, Em>
where
    I2C: Write + WriteRead,
    Ex: Expander<I2C> + Send,
    Em: ExpanderMutex<Ex>,
{
    expander_mutex: Em,
    phantom_data: PhantomData<Ex>,
    phantom_data_2: PhantomData<I2C>,
}

impl<I2C, Em, Ex> IoExpander<I2C, Ex, Em>
where
    I2C: Write + WriteRead,
    Em: ExpanderMutex<Ex>,
    Ex: Expander<I2C> + Send,
{
    /// Creates a new IoExpander instance out of an Expander.
    pub fn new(expander: Ex) -> IoExpander<I2C, Ex, Em> {
        IoExpander {
            expander_mutex: Em::new(expander),
            phantom_data: PhantomData,
            phantom_data_2: PhantomData,
        }
    }
}

impl<I2C, Em, Ex> SyncExpander<I2C> for IoExpander<I2C, Ex, Em>
where
    I2C: Write + WriteRead,
    Em: ExpanderMutex<Ex>,
    Ex: Expander<I2C> + Send,
{
    fn write_byte(
        &self,
        register: Register,
        data: u8,
    ) -> Result<(), ExpanderError<<I2C as Write>::Error>> {
        self.expander_mutex.lock(|ex| ex.write_byte(register, data))
    }
    fn read_byte(
        &self,
        register: Register,
        buffer: &mut u8,
    ) -> Result<(), ExpanderError<<I2C as WriteRead>::Error>> {
        self.expander_mutex
            .lock(|ex| ex.read_byte(register, buffer))
    }
    fn write_halfword(
        &self,
        register: Register,
        data: u16,
    ) -> Result<(), ExpanderError<<I2C as Write>::Error>> {
        self.expander_mutex
            .lock(|ex| ex.write_halfword(register, data))
    }
    fn read_halfword(
        &self,
        register: Register,
        buffer: &mut u16,
    ) -> Result<(), ExpanderError<<I2C as WriteRead>::Error>> {
        self.expander_mutex
            .lock(|ex| ex.read_halfword(register, buffer))
    }
}
