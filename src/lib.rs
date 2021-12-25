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
The library uses the blocking I2C embedded-hal traits. Each implementation of [`Expander`] owns the provided I2C instance, if multiple device access to the bus is required the user has to provide the code to make it work. No synchronization is done inside the library.
For this purpose it is recommended to use crates like [shared-bus](https://crates.io/crates/shared-bus)

# Usage
This library can be used in multiple ways depending on the use case and needs.

## Operation types
The device has two possible configurations on how i2c bus traffic is handled:

### Immediate
The immediate expander interface [`Pca9535Immediate`] issues a i2c bus transaction on each function call a state change of the expander. It does not make use of the open drain interrupt output of the device to reduce bus traffic and does not hold any state on the device registers.
```ignore
use pca9535::Pca9535Immediate;

let expander = Pca9535Immediate::new(i2c, address);
```
### Cached
The cached expander interface [`Pca9535Cached`] stores the state of the devices registers internally in order to reduce the i2c bus traffic as much as much as possible. It relies on the open drain interrupt pin of the device to detect any changes to the registers. Thus, the use of this hardware pin is mandatory for this interface.
```ignore
use pca9535::Pca9535Cached;

let expander_interrupt_pin = ...; //A HAL GPIO Input pin which is connected to the interrupt pin of the IO Expander
let expander = Pca9535Cached::new(i2c, address, &expander_interrupt_pin, true); // create cached expander and initialize cache to defaults
```
## Usage types
Once the operation type has been determined there are two ways of interacting with the IO expander:

### Standard Expander Interface
Every [`Expander`] implements the [`StandardExpanderInterface`]. This interface offers various functions to interact with the expander. Those functions do not hold any state of wether the pins are currently configured as inputs or outputs. The user needs to ensure that the pins are in the desired configuration before calling other functions in order to get valid and expected results.
```ignore
use pca9535::GPIOBank;
use pca9535::StandardExpanderInterface;

let mut expander = ...; //Either Immediate or Cached expander

expander.pin_into_output(GPIOBank::Bank0, 3).unwrap();
expander.pin_set_high(GPIOBank::Bank0, 3).unwrap();
// and so on...
```
### Expander HAL Pins
This interface offers the possibility to use the GPIO of the IO expander as [`hal`] pins, either to use for other [`hal`] librariers or just as a standardized way to handle GPIOs. As this is a special interface which is sync and can be used across multiple threads etc. the operation types need to be wrapped into an [`IoExpander`] type.
```ignore
use pca9535::IoExpander;
use std::sync::Mutex;

let expander = ...; //Either Immediate or Cached expander

let io_expander = IoExpander<Mutex<_>, _> = IoExpander::new(expander); // Wrapped expander in std environment using Mutex as ExpanderMutex
```
By using this wrapper, the expander gets automatically wrapped into an [`ExpanderMutex`] which ensures exclusive access to the expander and makes it [`Sync`]. Currently ExpanderMutex is only implemented for `std` environment. You can activate this implementation by enabling the "std" feature of this crate. For other architectures on bare metal etc. the ExpanderMutex trait can be implemented on any type which ensures exclusive access to the contained data. Once this is done the expander can be wrapped inside a IoExpander as described previously using the newly implemented ExpanderMutex trait.

Now it is possible to generate either [`ExpanderInputPin`] or [`ExpanderOutputPin`] and manipulate the IO expander through those pins. They implement all the standard [`hal`] traits on GPIO pins and could theoretically also be used in other libraries requiring hal GPIO pins.
```ignore
use pca9535::{ExpanderInputPin, ExpanderOutputPin};
use pca9535::GPIOBank::{Bank0, Bank1};
use pca9535::PinState;

let io_expander = ...; // Wrapped expander

let mut expander_pin_1_5 = ExpanderInputPin::new(&io_expander, Bank1, 5).unwrap();
let mut expander_pin_0_2 = ExpanderOutputPin::new(&io_expander, Bank0, 2, PinState::Low).unwrap();

expander_pin_0_2.set_high();
expander_pin_1_5.into_output_pin(PinState::Low);
// and so on...
```
*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate embedded_hal as hal;

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
/// The registers of the device are all 8 bit and act as four register pairs. Therefore, writing a halfword to a register results in the 8 least significant bits being written to the provided register, while the 8 most significant bits will be automatically written to the other register of the pair.
///
/// **Pairs**
/// 1) InputPort0 and InputPort1
/// 2) OutputPort0 and Outputport1
/// 3) PolarityInversionPort0 and PolarityInversionPort1
/// 4) ConfigurationPort0 and ConfigurationPort1
///
/// Example code
/// ```ignore
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
/// ```ignore
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
