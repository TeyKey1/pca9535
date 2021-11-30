use core::fmt::Debug;

use super::{GPIOBank, Register};

pub mod cached;
pub mod immediate;
pub mod io;
mod standard;

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

/// Standard expander interface not using [`hal`].
///
/// This interface does not track the state of the pins! Therefore the user needs to ensure the pins are in input or output configuration before proceeding to call functions related to input or output pins. Otherwise the results of those functions might not cause the expected behavior of the device.
pub trait StandardExpanderInterface {
    type Error: core::fmt::Debug;

    fn pin_set_high(&mut self, bank: GPIOBank, pin: u8) -> Result<(), Self::Error>;
    fn pin_set_low(&mut self, bank: GPIOBank, pin: u8) -> Result<(), Self::Error>;
    fn pin_is_high(&mut self, bank: GPIOBank, pin: u8) -> Result<bool, Self::Error>;
    fn pin_is_low(&mut self, bank: GPIOBank, pin: u8) -> Result<bool, Self::Error>;
    fn pin_into_input(&mut self, bank: GPIOBank, pin: u8) -> Result<(), Self::Error>;
    fn pin_into_output(&mut self, bank: GPIOBank, pin: u8) -> Result<(), Self::Error>;
    fn pin_inverse_polarity(&mut self, bank: GPIOBank, pin: u8) -> Result<(), Self::Error>;
    fn pin_normal_polarity(&mut self, bank: GPIOBank, pin: u8) -> Result<(), Self::Error>;
    fn inverse_polarity(&mut self) -> Result<(), Self::Error>;
    fn normal_polarity(&mut self) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum ExpanderError<WR, W> {
    WriteError(W),
    WriteReadError(WR),
}

impl<WR, W> ExpanderError<WR, W> {
    fn from_write_read(err: WR) -> Self {
        Self::WriteReadError(err)
    }

    fn from_write(err: W) -> Self {
        Self::WriteError(err)
    }
}
