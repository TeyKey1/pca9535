use hal::digital::blocking::InputPin;
use hal::i2c::blocking::{Write, WriteRead};

use super::Expander;
use super::ExpanderError;
use super::Register;
use crate::pin::ExpanderInputPin;
#[derive(Debug)]
pub struct Pca9535Cached<I2C, IP>
where
    I2C: Write + WriteRead,
    IP: InputPin,
{
    address: u8,
    i2c: I2C,
    interrupt_pin: IP,

    input_port_0: u8,
    input_port_1: u8,
    output_port_0: u8,
    output_port_1: u8,
    polarity_inversion_port_0: u8,
    polarity_inversion_port_1: u8,
    configuration_port_0: u8,
    configuration_port_1: u8,
}

impl<I2C: Write + WriteRead, IP: InputPin> Pca9535Cached<I2C, IP> {
    fn new(i2c: I2C, address: u8) -> Self {
        assert!(address > 31 && address < 40);
    }

    fn init_cache(
        &mut self,
    ) -> Result<(), ExpanderError<<I2C as WriteRead>::Error, <I2C as Write>::Error>> {
    }

    fn create_pin(&mut self) -> ExpanderInputPin<Ex> {}

    fn get_cached(&self, register: Register) -> u8 {
        match register {
            Register::InputPort0 => self.input_port_0,
            Register::InputPort1 => self.input_port_1,
            Register::OutputPort0 => self.output_port_0,
            Register::OutputPort1 => self.output_port_1,
            Register::PolarityInversionPort0 => self.polarity_inversion_port_0,
            Register::PolarityInversionPort1 => self.polarity_inversion_port_1,
            Register::ConfigurationPort0 => self.configuration_port_0,
            Register::ConfigurationPort1 => self.configuration_port_1,
        }
    }

    fn set_cached(&mut self, register: Register, value: u8) {
        match register {
            Register::InputPort0 => self.input_port_0 = value,
            Register::InputPort1 => self.input_port_1 = value,
            Register::OutputPort0 => self.output_port_0 = value,
            Register::OutputPort1 => self.output_port_1 = value,
            Register::PolarityInversionPort0 => self.polarity_inversion_port_0 = value,
            Register::PolarityInversionPort1 => self.polarity_inversion_port_1 = value,
            Register::ConfigurationPort0 => self.configuration_port_0 = value,
            Register::ConfigurationPort1 => self.configuration_port_1 = value,
        };
    }
}

impl<I2C: Write + WriteRead, IP: InputPin> Expander for Pca9535Cached<I2C, IP> {
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
        if self.interrupt_pin.is_low().unwrap() {
            self.i2c
                .write_read(self.address, &[register as u8], &mut [*buffer])
                .map_err(Self::Error::from_write_read)?;
        } else {
            *buffer = self.get_cached(register);
        }

        Ok(())
    }

    /// Writes one halfword to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// **Register pairs**
    ///
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
    /// **Register pairs**
    ///
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    fn read_halfword(&mut self, register: Register, buffer: &mut u16) -> Result<(), Self::Error> {
        let mut reg_1_val: u8 = 0x00;
        let mut reg_2_val: u8 = 0x00;

        if self.interrupt_pin.is_low().unwrap() {
            self.i2c
                .write_read(self.address, &[register as u8], &mut [reg_1_val, reg_2_val])
                .map_err(Self::Error::from_write_read)?;

            self.set_cached(register, reg_1_val);
            self.set_cached(register.get_neighbor(), reg_2_val);

            *buffer = (reg_1_val as u16) << 8 & reg_2_val as u16;
        } else {
            *buffer = (self.get_cached(register) as u16) << 8
                & self.get_cached(register.get_neighbor()) as u16;
        }

        Ok(())
    }
}
