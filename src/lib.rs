/*!
PCA9535 driver using embedded-hal

## Device
The PCA9535 and PCA9535C are 16Bit IO-Expanders using the I2C/SMBus interface. The devices operate at a voltage level of 2.3-5.5V

### GPIO
The expander provides two 5V tolerant GPIO banks with eight pins. Each pin is configurable separately as either input or output and additionally allows for polarity inversion.
The open drain interrupt output of the device indicates a change if any of the input states differs from the state of the input port register.

On initialization all pins are configured as high impedance inputs. The PCA9535 features totem pole IOs while the PCA9535C IOs are open-drain.
### I2C
The device uses 7Bit addressing and allows the hardware configuration of the first 3 address bits, allowing for up to 8 expanders on the same bus.

## General info
The library uses the blocking I2C embedded-hal traits. Each implementation of [`expander::Expander`] owns the provided I2C instance, if multiple device access to the bus is required the user has to provide the code to make it work. No synchronization is done inside the library.
For this purpose it is recommended to use crates like [shared-bus](https://crates.io/crates/shared-bus)

# Usage
Usage
*/

#![no_std]

extern crate embedded_hal as hal;

pub mod expander;
pub mod pin;

pub use expander::cached::Pca9535Cached;
pub use expander::immediate::Pca9535Immediate;

pub use pin::ExpanderInputPin;
pub use pin::ExpanderOutputPin;

/*impl<I2C, E> Pca9535<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    ///Creates a new instance of PCA9535 with provided address
    pub fn new(addr: u8, i2c: I2C) -> Self {
        Self {
            address: addr,
            i2c: i2c,
        }
    }

    /// Drives given pin high.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    pub fn pin_set_high(&mut self, bank: GPIOBank, pin: u8) -> Result<(), E> {
        assert!(pin < 8);

        let register = match bank {
            GPIOBank::Bank0 => Register::InputPort0,
            GPIOBank::Bank1 => Register::InputPort1,
        };

        let mut reg_val: u8 = 0x00;

        self.read_byte(register, &mut reg_val)?;

        self.write_byte(register, reg_val | (0x01 << pin))
    }

    /// Drives given pin low.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    pub fn pin_set_low(&mut self, bank: GPIOBank, pin: u8) -> Result<(), E> {
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
    /// The function result does not necessarily represent the logic level of the applied voltage at the given pin but the value inside the input register of the device. Which is `1` or `0` Depending on the current polarity inversion configuration of the pin.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    pub fn pin_is_high(&mut self, bank: GPIOBank, pin: u8) -> Result<bool, E> {
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
    /// The function result does not necessarily represent the logic level of the applied voltage at the given pin but the value inside the input register of the device. Which is `1` or `0` Depending on the current polarity inversion configuration of the pin.
    ///
    /// # Panics
    /// The function will panic if the provided pin is not in the allowed range of 0-7
    pub fn pin_is_low(&mut self, bank: GPIOBank, pin: u8) -> Result<bool, E> {
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
    pub fn pin_into_input(&mut self, bank: GPIOBank, pin: u8) -> Result<(), E> {
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
    pub fn pin_into_output(&mut self, bank: GPIOBank, pin: u8) -> Result<(), E> {
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
    pub fn pin_inverse_polarity(&mut self, bank: GPIOBank, pin: u8) -> Result<(), E> {
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
    pub fn pin_normal_polarity(&mut self, bank: GPIOBank, pin: u8) -> Result<(), E> {
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
    pub fn inverse_polarity(&mut self) -> Result<(), E> {
        self.write_halfword(Register::PolarityInversionPort0, 0xFFFF as u16)
    }

    /// Sets the input polarity of all pins to normal.
    ///
    /// A logic high voltage applied at an input pin results in a `1` written to the devices input register and thus being registered as `high` by the driver.
    pub fn normal_polarity(&mut self) -> Result<(), E> {
        self.write_halfword(Register::PolarityInversionPort0, 0x0 as u16)
    }
}*/

/// The data registers of the device
///
/// The enum represents the command byte values used to access the corresponding registers.
///
/// # Register pairs
/// The registers of the device are all 8 bit and act as four register pairs. Therefore writing a halfword to a register results in the 8 least significant bits being written to the provided register, while the 8 most significant bits will be automatically written to the other register of the pair.
///
/// **Pairs**
/// 1) InputPort0 and InputPort1
/// 2) OutputPort0 and Outputport1
/// 3) PolarityInversionPort0 and PolarityInversionPort1
/// 4) ConfigurationPort0 and ConfigurationPort1
///
/// Example code
/// ```
/// expander.write_halfword(OutputPort0, 0x4A07 as u16).unwrap();
///
/// let mut output_bank0: u8 = 0x00;
/// let mut output_bank1: u8 = 0x00;
///
/// expander.read_byte(OutputPort0, &mut output_bank0).unwrap();
/// expander.read_byte(OutputPort1, &mut output_bank1).unwrap();
///
/// assert_eq!(output_bank0, 0x4A as u8);
/// assert_eq!(output_bank1, 0x07 as u8);
/// ```
/// Or
/// ```
/// expander.write_halfword(OutputPort1, 0x4A07 as u16).unwrap();
///
/// let mut output_bank0: u8 = 0x00;
/// let mut output_bank1: u8 = 0x00;
///
/// expander.read_byte(OutputPort0, &mut output_bank0).unwrap();
/// expander.read_byte(OutputPort1, &mut output_bank1).unwrap();
///
/// assert_eq!(output_bank0, 0x07 as u8);
/// assert_eq!(output_bank1, 0x4A as u8);
/// ```
/// The same principle applies to reads.
#[derive(Copy, Clone)]
pub enum Register {
    InputPort0 = 0x00,
    InputPort1 = 0x01,
    OutputPort0 = 0x02,
    OutputPort1 = 0x03,
    PolarityInversionPort0 = 0x04,
    PolarityInversionPort1 = 0x05,
    ConfigurationPort0 = 0x06,
    ConfigurationPort1 = 0x07,
}

impl Register {
    ///Return the other pair member of the given register
    fn get_neighbor(&self) -> Register {
        match self {
            Self::InputPort0 => Self::InputPort1,
            Self::InputPort1 => Self::InputPort0,
            Self::OutputPort0 => Self::OutputPort1,
            Self::OutputPort1 => Self::OutputPort0,
            Self::PolarityInversionPort0 => Self::PolarityInversionPort1,
            Self::PolarityInversionPort1 => Self::PolarityInversionPort0,
            Self::ConfigurationPort0 => Self::ConfigurationPort1,
            Self::ConfigurationPort1 => Self::ConfigurationPort0,
        }
    }

    //Returns true if register is an input register
    fn is_input(&self) -> bool {
        matches!(self, Self::InputPort0 | Self::InputPort1)
    }
}

/// The gpio banks of the device
#[derive(Copy, Clone)]
pub enum GPIOBank {
    Bank0 = 0,
    Bank1 = 1,
}

/// The possible polarity states of inputs and outputs of the device
pub enum Polarity {
    Normal = 0,
    Inverse = 1,
}
