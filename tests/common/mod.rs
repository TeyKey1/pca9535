extern crate embedded_hal;
extern crate lazy_static;
extern crate pca9535;
extern crate rppal;
extern crate shared_bus;

use embedded_hal::i2c::blocking::{Write, WriteRead};
use lazy_static::lazy_static;
use shared_bus::{BusManager, I2cProxy};
use std::sync::Mutex;

use pca9535::expander::SyncExpander;
use pca9535::{ExpanderInputPin, ExpanderOutputPin};
use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::i2c::I2c;

pub const ADDR: u8 = 33; //I2C address of IO Expander

pub type ShareableI2c = I2cProxy<'static, Mutex<I2c>>;

lazy_static! {
    pub static ref I2C_BUS: Mutex<&'static BusManager<Mutex<I2c>>> = {
        let i2c = I2c::new().unwrap();
        let i2c_bus: &'static _ = shared_bus::new_std!(I2c = i2c).unwrap();

        Mutex::new(i2c_bus)
    };
    pub static ref RPI_GPIO: Mutex<RpiGPIO> = {
        let gpio = Gpio::new().unwrap();

        let rpi_gpio = RpiGPIO {
            in0_3: gpio.get(10).unwrap().into_input(),
            out0_4: gpio.get(22).unwrap().into_output_low(),
            out0_7: gpio.get(4).unwrap().into_output_low(),
            in1_5: gpio.get(25).unwrap().into_input(),
            out1_0: gpio.get(14).unwrap().into_output_low(),
            out1_1: gpio.get(15).unwrap().into_output_low(),
            _out1_2: gpio.get(18).unwrap().into_output_low(),
            _out1_3: gpio.get(23).unwrap().into_output_low(),
            _in1_4: gpio.get(24).unwrap().into_input(),
            in1_6: gpio.get(8).unwrap().into_input(),
            _in1_7: gpio.get(7).unwrap().into_input(),
        };

        Mutex::new(rpi_gpio)
    };
}

pub struct RpiGPIO {
    pub in0_3: InputPin,
    pub out0_4: OutputPin,
    pub out0_7: OutputPin,
    pub in1_5: InputPin,
    pub out1_0: OutputPin,
    pub out1_1: OutputPin,
    pub _out1_2: OutputPin,
    pub _out1_3: OutputPin,
    pub _in1_4: InputPin,
    pub in1_6: InputPin,
    pub _in1_7: InputPin,
}

pub struct Pca9535GPIO<'a, T, I2C>
where
    T: SyncExpander<I2C>,
    I2C: Write + WriteRead,
{
    pub _in0_3: ExpanderInputPin<'a, I2C, T>,
    pub in0_4: ExpanderInputPin<'a, I2C, T>,
    pub _out0_7: ExpanderOutputPin<'a, I2C, T>,
    pub out1_5: ExpanderOutputPin<'a, I2C, T>,
}
