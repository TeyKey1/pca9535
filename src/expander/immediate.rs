//! Contains the implementation of the Immediate Expander interface.
use core::fmt::Debug;

use hal::i2c::blocking::{Write, WriteRead};

use crate::StandardExpanderInterface;

use super::{Expander, ExpanderError, Register};

#[derive(Debug)]
pub struct Pca9535Immediate<I2C>
where
    I2C: Write + WriteRead,
{
    address: u8,
    i2c: I2C,
}

impl<I2C> Pca9535Immediate<I2C>
where
    I2C: Write + WriteRead,
{
    /// Creates a new immediate PCA9535 instance.
    ///
    /// # Panics
    /// If given device hardware address is outside of the permittable range of `32-39`.
    pub fn new(i2c: I2C, address: u8) -> Self {
        assert!(address > 31 && address < 40);

        Self { address, i2c }
    }
}

impl<I2C, E> Expander<I2C> for Pca9535Immediate<I2C>
where
    E: Debug,
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Writes one byte to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    fn write_byte(&mut self, register: Register, data: u8) -> Result<(), ExpanderError<E>> {
        self.i2c
            .write(self.address, &[register as u8, data])
            .map_err(ExpanderError::WriteError)
    }

    /// Reads one byte of given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), ExpanderError<E>> {
        let mut buf = [0_u8];

        self.i2c
            .write_read(self.address, &[register as u8], &mut buf)
            .map_err(ExpanderError::WriteReadError)?;

        *buffer = buf[0];

        Ok(())
    }

    /// Writes one halfword to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Register pairs
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    fn write_halfword(&mut self, register: Register, data: u16) -> Result<(), ExpanderError<E>> {
        self.i2c
            .write(
                self.address,
                &[register as u8, (data >> 8) as u8, data as u8],
            )
            .map_err(ExpanderError::WriteError)
    }

    /// Reads one halfword of given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Register pairs
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    fn read_halfword(
        &mut self,
        register: Register,
        buffer: &mut u16,
    ) -> Result<(), ExpanderError<E>> {
        let mut reg_val: [u8; 2] = [0x00; 2];

        self.i2c
            .write_read(self.address, &[register as u8], &mut reg_val)
            .map_err(ExpanderError::WriteReadError)?;

        *buffer = (reg_val[0] as u16) << 8 | reg_val[1] as u16;

        Ok(())
    }
}

impl<I2C, E> StandardExpanderInterface<I2C, E> for Pca9535Immediate<I2C>
where
    E: Debug,
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
}
