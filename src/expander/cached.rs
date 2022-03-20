//! Contains the implementation of the Cached Expander interface.

use core::fmt::Debug;

use hal::digital::blocking::InputPin;
use hal::i2c::blocking::{Write, WriteRead};

use crate::StandardExpanderInterface;

use super::{Expander, ExpanderError, Register};

#[derive(Debug)]
pub struct Pca9535Cached<'a, I2C, IP>
where
    I2C: Write + WriteRead,
    IP: InputPin,
{
    address: u8,
    i2c: I2C,
    interrupt_pin: &'a IP,

    input_port_0: u8,
    input_port_1: u8,
    output_port_0: u8,
    output_port_1: u8,
    polarity_inversion_port_0: u8,
    polarity_inversion_port_1: u8,
    configuration_port_0: u8,
    configuration_port_1: u8,
}

impl<'a, I2C, E, IP> Pca9535Cached<'a, I2C, IP>
where
    IP: InputPin,
    E: Debug,
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    ///Creates a new cached PCA9535 instance.
    ///
    /// # Cached registers
    /// The init_defaults argument assumes the default values for all the registers of the device if set to `true` (Default register condition after device startup, see the device's documentation for more information). In that case no bus transaction is created to verify if this is actually the case on the device. Only use this option if you have not made any transactions with the device before creating this expander struct, otherwise you might encounter unexpected behavior of the device!
    /// If the device was used before calling this function and should keep its state you should set init_defaults to `false`. This triggers a bus transaction to read out all the devices' registers and caches the received values.
    ///
    /// # Panics
    /// If given device hardware address is outside of the permittable range of `32-39`.
    pub fn new(
        i2c: I2C,
        address: u8,
        interrupt_pin: &'a IP,
        init_defaults: bool,
    ) -> Result<Self, ExpanderError<E>> {
        assert!(address > 31 && address < 40);

        let mut expander = Self {
            address,
            i2c,
            interrupt_pin,
            input_port_0: 0x00,
            input_port_1: 0x00,
            output_port_0: 0xFF,
            output_port_1: 0xFF,
            polarity_inversion_port_0: 0x00,
            polarity_inversion_port_1: 0x00,
            configuration_port_0: 0xFF,
            configuration_port_1: 0xFF,
        };

        if !init_defaults {
            Self::init_cache(&mut expander)?;
        }

        Ok(expander)
    }

    /// Initializes the device's cache by reading out all the required registers of the device.
    fn init_cache(expander: &mut Self) -> Result<(), ExpanderError<E>> {
        let mut buf: [u8; 2] = [0x00, 0x00];

        expander
            .i2c
            .write_read(
                expander.address,
                &[Register::ConfigurationPort0 as u8],
                &mut buf,
            )
            .map_err(ExpanderError::WriteReadError)?;
        expander.configuration_port_0 = buf[0];
        expander.configuration_port_1 = buf[1];

        expander
            .i2c
            .write_read(expander.address, &[Register::InputPort0 as u8], &mut buf)
            .map_err(ExpanderError::WriteReadError)?;
        expander.input_port_0 = buf[0];
        expander.input_port_1 = buf[1];

        expander
            .i2c
            .write_read(expander.address, &[Register::OutputPort0 as u8], &mut buf)
            .map_err(ExpanderError::WriteReadError)?;
        expander.output_port_0 = buf[0];
        expander.output_port_1 = buf[1];

        expander
            .i2c
            .write_read(
                expander.address,
                &[Register::PolarityInversionPort0 as u8],
                &mut buf,
            )
            .map_err(ExpanderError::WriteReadError)?;
        expander.polarity_inversion_port_0 = buf[0];
        expander.polarity_inversion_port_1 = buf[1];

        Ok(())
    }

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

impl<'a, I2C, E, IP> Expander<I2C> for Pca9535Cached<'a, I2C, IP>
where
    IP: InputPin,
    E: Debug,
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Writes one byte to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Cached
    /// If the bus write succeeds the written data is cached to avoid the need for bus traffic upon reading the written register.
    fn write_byte(&mut self, register: Register, data: u8) -> Result<(), ExpanderError<E>> {
        self.i2c
            .write(self.address, &[register as u8, data])
            .map_err(ExpanderError::WriteError)?;

        // As the IO Expander does not trigger an interrupt once the polarity inversion register value changes, writes to the polarity inversion registers need a special implementation in order to ensure that the input register cache stays up to date.
        if register.is_polarity_inversion() {
            let input_mask = self.get_cached(register) ^ data;

            match register {
                Register::PolarityInversionPort0 => self.set_cached(
                    Register::InputPort0,
                    self.get_cached(Register::InputPort0) ^ input_mask,
                ),
                Register::PolarityInversionPort1 => self.set_cached(
                    Register::InputPort1,
                    self.get_cached(Register::InputPort1) ^ input_mask,
                ),
                _ => unreachable!(),
            }
        }

        self.set_cached(register, data);
        Ok(())
    }

    /// Reads one byte of given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Cached
    /// This function only creates bus traffic in case the provided interrupt pin is held at a `low` voltage level at the time of the function call and the provided register is an input register. In that case the data is being read from the device, as the devices interrupt output indicates a data change. Otherwise the cached value is returned without causing any bus traffic.
    fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), ExpanderError<E>> {
        if self.interrupt_pin.is_low().unwrap() && register.is_input() {
            let mut buf = [0u8];

            self.i2c
                .write_read(self.address, &[register as u8], &mut buf)
                .map_err(ExpanderError::WriteReadError)?;

            self.set_cached(register, buf[0]);

            *buffer = buf[0];
        } else {
            *buffer = self.get_cached(register);
        }

        Ok(())
    }

    /// Writes one halfword to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Register pairs
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    ///
    /// # Cached
    /// If the bus write succeeds the written data is cached to avoid the need for bus traffic upon reading the written register.
    fn write_halfword(&mut self, register: Register, data: u16) -> Result<(), ExpanderError<E>> {
        self.i2c
            .write(
                self.address,
                &[register as u8, (data >> 8) as u8, data as u8],
            )
            .map_err(ExpanderError::WriteError)?;

        // As the IO Expander does not trigger an interrupt once the polarity inversion register value changes, writes to the polarity inversion registers need a special implementation in order to ensure that the input register cache stays up to date.
        if register.is_polarity_inversion() {
            let input_mask_1 = self.get_cached(register) ^ (data >> 8) as u8;
            let input_mask_2 = self.get_cached(register.get_neighbor()) ^ data as u8;

            if matches!(register, Register::PolarityInversionPort0) {
                self.set_cached(
                    Register::InputPort0,
                    self.get_cached(Register::InputPort0) ^ input_mask_1,
                );
                self.set_cached(
                    Register::InputPort1,
                    self.get_cached(Register::InputPort1) ^ input_mask_2,
                );
            } else {
                self.set_cached(
                    Register::InputPort1,
                    self.get_cached(Register::InputPort1) ^ input_mask_1,
                );
                self.set_cached(
                    Register::InputPort0,
                    self.get_cached(Register::InputPort0) ^ input_mask_2,
                );
            }
        }

        self.set_cached(register, (data >> 8) as u8);
        self.set_cached(register.get_neighbor(), data as u8);

        Ok(())
    }

    /// Reads one halfword of given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Register pairs
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    ///
    /// # Cached
    /// This function only creates bus traffic in case the provided interrupt pin is held at a `low` voltage level at the time of the function call and the provided register is an input register. In that case the data is being read from the device, as the devices interrupt output indicates a data change. Otherwise the cached value is returned without causing any bus traffic.
    fn read_halfword(
        &mut self,
        register: Register,
        buffer: &mut u16,
    ) -> Result<(), ExpanderError<E>> {
        let mut reg_val: [u8; 2] = [0x00; 2];

        if self.interrupt_pin.is_low().unwrap() && register.is_input() {
            self.i2c
                .write_read(self.address, &[register as u8], &mut reg_val)
                .map_err(ExpanderError::WriteReadError)?;

            self.set_cached(register, reg_val[0]);
            self.set_cached(register.get_neighbor(), reg_val[1]);

            *buffer = (reg_val[0] as u16) << 8 | reg_val[1] as u16;
        } else {
            *buffer = (self.get_cached(register) as u16) << 8
                | self.get_cached(register.get_neighbor()) as u16;
        }

        Ok(())
    }
}

impl<'a, I2C, E, IP> StandardExpanderInterface<I2C, E> for Pca9535Cached<'a, I2C, IP>
where
    IP: InputPin,
    E: Debug,
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
}
