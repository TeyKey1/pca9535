//! Contains the implementation of the Immediate Expander interface.
use core::fmt::Debug;

use hal::i2c::I2c;

use crate::StandardExpanderInterface;

use super::{Expander, ExpanderError, Register};

#[derive(Debug)]
pub struct Pca9535Immediate<I2C>
where
    I2C: I2c,
{
    address: u8,
    i2c: I2C,
}

impl<I2C> Pca9535Immediate<I2C>
where
    I2C: I2c,
{
    /// Creates a new immediate PCA9535 instance.
    ///
    /// # Panics
    /// If the given device hardware address is outside the permittable range of `32-39`.
    pub fn new(i2c: I2C, address: u8) -> Self {
        assert!(address > 31 && address < 40);

        Self { address, i2c }
    }

    /// Destroys the expander struct, returning the contained I2C
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, E> Expander<I2C> for Pca9535Immediate<I2C>
where
    E: Debug,
    I2C: I2c<Error = E>,
{
    /// Writes one byte to the given register
    ///
    /// Only use this function if you really have to. For most use cases, the crate provides simpler ways of interacting with the device.
    fn write_byte(&mut self, register: Register, data: u8) -> Result<(), ExpanderError<E>> {
        self.i2c
            .write(self.address, &[register as u8, data])
            .map_err(ExpanderError::WriteError)
    }

    /// Reads one byte of the given register
    ///
    /// Only use this function if you really have to. For most use cases, the crate provides simpler ways of interacting with the device.
    fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), ExpanderError<E>> {
        let mut buf = [0_u8];

        self.i2c
            .write_read(self.address, &[register as u8], &mut buf)
            .map_err(ExpanderError::WriteReadError)?;

        *buffer = buf[0];

        Ok(())
    }

    /// Writes one halfword to the given register
    ///
    /// Only use this function if you really have to. For most use cases, the crate provides simpler ways of interacting with the device.
    ///
    /// # Register pairs
    /// Please see [`Register`] for more information about the register pairs and how they affect the half-word read and write functions.
    fn write_halfword(&mut self, register: Register, data: u16) -> Result<(), ExpanderError<E>> {
        self.i2c
            .write(
                self.address,
                &[register as u8, (data >> 8) as u8, data as u8],
            )
            .map_err(ExpanderError::WriteError)
    }

    /// Reads one halfword of the given register
    ///
    /// Only use this function if you really have to. For most use cases, the crate provides simpler ways of interacting with the device.
    ///
    /// # Register pairs
    /// Please see [`Register`] for more information about the register pairs and how they affect the half-word read and write functions.
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
    I2C: I2c<Error = E>,
{
}
