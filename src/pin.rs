use hal::digital::blocking::{InputPin, IoPin, OutputPin};
use hal::digital::PinState;

use crate::{Expander, GPIOBank, Register};

struct ExpanderInputPin<Ex>
where
    Ex: Expander,
{
    expander: Ex,
    bank: GPIOBank,
    pin: u8,
}

struct ExpanderOutputPin<Ex>
where
    Ex: Expander,
{
    expander: Ex,
    bank: GPIOBank,
    pin: u8,
}

impl<Ex: Expander> InputPin for ExpanderInputPin<Ex> {
    type Error = Ex::Error;

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

impl<Ex: Expander> IoPin<ExpanderInputPin<Ex>, ExpanderOutputPin<Ex>> for ExpanderInputPin<Ex> {
    type Error = Expander::Error;

    fn into_input_pin(self) -> Result<ExpanderInputPin<Ex>, Self::Error> {
        Ok(self)
    }

    fn into_output_pin(self, state: PinState) -> Result<ExpanderOutputPin<Ex>, Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        self.expander
            .write_byte(register, reg_val & !(0x01 << self.pin))?;

        Ok(ExpanderOutputPin { ..self })
    }
}

impl<Ex: Expander> OutputPin for ExpanderOutputPin<Ex> {
    type Error = Ex::Error;

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
            GPIOBank::Bank0 => Register::InputPort0,
            GPIOBank::Bank1 => Register::InputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        self.expander
            .write_byte(register, reg_val | (0x01 << self.pin))
    }
}

impl<Ex: Expander> IoPin<ExpanderInputPin<Ex>, ExpanderOutputPin<Ex>> for ExpanderOutputPin<Ex> {
    type Error = Expander::Error;

    fn into_input_pin(self) -> Result<ExpanderInputPin<Ex>, Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.expander.read_byte(register, &mut reg_val)?;

        self.expander
            .write_byte(register, reg_val | (0x01 << self.pin))?;

        Ok(ExpanderOutputPin { ..self })
    }

    fn into_output_pin(self, state: PinState) -> Result<ExpanderOutputPin<Ex>, Self::Error> {
        Ok(self)
    }
}
