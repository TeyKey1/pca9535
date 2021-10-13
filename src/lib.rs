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
The library uses the blocking I2C embedded-hal traits.

# Usage
Usage
*/

#![no_std]

extern crate embedded_hal as hal;
use hal::i2c::blocking::{Write, WriteRead};

pub struct Pca9535<'a, I2C>
where
    I2C: Write + WriteRead,
{
    address: u8,
    i2c: &'a mut I2C,
}

impl<'a, I2C, E> Pca9535<'a, I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(addr: u8, i2c: &'a mut I2C) -> Self {
        Self {
            address: addr,
            i2c: i2c,
        }
    }

    pub fn write_byte(&mut self, register: Register, data: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[register as u8, data])
    }

    pub fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), E> {
        self.i2c
            .write_read(self.address, &[register as u8], &mut [*buffer])
    }
}

/// The data registers of the device
///
/// The enum represents the command byte values used to access the corresponding registers.
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
