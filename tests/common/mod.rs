use std::sync::Mutex;

use embedded_hal_bus::i2c::MutexDevice;
use hal::digital::{ErrorType as HalErrorType, InputPin as HalInputPin};
use hal::i2c::I2c as HalI2c;
use once_cell::sync::Lazy;
use pca9535::expander::SyncExpander;
use pca9535::{ExpanderInputPin, ExpanderOutputPin};
use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::i2c::I2c;

pub const ADDR: u8 = 33; //I2C address of IO Expander

pub type ShareableI2c = MutexDevice<'static, I2c>;

pub static I2C_MUTEX: Lazy<Mutex<I2c>> = Lazy::new(|| Mutex::new(I2c::new().unwrap()));
pub static RPI_GPIO: Lazy<Mutex<RpiGPIO>> = Lazy::new(|| {
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
});

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
    I2C: HalI2c,
{
    pub _in0_3: ExpanderInputPin<'a, I2C, T>,
    pub in0_4: ExpanderInputPin<'a, I2C, T>,
    pub _out0_7: ExpanderOutputPin<'a, I2C, T>,
    pub out1_5: ExpanderOutputPin<'a, I2C, T>,
}

/// embedded-hal [`HalInputPin`] which is shareable across threads using a [`Mutex`]
pub struct ShareableInputPin<'a, T> {
    pin: &'a Mutex<T>,
}

impl<'a, T> ShareableInputPin<'a, T> {
    pub fn new(pin: &'a Mutex<T>) -> Self {
        Self { pin }
    }
}

impl<'a, T> HalInputPin for ShareableInputPin<'a, T>
where
    T: HalInputPin,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.pin.lock().unwrap().is_high()
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.pin.lock().unwrap().is_low()
    }
}

impl<'a, T> HalErrorType for ShareableInputPin<'a, T>
where
    T: HalInputPin,
{
    type Error = T::Error;
}
