/*!
PCA9535 driver using embedded-hal

## Device
The PCA9535 and PCA9535C are 16-bit IO-Expanders using the I2C/SMBus interface. The devices operate at a voltage level of 2.3-5.5V

### GPIO
The expander provides two 5V tolerant GPIO banks with eight pins. Each pin is configurable separately as either input or output and additionally allows for polarity inversion.
The open-drain interrupt output of the device indicates a change if any of the input states differs from the state of the input port register.

On initialization, all pins are configured as high-impedance inputs. The PCA9535 features totem pole IOs, while the PCA9535C IOs are open-drain.
### I2C
The device uses 7Bit addressing and allows the hardware configuration of the first 3 address bits, allowing for up to 8 expanders on the same bus.

## General info
The library uses the blocking I2C embedded-hal traits. Each implementation of [`Expander`] owns the provided I2C instance.
If multiple drivers/devices need access to the same I2C bus, sharing the bus using crates like [embedded-hal-bus](https://crates.io/crates/embedded-hal-bus) is recommended.

# Usage
This library can be used in multiple ways depending on the use case and needs.

## Operation types
The device has two possible configurations on how i2c bus traffic is handled:

### Immediate
The immediate expander interface [`Pca9535Immediate`] issues an i2c bus transaction on each function call, which changes the state of the expander.
It does not make use of the open drain interrupt output of the device to reduce bus traffic and does not hold any state on the device registers.
```no_run
use rppal::i2c::I2c;
use pca9535::Pca9535Immediate;

let i2c = I2c::new().unwrap();
let address = 32;

let expander = Pca9535Immediate::new(i2c, address);
```
### Cached
The cached expander interface [`Pca9535Cached`] stores the state of the device registers internally to reduce the i2c bus traffic as much as possible.
It relies on the open drain interrupt pin of the device to detect any changes to the registers. Thus, the use of this hardware pin is mandatory for this interface.
```no_run
use rppal::i2c::I2c;
use rppal::gpio::Gpio;
use pca9535::Pca9535Cached;

let gpio = Gpio::new().unwrap();
//A HAL GPIO Input pin which is connected to the interrupt pin of the IO Expander
let expander_interrupt_pin = gpio.get(0).unwrap().into_input();

let i2c = I2c::new().unwrap();
let address = 32;

let expander = Pca9535Cached::new(i2c, address, expander_interrupt_pin, true); // create cached expander and initialize cache to defaults
```
## Usage types
Once the operation type has been determined, there are two ways of interacting with the IO expander:

### Standard Expander Interface
Every [`Expander`] implements the [`StandardExpanderInterface`]. This interface offers various functions to interact with the expander.
Those functions do not hold any state of whether the pins are currently configured as inputs or outputs. The user needs to ensure that the pins are in the desired configuration
before calling other functions to get valid and expected results.
```no_run
use rppal::i2c::I2c;
use pca9535::GPIOBank;
use pca9535::StandardExpanderInterface;
use pca9535::Pca9535Immediate;

let i2c = I2c::new().unwrap();
let address = 32;

let mut expander = Pca9535Immediate::new(i2c, address); //Either Immediate or Cached expander

expander.pin_into_output(GPIOBank::Bank0, 3).unwrap();
expander.pin_set_high(GPIOBank::Bank0, 3).unwrap();
// and so on...
```
### Expander HAL Pins
This interface offers the possibility to use the GPIO of the IO expander as [`hal`] pins, either to use for other [`hal`] librariers or just as a standardized way to handle GPIOs.
As this is a special interface which is sync and can be used across multiple threads etc. the operation types need to be wrapped into an [`IoExpander`] type.
```no_run
use std::sync::Mutex;
use rppal::i2c::I2c;
use pca9535::IoExpander;
use pca9535::Pca9535Immediate;

let i2c = I2c::new().unwrap();
let address = 32;

let mut expander = Pca9535Immediate::new(i2c, address); //Either Immediate or Cached expander

let io_expander: IoExpander<_, _, Mutex<_>> = IoExpander::new(expander); // Wrapped expander in std environment using Mutex as ExpanderMutex
```
Using this wrapper, the expander gets automatically wrapped into an [`ExpanderMutex`], which ensures exclusive access to the expander and makes it [`Sync`].
Currently, ExpanderMutex is only implemented for the `std` environment. You can activate this implementation by enabling the "std" feature of this crate. For other architectures on bare metal, etc.
The ExpanderMutex trait can be implemented on any type, which ensures exclusive access to the contained data. Once this is done, the expander can be wrapped inside an IoExpander as described previously
using the newly implemented ExpanderMutex trait.

Now, it is possible to generate either [`ExpanderInputPin`] or [`ExpanderOutputPin`] and manipulate the IO expander through those pins.
They implement all the standard [`hal`] traits on GPIO pins and could theoretically also be used in other libraries requiring hal GPIO pins.
```no_run
use std::sync::Mutex;
use rppal::i2c::I2c;
use hal::digital::{InputPin, OutputPin};
use pca9535::IoExpander;
use pca9535::Pca9535Immediate;
use pca9535::{ExpanderInputPin, ExpanderOutputPin};
use pca9535::GPIOBank::{Bank0, Bank1};
use pca9535::PinState;

let i2c = I2c::new().unwrap();
let address = 32;

let mut expander = Pca9535Immediate::new(i2c, address);

let io_expander: IoExpander<_, _, Mutex<_>> = IoExpander::new(expander);

let mut expander_pin_1_5 = ExpanderInputPin::new(&io_expander, Bank1, 5).unwrap();
let mut expander_pin_0_2 = ExpanderOutputPin::new(&io_expander, Bank0, 2, PinState::Low).unwrap();

expander_pin_0_2.set_high();
let is_high = expander_pin_1_5.is_high();
// and so on...
```
*/
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod expander;
pub mod mutex;
pub mod pin;

pub use expander::cached::Pca9535Cached;
pub use expander::immediate::Pca9535Immediate;
pub use expander::io::IoExpander;
pub use expander::standard::StandardExpanderInterface;
pub use expander::Expander;
pub use expander::ExpanderError;
pub use expander::SyncExpander;
pub use hal::digital::PinState;
pub use mutex::ExpanderMutex;
pub use pin::ExpanderInputPin;
pub use pin::ExpanderOutputPin;

/// The data registers of the device
///
/// The enum represents the command byte values used to access the corresponding registers.
///
/// # Register pairs
/// The device's registers are all 8-bit and act as four register pairs. Therefore, writing a half-word to a register results in the eight least significant bits
/// being written to the provided register, while the eight most significant bits will be automatically written to the other register of the pair.
///
/// **Pairs**
/// 1) InputPort0 and InputPort1
/// 2) OutputPort0 and Outputport1
/// 3) PolarityInversionPort0 and PolarityInversionPort1
/// 4) ConfigurationPort0 and ConfigurationPort1
///
/// Example code
/// ```no_run
/// # use rppal::i2c::I2c;
/// # use pca9535::Pca9535Immediate;
/// # use pca9535::Expander;
/// # use pca9535::Register;
/// #
/// # let i2c = I2c::new().unwrap();
/// # let address = 32;
/// #
/// # let mut expander = Pca9535Immediate::new(i2c, address);
/// #
/// expander.write_halfword(Register::OutputPort0, 0x4A07 as u16).unwrap();
///
/// let mut output_bank0: u8 = 0x00;
/// let mut output_bank1: u8 = 0x00;
///
/// expander.read_byte(Register::OutputPort0, &mut output_bank0).unwrap();
/// expander.read_byte(Register::OutputPort1, &mut output_bank1).unwrap();
///
/// assert_eq!(output_bank0, 0x4A as u8);
/// assert_eq!(output_bank1, 0x07 as u8);
/// ```
/// Or
/// ```no_run
/// # use rppal::i2c::I2c;
/// # use pca9535::Pca9535Immediate;
/// # use pca9535::Expander;
/// # use pca9535::Register;
/// #
/// # let i2c = I2c::new().unwrap();
/// # let address = 32;
/// #
/// # let mut expander = Pca9535Immediate::new(i2c, address);
/// #
/// expander.write_halfword(Register::OutputPort1, 0x4A07 as u16).unwrap();
///
/// let mut output_bank0: u8 = 0x00;
/// let mut output_bank1: u8 = 0x00;
///
/// expander.read_byte(Register::OutputPort0, &mut output_bank0).unwrap();
/// expander.read_byte(Register::OutputPort1, &mut output_bank1).unwrap();
///
/// assert_eq!(output_bank0, 0x07 as u8);
/// assert_eq!(output_bank1, 0x4A as u8);
/// ```
/// The same principle applies to reads.
#[derive(Debug, Copy, Clone)]
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
    /// Return the other pair member of the given register
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

    /// Returns true if register is an input register
    fn is_input(&self) -> bool {
        matches!(self, Self::InputPort0 | Self::InputPort1)
    }

    /// Returns true if register is a polarity inversion register
    fn is_polarity_inversion(&self) -> bool {
        matches!(
            self,
            Self::PolarityInversionPort0 | Self::PolarityInversionPort1
        )
    }
}

/// The gpio banks of the device
#[derive(Debug, Copy, Clone)]
pub enum GPIOBank {
    Bank0 = 0,
    Bank1 = 1,
}

/// The possible polarity states of inputs and outputs of the device
#[derive(Debug, Copy, Clone)]
pub enum Polarity {
    Normal = 0,
    Inverse = 1,
}
