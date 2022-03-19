//! Contains all available Expander interfaces and traits.

use core::fmt::Debug;
use hal::i2c::blocking::{Write, WriteRead};

use super::{GPIOBank, Register};

pub mod cached;
pub mod immediate;
pub mod io;
pub mod standard;

/// Trait for standard IO expanders which are not Sync
pub trait Expander<I2C: Write + WriteRead> {
    fn write_byte(&mut self, register: Register, data: u8) -> Result<(), ExpanderError<I2C>>;
    fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), ExpanderError<I2C>>;
    fn write_halfword(&mut self, register: Register, data: u16) -> Result<(), ExpanderError<I2C>>;
    fn read_halfword(
        &mut self,
        register: Register,
        buffer: &mut u16,
    ) -> Result<(), ExpanderError<I2C>>;
}

/// Trait for IO expanders which use some synchronization primitive for the writes and reads. This implementation makes the expander sync and usable accross threads etc.
pub trait SyncExpander<I2C: Write + WriteRead> {
    fn write_byte(&self, register: Register, data: u8) -> Result<(), ExpanderError<I2C>>;
    fn read_byte(&self, register: Register, buffer: &mut u8) -> Result<(), ExpanderError<I2C>>;
    fn write_halfword(&self, register: Register, data: u16) -> Result<(), ExpanderError<I2C>>;
    fn read_halfword(&self, register: Register, buffer: &mut u16)
        -> Result<(), ExpanderError<I2C>>;
}

#[derive(Debug)]
pub enum ExpanderError<I2C>
where
    I2C: Write + WriteRead,
{
    WriteError(<I2C as Write>::Error),
    WriteReadError(<I2C as WriteRead>::Error),
}

#[cfg(feature = "std")]
impl<T: core::fmt::Debug + Write + WriteRead> std::fmt::Display for ExpanderError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self)
    }
}

#[cfg(feature = "std")]
impl<I2C: Write + WriteRead + Debug> std::error::Error for ExpanderError<I2C> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
