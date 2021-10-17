use core::cell::RefCell;
use hal::digital::blocking::{InputPin, IoPin, OutputPin};
use hal::digital::PinState;

use super::expander::Expander;
use super::GPIOBank;
use super::Register;

pub struct ExpanderInputPin<Ex>
where
    Ex: Expander,
{
    expander: RefCell<Ex>,
    bank: GPIOBank,
    pin: u8,
}

pub struct ExpanderOutputPin<Ex>
where
    Ex: Expander,
{
    expander: RefCell<Ex>,
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

        self.expander
            .borrow_mut()
            .read_byte(register, &mut reg_val)?;

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

        self.expander
            .borrow_mut()
            .read_byte(register, &mut reg_val)?;

        match (reg_val >> self.pin) & 1 {
            1 => Ok(false),
            _ => Ok(true),
        }
    }
}

impl<Ex: Expander> IoPin<ExpanderInputPin<Ex>, ExpanderOutputPin<Ex>> for ExpanderInputPin<Ex> {
    type Error = Ex::Error;

    fn into_input_pin(self) -> Result<ExpanderInputPin<Ex>, Self::Error> {
        Ok(self)
    }

    fn into_output_pin(self, state: PinState) -> Result<ExpanderOutputPin<Ex>, Self::Error> {
        let cp_register = match self.bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let op_register = match self.bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        {
            let mut expander = self.expander.borrow_mut();
            let mut reg_val: u8 = 0x00;

            expander.read_byte(op_register, &mut reg_val)?;

            if let PinState::High = state {
                expander.write_byte(op_register, reg_val | (0x01 << self.pin))?;
            } else {
                expander.write_byte(op_register, reg_val & !(0x01 << self.pin))?;
            }

            expander.read_byte(cp_register, &mut reg_val)?;

            expander.write_byte(cp_register, reg_val & !(0x01 << self.pin))?;
        }

        Ok(ExpanderOutputPin {
            expander: self.expander,
            bank: self.bank,
            pin: self.pin,
        })
    }
}

impl<Ex: Expander> OutputPin for ExpanderOutputPin<Ex> {
    type Error = Ex::Error;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        let mut expander = self.expander.borrow_mut();
        let mut reg_val: u8 = 0x00;

        expander.read_byte(register, &mut reg_val)?;

        expander.write_byte(register, reg_val & !(0x01 << self.pin))
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::InputPort0,
            GPIOBank::Bank1 => Register::InputPort1,
        };

        let mut expander = self.expander.borrow_mut();
        let mut reg_val: u8 = 0x00;

        expander.read_byte(register, &mut reg_val)?;

        expander.write_byte(register, reg_val | (0x01 << self.pin))
    }
}

impl<Ex: Expander> IoPin<ExpanderInputPin<Ex>, ExpanderOutputPin<Ex>> for ExpanderOutputPin<Ex> {
    type Error = Ex::Error;

    fn into_input_pin(self) -> Result<ExpanderInputPin<Ex>, Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        {
            let mut expander = self.expander.borrow_mut();
            let mut reg_val: u8 = 0x00;

            expander.read_byte(register, &mut reg_val)?;

            expander.write_byte(register, reg_val | (0x01 << self.pin))?;
        }

        Ok(ExpanderInputPin {
            expander: self.expander,
            bank: self.bank,
            pin: self.pin,
        })
    }

    fn into_output_pin(self, state: PinState) -> Result<ExpanderOutputPin<Ex>, Self::Error> {
        let register = match self.bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        {
            let mut expander = self.expander.borrow_mut();
            let mut reg_val: u8 = 0x00;

            expander.read_byte(register, &mut reg_val)?;

            if let PinState::High = state {
                expander.write_byte(register, reg_val | (0x01 << self.pin))?;
            } else {
                expander.write_byte(register, reg_val & !(0x01 << self.pin))?;
            }
        }

        Ok(self)
    }
}
