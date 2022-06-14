mod common;

use common::{ShareableI2c, ADDR, I2C_BUS};

use lazy_static::lazy_static;

use rppal::gpio::{Gpio, InputPin};
use serial_test::serial;
use std::sync::Mutex;

use pca9535::{Expander, Pca9535Cached, Register};

pub type CachedExpander = Pca9535Cached<'static, ShareableI2c, InputPin>;

lazy_static! {
    static ref INTERRUPT_PIN: InputPin = {
        let gpio = Gpio::new().unwrap();

        gpio.get(6).unwrap().into_input()
    };
    static ref EXPANDER: Mutex<CachedExpander> = {
        let i2c_bus = *I2C_BUS.lock().unwrap();
        let expander =
            Pca9535Cached::new(i2c_bus.acquire_i2c(), ADDR, &*INTERRUPT_PIN, false).unwrap();

        Mutex::new(expander)
    };
}

#[test]
#[serial(cached_std)]
fn read_write_byte() {
    let expander = &mut *EXPANDER.lock().unwrap();

    expander
        .write_byte(Register::PolarityInversionPort0, 0xCD)
        .unwrap();

    let mut buffer: u8 = 0;

    expander
        .read_byte(Register::PolarityInversionPort0, &mut buffer)
        .unwrap();

    assert_eq!(buffer, 0xCD);

    expander
        .write_byte(Register::PolarityInversionPort0, 0x00)
        .unwrap();
}

#[test]
#[serial(cached_std)]
fn read_write_halfword() {
    let expander = &mut *EXPANDER.lock().unwrap();

    expander
        .write_halfword(Register::PolarityInversionPort0, 0xABCD)
        .unwrap();

    let mut buffer: u16 = 0;

    expander
        .read_halfword(Register::PolarityInversionPort0, &mut buffer)
        .unwrap();

    assert_eq!(0xABCD, buffer);

    // Reset register to default state
    expander
        .write_halfword(Register::PolarityInversionPort0, 0x0000)
        .unwrap();
}

#[cfg(test)]
mod standard {
    use super::common::RPI_GPIO;
    use super::EXPANDER;

    use serial_test::serial;

    use pca9535::{GPIOBank, StandardExpanderInterface};

    #[test]
    #[serial(cached_std)]
    fn input_pin_is_high() {
        let expander = &mut *EXPANDER.lock().unwrap();
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

        expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();

        rpi_gpio.out1_0.set_high();

        assert!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());
        assert!(!expander.pin_is_low(GPIOBank::Bank1, 0).unwrap());
    }

    #[test]
    #[serial(cached_std)]
    fn input_pin_is_low() {
        let expander = &mut *EXPANDER.lock().unwrap();
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

        expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();

        rpi_gpio.out1_0.set_high();

        assert!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());
        assert!(!expander.pin_is_low(GPIOBank::Bank1, 0).unwrap());
    }

    #[test]
    #[serial(cached_std)]
    fn output_high() {
        let expander = &mut *EXPANDER.lock().unwrap();
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

        expander.pin_into_output(GPIOBank::Bank1, 6).unwrap();
        expander.pin_set_low(GPIOBank::Bank1, 6).unwrap();

        expander.pin_set_high(GPIOBank::Bank1, 6).unwrap();

        assert!(rpi_gpio.in1_6.is_high());
    }

    #[test]
    #[serial(cached_std)]
    fn output_low() {
        let expander = &mut *EXPANDER.lock().unwrap();
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

        expander.pin_into_output(GPIOBank::Bank1, 6).unwrap();
        expander.pin_set_high(GPIOBank::Bank1, 6).unwrap();

        expander.pin_set_low(GPIOBank::Bank1, 6).unwrap();

        assert!(rpi_gpio.in1_6.is_low());
    }

    #[test]
    #[serial(cached_std)]
    fn input_polarity_single() {
        let expander = &mut *EXPANDER.lock().unwrap();
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

        expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();
        rpi_gpio.out1_0.set_high();

        // Check internal Input register cache logic on polarity change
        expander.pin_inverse_polarity(GPIOBank::Bank1, 0).unwrap();
        expander.pin_normal_polarity(GPIOBank::Bank1, 0).unwrap();
        expander.pin_inverse_polarity(GPIOBank::Bank1, 0).unwrap();

        assert!(!expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());

        expander.pin_normal_polarity(GPIOBank::Bank1, 0).unwrap();

        assert!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());

        rpi_gpio.out1_0.set_low();

        expander.pin_inverse_polarity(GPIOBank::Bank1, 0).unwrap();

        assert!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());

        expander.pin_normal_polarity(GPIOBank::Bank1, 0).unwrap();

        assert!(!expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());
    }

    #[test]
    #[serial(cached_std)]
    fn input_polarity_all() {
        let expander = &mut *EXPANDER.lock().unwrap();
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

        expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();
        expander.pin_into_input(GPIOBank::Bank1, 1).unwrap();
        rpi_gpio.out1_0.set_high();
        rpi_gpio.out1_1.set_high();

        // Check internal Input register cache logic on polarity change
        expander.inverse_polarity().unwrap();
        expander.normal_polarity().unwrap();
        expander.inverse_polarity().unwrap();

        assert!(!expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());
        assert!(!expander.pin_is_high(GPIOBank::Bank1, 1).unwrap());

        expander.normal_polarity().unwrap();

        assert!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());
        assert!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap());

        rpi_gpio.out1_0.set_low();

        expander.inverse_polarity().unwrap();

        assert!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());
        assert!(!expander.pin_is_high(GPIOBank::Bank1, 1).unwrap());

        expander.normal_polarity().unwrap();

        assert!(!expander.pin_is_high(GPIOBank::Bank1, 0).unwrap());
        assert!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap());
    }
}

#[cfg(test)]
mod pin {
    use super::common::{Pca9535GPIO, ShareableI2c, ADDR, I2C_BUS, RPI_GPIO};

    use super::{CachedExpander, INTERRUPT_PIN};

    use lazy_static::lazy_static;
    use serial_test::serial;
    use std::sync::Mutex;

    use hal::digital::blocking::{InputPin as HalInputPin, IoPin, OutputPin as HalOutputPin};
    use pca9535::{
        ExpanderInputPin, ExpanderOutputPin, GPIOBank, IoExpander, Pca9535Cached, PinState,
    };

    type Pca9535Gpio = Mutex<
        Pca9535GPIO<
            'static,
            IoExpander<ShareableI2c, CachedExpander, Mutex<CachedExpander>>,
            ShareableI2c,
        >,
    >;

    lazy_static! {
        static ref IO_EXPANDER: IoExpander<ShareableI2c, CachedExpander, Mutex<CachedExpander>> = {
            let i2c_bus = *I2C_BUS.lock().unwrap();
            let expander =
                Pca9535Cached::new(i2c_bus.acquire_i2c(), ADDR, &*INTERRUPT_PIN, false).unwrap();

            IoExpander::new(expander)
        };
        static ref PCA9535_GPIO: Pca9535Gpio = {
            let pca9535_gpio = Pca9535GPIO {
                _in0_3: ExpanderInputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 3).unwrap(),
                in0_4: ExpanderInputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 4).unwrap(),
                _out0_7: ExpanderOutputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 7, PinState::High)
                    .unwrap(),
                out1_5: ExpanderOutputPin::new(&*IO_EXPANDER, GPIOBank::Bank1, 5, PinState::Low)
                    .unwrap(),
            };

            Mutex::new(pca9535_gpio)
        };
    }

    #[test]
    #[serial(cached_pin)]
    fn input_pin_is_high() {
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
        let pca9535_gpio = PCA9535_GPIO.lock().unwrap();

        rpi_gpio.out0_4.set_high();

        assert!(pca9535_gpio.in0_4.is_high().unwrap());
    }

    #[test]
    #[serial(cached_pin)]
    fn input_pin_is_low() {
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
        let pca9535_gpio = PCA9535_GPIO.lock().unwrap();

        rpi_gpio.out0_4.set_low();

        assert!(pca9535_gpio.in0_4.is_low().unwrap());
    }

    #[test]
    #[serial(cached_pin)]
    fn output_low() {
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
        let mut pca9535_gpio = PCA9535_GPIO.lock().unwrap();

        pca9535_gpio.out1_5.set_low().unwrap();

        assert!(rpi_gpio.in1_5.is_low());
    }

    #[test]
    #[serial(cached_pin)]
    fn output_high() {
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
        let mut pca9535_gpio = PCA9535_GPIO.lock().unwrap();

        pca9535_gpio.out1_5.set_high().unwrap();

        assert!(rpi_gpio.in1_5.is_high());
    }

    #[test]
    #[serial(cached_pin)]
    fn pin_conversion() {
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
        let input_pin = ExpanderInputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 3).unwrap();
        let output_pin =
            ExpanderOutputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 7, PinState::High).unwrap();

        let mut input_to_output = input_pin.into_output_pin(PinState::Low).unwrap();

        input_to_output.set_high().unwrap();

        assert!(rpi_gpio.in0_3.is_high());

        rpi_gpio.out0_7.set_high();

        let output_to_input = output_pin.into_input_pin().unwrap();

        assert!(output_to_input.is_high().unwrap());

        output_to_input.into_output_pin(PinState::Low).unwrap();
        input_to_output.into_input_pin().unwrap();
    }
}
