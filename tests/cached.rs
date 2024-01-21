mod common;

use common::{ShareableI2c, ADDR, I2C_MUTEX};

use once_cell::sync::Lazy;

use embedded_hal_bus::i2c::MutexDevice;
use rppal::gpio::{Gpio, InputPin};
use serial_test::serial;
use std::sync::Mutex;

use pca9535::{Expander, Pca9535Cached, Register};

use crate::common::ShareableInputPin;

pub type CachedExpander = Pca9535Cached<ShareableI2c, ShareableInputPin<'static, InputPin>>;

static INTERRUPT_PIN: Lazy<Mutex<InputPin>> = Lazy::new(|| {
    let gpio = Gpio::new().unwrap();

    Mutex::new(gpio.get(6).unwrap().into_input())
});

static EXPANDER: Lazy<Mutex<CachedExpander>> = Lazy::new(|| {
    let expander = Pca9535Cached::new(
        MutexDevice::new(&I2C_MUTEX),
        ADDR,
        ShareableInputPin::new(&INTERRUPT_PIN),
        false,
    )
    .unwrap();

    Mutex::new(expander)
});

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
    use super::common::{Pca9535GPIO, ShareableI2c, ShareableInputPin, ADDR, I2C_MUTEX, RPI_GPIO};

    use super::{CachedExpander, INTERRUPT_PIN};

    use embedded_hal_bus::i2c::MutexDevice;
    use once_cell::sync::Lazy;
    use serial_test::serial;
    use std::sync::Mutex;

    use hal::digital::{InputPin as HalInputPin, OutputPin as HalOutputPin};
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

    static IO_EXPANDER: Lazy<IoExpander<ShareableI2c, CachedExpander, Mutex<CachedExpander>>> =
        Lazy::new(|| {
            let expander = Pca9535Cached::new(
                MutexDevice::new(&I2C_MUTEX),
                ADDR,
                ShareableInputPin::new(&INTERRUPT_PIN),
                false,
            )
            .unwrap();

            IoExpander::new(expander)
        });
    static PCA9535_GPIO: Lazy<Pca9535Gpio> = Lazy::new(|| {
        let pca9535_gpio = Pca9535GPIO {
            _in0_3: ExpanderInputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 3).unwrap(),
            in0_4: ExpanderInputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 4).unwrap(),
            _out0_7: ExpanderOutputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 7, PinState::High)
                .unwrap(),
            out1_5: ExpanderOutputPin::new(&*IO_EXPANDER, GPIOBank::Bank1, 5, PinState::Low)
                .unwrap(),
        };

        Mutex::new(pca9535_gpio)
    });

    #[test]
    #[serial(cached_pin)]
    fn input_pin_is_high() {
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
        let mut pca9535_gpio = PCA9535_GPIO.lock().unwrap();

        rpi_gpio.out0_4.set_high();

        assert!(pca9535_gpio.in0_4.is_high().unwrap());
    }

    #[test]
    #[serial(cached_pin)]
    fn input_pin_is_low() {
        let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
        let mut pca9535_gpio = PCA9535_GPIO.lock().unwrap();

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
}
