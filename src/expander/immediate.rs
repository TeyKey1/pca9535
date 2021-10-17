use hal::i2c::blocking::{Write, WriteRead};

use super::Expander;
use super::ExpanderError;
use super::Register;

#[derive(Debug)]
pub struct Pca9535Immediate<I2C>
where
    I2C: Write + WriteRead,
{
    address: u8,
    i2c: I2C,
}

impl<I2C: Write + WriteRead> Expander for Pca9535Immediate<I2C> {
    type Error = ExpanderError<<I2C as WriteRead>::Error, <I2C as Write>::Error>;

    /// Writes one byte to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    fn write_byte(&mut self, register: Register, data: u8) -> Result<(), Self::Error> {
        self.i2c
            .write(self.address, &[register as u8, data])
            .map_err(Self::Error::from_write)
    }

    /// Reads one byte of given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), Self::Error> {
        self.i2c
            .write_read(self.address, &[register as u8], &mut [*buffer])
            .map_err(Self::Error::from_write_read)
    }

    /// Writes one halfword to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Register pairs
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    fn write_halfword(&mut self, register: Register, data: u16) -> Result<(), Self::Error> {
        self.i2c
            .write(
                self.address,
                &[register as u8, (data >> 8) as u8, data as u8],
            )
            .map_err(Self::Error::from_write)
    }

    /// Reads one halfword of given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Register pairs
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    fn read_halfword(&mut self, register: Register, buffer: &mut u16) -> Result<(), Self::Error> {
        let mut reg_val: [u8; 2] = [0x00; 2];

        self.i2c
            .write_read(self.address, &[register as u8], &mut reg_val)
            .map_err(Self::Error::from_write_read)?;

        *buffer = (reg_val[0] as u16) << 8 & reg_val[1] as u16;

        Ok(())
    }
}
