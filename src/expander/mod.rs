//! Contains all available Expander interfaces and traits.
use core::fmt::Debug;

use hal::i2c::{ErrorType, I2c};

use super::{GPIOBank, Register};

pub mod cached;
pub mod immediate;
pub mod io;
pub mod standard;

/// Trait for standard IO expanders which are not Sync
pub trait Expander<I2C>
where
    I2C: I2c,
{
    fn write_byte(
        &mut self,
        register: Register,
        data: u8,
    ) -> Result<(), ExpanderError<<I2C as ErrorType>::Error>>;

    fn read_byte(
        &mut self,
        register: Register,
        buffer: &mut u8,
    ) -> Result<(), ExpanderError<<I2C as ErrorType>::Error>>;
    fn write_halfword(
        &mut self,
        register: Register,
        data: u16,
    ) -> Result<(), ExpanderError<<I2C as ErrorType>::Error>>;
    fn read_halfword(
        &mut self,
        register: Register,
        buffer: &mut u16,
    ) -> Result<(), ExpanderError<<I2C as ErrorType>::Error>>;
}

/// Trait for IO expanders which use some synchronization primitive for the writes and reads. This implementation makes the expander sync and usable accross threads etc.
pub trait SyncExpander<I2C>
where
    I2C: I2c,
{
    fn write_byte(
        &self,
        register: Register,
        data: u8,
    ) -> Result<(), ExpanderError<<I2C as ErrorType>::Error>>;
    fn read_byte(
        &self,
        register: Register,
        buffer: &mut u8,
    ) -> Result<(), ExpanderError<<I2C as ErrorType>::Error>>;
    fn write_halfword(
        &self,
        register: Register,
        data: u16,
    ) -> Result<(), ExpanderError<<I2C as ErrorType>::Error>>;
    fn read_halfword(
        &self,
        register: Register,
        buffer: &mut u16,
    ) -> Result<(), ExpanderError<<I2C as ErrorType>::Error>>;
}

#[derive(Debug)]
pub enum ExpanderError<ERR>
where
    ERR: Debug,
{
    WriteError(ERR),
    WriteReadError(ERR),
}

impl<ERR: Debug> hal::digital::Error for ExpanderError<ERR> {
    fn kind(&self) -> hal::digital::ErrorKind {
        hal::digital::ErrorKind::Other
    }
}

#[cfg(feature = "std")]
impl<T> std::fmt::Display for ExpanderError<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self)
    }
}

#[cfg(feature = "std")]
impl<T> std::error::Error for ExpanderError<T>
where
    T: Debug,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
