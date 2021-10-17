use core::fmt::Debug;

use crate::Register;

pub mod cached;
pub mod immediate;

pub trait Expander {
    type Error: core::fmt::Debug;
    fn write_byte(&mut self, register: Register, data: u8) -> Result<(), Self::Error>;
    fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), Self::Error>;
    fn write_halfword(&mut self, register: Register, data: u16) -> Result<(), Self::Error>;
    fn read_halfword(&mut self, register: Register, buffer: &mut [u8]) -> Result<(), Self::Error>;
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
