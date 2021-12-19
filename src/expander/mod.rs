use core::fmt::Debug;
use hal::i2c::blocking::{Write, WriteRead};

use super::{GPIOBank, Register};

pub mod cached;
pub mod immediate;
pub mod io;
pub mod standard;

/// Trait for standard IO expanders which are not Sync
pub trait Expander {
    type Error: core::fmt::Debug;
    fn write_byte(&mut self, register: Register, data: u8) -> Result<(), Self::Error>;
    fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), Self::Error>;
    fn write_halfword(&mut self, register: Register, data: u16) -> Result<(), Self::Error>;
    fn read_halfword(&mut self, register: Register, buffer: &mut u16) -> Result<(), Self::Error>;
}

/// Trait for IO expanders which use some synchronization primitive for the writes and reads. This implementation makes the expander sync and usable accross threads etc.
pub trait SyncExpander {
    type Error: core::fmt::Debug;
    fn write_byte(&self, register: Register, data: u8) -> Result<(), Self::Error>;
    fn read_byte(&self, register: Register, buffer: &mut u8) -> Result<(), Self::Error>;
    fn write_halfword(&self, register: Register, data: u16) -> Result<(), Self::Error>;
    fn read_halfword(&self, register: Register, buffer: &mut u16) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum ExpanderError<I2C>
where
    I2C: Write + WriteRead,
{
    WriteError(<I2C as Write>::Error),
    WriteReadError(<I2C as WriteRead>::Error),
}
