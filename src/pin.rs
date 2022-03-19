//! Contains the implementation of the hal-pin usage inteface.

extern crate alloc;

use std::marker::PhantomData;

use hal::digital::blocking::{InputPin, IoPin, OutputPin};
use hal::digital::PinState;
use hal::i2c::blocking::{Write, WriteRead};

use crate::ExpanderError;

use super::expander::SyncExpander;
use super::GPIOBank;
use super::Polarity;
use super::Register;

/// Single input device pin implementing [`InputPin`] and [`IoPin`] trait.
///
/// The [`ExpanderInputPin`] instance can be used with other pieces of software using [`hal`].
pub struct ExpanderInputPin<'a, I2C, Io>
where
    I2C: Write + WriteRead,
    Io: SyncExpander<I2C>,
{
    expander: &'a Io,
    bank: GPIOBank,
    pin: u8,
    phantom_data: PhantomData<I2C>,
}

/// Single output device pin implementing [`OutputPin`] and [`IoPin`] trait.
///
/// The [`ExpanderInputPin`] instance can be used with other pieces of software using [`hal`].
pub struct ExpanderOutputPin<'a, I2C, Io>
where
    I2C: Write + WriteRead,
    Io: SyncExpander<I2C>,
{
    expander: &'a Io,
    bank: GPIOBank,
    pin: u8,
    phantom_data: PhantomData<I2C>,
}

impl<'a, I2C: Write + WriteRead, Io: SyncExpander<I2C>> ExpanderInputPin<'a, I2C, Io> {
    /// Create a new input pin
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    pub fn new(expander: &'a Io, bank: GPIOBank, pin: u8) -> Result<Self, ExpanderError<I2C>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let mut reg_val: u8 = 0x00;

        expander.read_byte(register, &mut reg_val)?;
        expander.write_byte(register, reg_val | (0x01 << pin))?;

        Ok(Self {
            expander,
            bank,
            pin,
            phantom_data: PhantomData,
        })
    }

    /// Sets the polarity of the input pin. The input pins have normal polarity by default on device startup.
    ///
    /// If the polarity is [`Polarity::Normal`] a logic `high` voltage level on the input is detected as `high` in the software.
    ///
    /// If the polarity is [`Polarity::Inverse`] a logic `high` voltage level on the input is detected as `low` by the software.
    pub fn set_polarity(&mut self, polarity: Polarity) -> Result<(), ExpanderError<I2C>> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::PolarityInversionPort0,
            GPIOBank::Bank1 => Register::PolarityInversionPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        if let Polarity::Normal = polarity {
            self.expander
                .write_byte(register, reg_val & !(0x01 << self.pin))?;
        } else {
            self.expander
                .write_byte(register, reg_val | (0x01 << self.pin))?;
        }

        Ok(())
    }
}

impl<'a, I2C: Write + WriteRead, Io: SyncExpander<I2C>> ExpanderOutputPin<'a, I2C, Io> {
    /// Create a new output pin
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    pub fn new(
        expander: &'a Io,
        bank: GPIOBank,
        pin: u8,
        state: PinState,
    ) -> Result<Self, ExpanderError<I2C>> {
        assert!(pin < 8);

        let cp_register = match bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let op_register = match bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        let mut reg_val: u8 = 0x00;

        expander.read_byte(op_register, &mut reg_val)?;

        if let PinState::High = state {
            expander.write_byte(op_register, reg_val | (0x01 << pin))?;
        } else {
            expander.write_byte(op_register, reg_val & !(0x01 << pin))?;
        }

        expander.read_byte(cp_register, &mut reg_val)?;

        expander.write_byte(cp_register, reg_val & !(0x01 << pin))?;

        Ok(Self {
            expander,
            bank,
            pin,
            phantom_data: PhantomData,
        })
    }
}

impl<'a, I2C: Write + WriteRead, Io: SyncExpander<I2C>> InputPin for ExpanderInputPin<'a, I2C, Io> {
    type Error = ExpanderError<I2C>;

    fn is_high(&self) -> Result<bool, Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::InputPort0,
            GPIOBank::Bank1 => Register::InputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        match (reg_val >> self.pin) & 1 {
            1 => Ok(true),
            _ => Ok(false),
        }
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::InputPort0,
            GPIOBank::Bank1 => Register::InputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        match (reg_val >> self.pin) & 1 {
            1 => Ok(false),
            _ => Ok(true),
        }
    }
}

impl<'a, I2C: Write + WriteRead, Io: SyncExpander<I2C>>
    IoPin<ExpanderInputPin<'a, I2C, Io>, ExpanderOutputPin<'a, I2C, Io>>
    for ExpanderInputPin<'a, I2C, Io>
{
    type Error = ExpanderError<I2C>;

    fn into_input_pin(self) -> Result<ExpanderInputPin<'a, I2C, Io>, Self::Error> {
        Ok(self)
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<ExpanderOutputPin<'a, I2C, Io>, Self::Error> {
        let cp_register = match self.bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let op_register = match self.bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(op_register, &mut reg_val)?;

        if let PinState::High = state {
            self.expander
                .write_byte(op_register, reg_val | (0x01 << self.pin))?;
        } else {
            self.expander
                .write_byte(op_register, reg_val & !(0x01 << self.pin))?;
        }

        self.expander.read_byte(cp_register, &mut reg_val)?;

        self.expander
            .write_byte(cp_register, reg_val & !(0x01 << self.pin))?;

        Ok(ExpanderOutputPin {
            expander: self.expander,
            bank: self.bank,
            pin: self.pin,
            phantom_data: PhantomData,
        })
    }
}

impl<'a, I2C: Write + WriteRead, Io: SyncExpander<I2C>> OutputPin
    for ExpanderOutputPin<'a, I2C, Io>
{
    type Error = ExpanderError<I2C>;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        self.expander
            .write_byte(register, reg_val & !(0x01 << self.pin))
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        self.expander
            .write_byte(register, reg_val | (0x01 << self.pin))
    }
}

impl<'a, I2C: Write + WriteRead, Io: SyncExpander<I2C>>
    IoPin<ExpanderInputPin<'a, I2C, Io>, ExpanderOutputPin<'a, I2C, Io>>
    for ExpanderOutputPin<'a, I2C, Io>
{
    type Error = ExpanderError<I2C>;

    fn into_input_pin(self) -> Result<ExpanderInputPin<'a, I2C, Io>, Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        self.expander
            .write_byte(register, reg_val | (0x01 << self.pin))?;

        Ok(ExpanderInputPin {
            expander: self.expander,
            bank: self.bank,
            pin: self.pin,
            phantom_data: PhantomData,
        })
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<ExpanderOutputPin<'a, I2C, Io>, Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        if let PinState::High = state {
            self.expander
                .write_byte(register, reg_val | (0x01 << self.pin))?;
        } else {
            self.expander
                .write_byte(register, reg_val & !(0x01 << self.pin))?;
        }

        Ok(self)
    }
}
