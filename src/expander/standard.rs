//! Implements the standard interface for all types implementing [`Expander`] trait.
use core::fmt::Debug;

use hal::i2c::I2c;

use super::{Expander, ExpanderError, GPIOBank, Register};

/// Standard expander interface not using [`hal`].
///
/// This interface does not track the state of the pins! Therefore, the user needs to ensure the pins are in input or output configuration before
/// proceeding to call functions related to input or output pins. Otherwise the results of those functions might not cause the expected behavior of the device.
pub trait StandardExpanderInterface<I2C, E>: Expander<I2C>
where
    E: Debug,
    I2C: I2c<Error = E>,
{
    fn pin_set_high(&mut self, bank: GPIOBank, pin: u8) -> Result<(), ExpanderError<E>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        self.write_byte(register, reg_val | (0x01 << pin))
    }

    /// Drives given pin low.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    fn pin_set_low(&mut self, bank: GPIOBank, pin: u8) -> Result<(), ExpanderError<E>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::OutputPort0,
            GPIOBank::Bank1 => Register::OutputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        self.write_byte(register, reg_val & !(0x01 << pin))
    }

    /// Checks if input state of given pin is `high`. This function works with pins configured as inputs as well as outputs.
    ///
    /// The function result does not necessarily represent the logic level of the applied voltage at the given pin but the value inside the input register of the device.
    /// Which is `1` or `0` Depending on the current polarity inversion configuration of the pin.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    fn pin_is_high(&mut self, bank: GPIOBank, pin: u8) -> Result<bool, ExpanderError<E>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::InputPort0,
            GPIOBank::Bank1 => Register::InputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        match (reg_val >> pin) & 1 {
            1 => Ok(true),
            _ => Ok(false),
        }
    }

    /// Checks if input state of given pin is `low`. This function works with pins configured as inputs as well as outputs.
    ///
    /// The function result does not necessarily represent the logic level of the applied voltage at the given pin but the value inside the input register of the device.
    /// Which is `1` or `0` Depending on the current polarity inversion configuration of the pin.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    fn pin_is_low(&mut self, bank: GPIOBank, pin: u8) -> Result<bool, ExpanderError<E>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::InputPort0,
            GPIOBank::Bank1 => Register::InputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        match (reg_val >> pin) & 1 {
            1 => Ok(false),
            _ => Ok(true),
        }
    }

    /// Configures given pin as input.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    fn pin_into_input(&mut self, bank: GPIOBank, pin: u8) -> Result<(), ExpanderError<E>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        self.write_byte(register, reg_val | (0x01 << pin))
    }

    /// Configures given pin as output.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    fn pin_into_output(&mut self, bank: GPIOBank, pin: u8) -> Result<(), ExpanderError<E>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::ConfigurationPort0,
            GPIOBank::Bank1 => Register::ConfigurationPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        self.write_byte(register, reg_val & !(0x01 << pin))
    }

    /// Sets the input polarity of the given pin to inverted.
    ///
    /// A logic high voltage applied at this input pin results in a `0` written to the devices input register and thus being registered as `low` by the driver.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    fn pin_inverse_polarity(&mut self, bank: GPIOBank, pin: u8) -> Result<(), ExpanderError<E>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::PolarityInversionPort0,
            GPIOBank::Bank1 => Register::PolarityInversionPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        self.write_byte(register, reg_val | (0x01 << pin))
    }

    /// Sets the input polarity of the given pin to normal.
    ///
    /// A logic high voltage applied at an input pin results in a `1` written to the devices input register and thus being registered as `high` by the driver.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    fn pin_normal_polarity(&mut self, bank: GPIOBank, pin: u8) -> Result<(), ExpanderError<E>> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::PolarityInversionPort0,
            GPIOBank::Bank1 => Register::PolarityInversionPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        self.write_byte(register, reg_val & !(0x01 << pin))
    }

    /// Sets the input polarity of all pins to inverted.
    ///
    /// A logic high voltage applied at an input pin results in a `0` written to the devices input register and thus being registered as `low` by the driver.
    fn inverse_polarity(&mut self) -> Result<(), ExpanderError<E>> {
        self.write_halfword(Register::PolarityInversionPort0, 0xFFFF_u16)
    }

    /// Sets the input polarity of all pins to normal.
    ///
    /// A logic high voltage applied at an input pin results in a `1` written to the devices input register and thus being registered as `high` by the driver.
    fn normal_polarity(&mut self) -> Result<(), ExpanderError<E>> {
        self.write_halfword(Register::PolarityInversionPort0, 0x0_u16)
    }
}
