extern crate embedded_hal;
extern crate lazy_static;
extern crate pca9535; //How to nable feature "std" of this crate which is being tested?
extern crate rppal;
extern crate shared_bus;

use lazy_static::lazy_static;
use shared_bus::BusManager;
use std::sync::Mutex;

use pca9535::expander::SyncExpander;
use pca9535::{ExpanderInputPin, ExpanderOutputPin};
use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::i2c::I2c;

lazy_static! {
    static ref I2C_BUS: Mutex<&'static BusManager<Mutex<I2c>>> = {
        let i2c = I2c::new().unwrap();
        let i2c_bus: &'static _ = shared_bus::new_std!(I2c = i2c).unwrap();

        Mutex::new(i2c_bus)
    };
    static ref RPI_GPIO: Mutex<RpiGPIO> = {
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

const ADDR: u8 = 33; //I2C address of IO Expander

struct RpiGPIO {
    in0_3: InputPin,
    out0_4: OutputPin,
    out0_7: OutputPin,
    in1_5: InputPin,
    out1_0: OutputPin,
    out1_1: OutputPin,
    _out1_2: OutputPin,
    _out1_3: OutputPin,
    _in1_4: InputPin,
    in1_6: InputPin,
    _in1_7: InputPin,
}

struct Pca9535GPIO<'a, T>
where
    T: SyncExpander,
{
    _in0_3: ExpanderInputPin<'a, T>,
    in0_4: ExpanderInputPin<'a, T>,
    _out0_7: ExpanderOutputPin<'a, T>,
    out1_5: ExpanderOutputPin<'a, T>,
}

#[cfg(test)]
mod immediate {
    use crate::{lazy_static::lazy_static, ADDR, I2C_BUS};

    use rppal::i2c::I2c;
    use shared_bus::I2cProxy;
    use std::sync::Mutex;

    use pca9535::{Expander, Pca9535Immediate, Register};

    lazy_static! {
        static ref EXPANDER: Mutex<Pca9535Immediate<I2cProxy<'static, Mutex<I2c>>>> = {
            let i2c_bus = *I2C_BUS.lock().unwrap();
            let expander = Pca9535Immediate::new(i2c_bus.acquire_i2c(), ADDR);

            Mutex::new(expander)
        };
    }

    #[test]
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
    }

    #[test]
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
        use super::EXPANDER;
        use crate::RPI_GPIO;

        use pca9535::{GPIOBank, StandardExpanderInterface};

        #[test]
        fn input_pin_is_high() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();

            rpi_gpio.out1_0.set_high();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);
            assert_eq!(expander.pin_is_low(GPIOBank::Bank1, 0).unwrap(), false);
        }

        #[test]
        fn input_pin_is_low() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();

            rpi_gpio.out1_0.set_high();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);
            assert_eq!(expander.pin_is_low(GPIOBank::Bank1, 0).unwrap(), false);
        }

        #[test]
        fn output_high() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_output(GPIOBank::Bank1, 6).unwrap();
            expander.pin_set_low(GPIOBank::Bank1, 6).unwrap();

            expander.pin_set_high(GPIOBank::Bank1, 6).unwrap();

            assert_eq!(rpi_gpio.in1_6.is_high(), true);
        }

        #[test]
        fn output_low() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_output(GPIOBank::Bank1, 6).unwrap();
            expander.pin_set_high(GPIOBank::Bank1, 6).unwrap();

            expander.pin_set_low(GPIOBank::Bank1, 6).unwrap();

            assert_eq!(rpi_gpio.in1_6.is_low(), true);
        }

        #[test]
        fn input_polarity_single() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();
            rpi_gpio.out1_0.set_high();

            expander.pin_inverse_polarity(GPIOBank::Bank1, 0).unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), false);

            expander.pin_normal_polarity(GPIOBank::Bank1, 0).unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);

            rpi_gpio.out1_0.set_low();

            expander.pin_inverse_polarity(GPIOBank::Bank1, 0).unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);

            expander.pin_normal_polarity(GPIOBank::Bank1, 0).unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), false);
        }

        #[test]
        fn input_polarity_all() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();
            expander.pin_into_input(GPIOBank::Bank1, 1).unwrap();
            rpi_gpio.out1_0.set_high();
            rpi_gpio.out1_1.set_high();

            expander.inverse_polarity().unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), false);
            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap(), false);

            expander.normal_polarity().unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);
            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap(), true);

            rpi_gpio.out1_0.set_low();

            expander.inverse_polarity().unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);
            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap(), false);

            expander.normal_polarity().unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), false);
            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap(), true);
        }
    }

    #[cfg(test)]
    mod pin {}
}

#[cfg(test)]
mod cached {
    use crate::{lazy_static::lazy_static, ADDR, I2C_BUS};

    use rppal::{
        gpio::{Gpio, InputPin},
        i2c::I2c,
    };
    use shared_bus::I2cProxy;
    use std::sync::Mutex;

    use pca9535::{Expander, Pca9535Cached, Register};

    lazy_static! {
        static ref EXPANDER: Mutex<Pca9535Cached<I2cProxy<'static, Mutex<I2c>>, InputPin>> = {
            let gpio = Gpio::new().unwrap();
            let i2c_bus = *I2C_BUS.lock().unwrap();
            let expander = Pca9535Cached::new(
                i2c_bus.acquire_i2c(),
                ADDR,
                gpio.get(6).unwrap().into_input(),
                false,
            )
            .unwrap();

            Mutex::new(expander)
        };
    }

    #[test]
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
    }

    #[test]
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
        use super::EXPANDER;
        use crate::RPI_GPIO;

        use pca9535::{GPIOBank, StandardExpanderInterface};

        #[test]
        fn input_pin_is_high() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();

            rpi_gpio.out1_0.set_high();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);
            assert_eq!(expander.pin_is_low(GPIOBank::Bank1, 0).unwrap(), false);
        }

        #[test]
        fn input_pin_is_low() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();

            rpi_gpio.out1_0.set_high();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);
            assert_eq!(expander.pin_is_low(GPIOBank::Bank1, 0).unwrap(), false);
        }

        #[test]
        fn output_high() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_output(GPIOBank::Bank1, 6).unwrap();
            expander.pin_set_low(GPIOBank::Bank1, 6).unwrap();

            expander.pin_set_high(GPIOBank::Bank1, 6).unwrap();

            assert_eq!(rpi_gpio.in1_6.is_high(), true);
        }

        #[test]
        fn output_low() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_output(GPIOBank::Bank1, 6).unwrap();
            expander.pin_set_high(GPIOBank::Bank1, 6).unwrap();

            expander.pin_set_low(GPIOBank::Bank1, 6).unwrap();

            assert_eq!(rpi_gpio.in1_6.is_low(), true);
        }

        #[test]
        fn input_polarity_single() {
            let expander = &mut *EXPANDER.lock().unwrap();
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();

            expander.pin_into_input(GPIOBank::Bank1, 0).unwrap();
            rpi_gpio.out1_0.set_high();

            // Check internal Input register cache logic on polarity change
            expander.pin_inverse_polarity(GPIOBank::Bank1, 0).unwrap();
            expander.pin_normal_polarity(GPIOBank::Bank1, 0).unwrap();
            expander.pin_inverse_polarity(GPIOBank::Bank1, 0).unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), false);

            expander.pin_normal_polarity(GPIOBank::Bank1, 0).unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);

            rpi_gpio.out1_0.set_low();

            expander.pin_inverse_polarity(GPIOBank::Bank1, 0).unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);

            expander.pin_normal_polarity(GPIOBank::Bank1, 0).unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), false);
        }

        #[test]
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

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), false);
            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap(), false);

            expander.normal_polarity().unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);
            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap(), true);

            rpi_gpio.out1_0.set_low();

            expander.inverse_polarity().unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), true);
            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap(), false);

            expander.normal_polarity().unwrap();

            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 0).unwrap(), false);
            assert_eq!(expander.pin_is_high(GPIOBank::Bank1, 1).unwrap(), true);
        }
    }

    #[cfg(test)]
    mod pin {
        use crate::{lazy_static::lazy_static, Pca9535GPIO, ADDR, I2C_BUS, RPI_GPIO};

        use embedded_hal::digital::blocking::{
            InputPin as HalInputPin, IoPin, OutputPin as HalOutputPin,
        };
        use pca9535::{
            expander::SyncExpander, ExpanderInputPin, ExpanderOutputPin, GPIOBank, IoExpander,
            Pca9535Cached, PinState,
        };
        use rppal::{
            gpio::{Gpio, InputPin},
            i2c::I2c,
        };
        use shared_bus::I2cProxy;
        use std::sync::Mutex;

        lazy_static! {
            static ref IO_EXPANDER: IoExpander<
                Mutex<Pca9535Cached<I2cProxy<'static, Mutex<I2c>>, InputPin>>,
                Pca9535Cached<I2cProxy<'static, Mutex<I2c>>, InputPin>,
            > = {
                let gpio = Gpio::new().unwrap();
                let i2c_bus = *I2C_BUS.lock().unwrap();
                let expander = Pca9535Cached::new(
                    i2c_bus.acquire_i2c(),
                    ADDR,
                    gpio.get(6).unwrap().into_input(),
                    false,
                )
                .unwrap();

                let exp = IoExpander::new(expander);

                &exp.write_halfword(pca9535::Register::ConfigurationPort0, 0x00);

                exp
            };
            static ref PCA9535_GPIO: Mutex<
                Pca9535GPIO<
                    'static,
                    IoExpander<
                        Mutex<Pca9535Cached<I2cProxy<'static, Mutex<I2c>>, InputPin>>,
                        Pca9535Cached<I2cProxy<'static, Mutex<I2c>>, InputPin>,
                    >,
                >,
            > = {
                let pca9535_gpio = Pca9535GPIO {
                    _in0_3: ExpanderInputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 3).unwrap(),
                    in0_4: ExpanderInputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 4).unwrap(),
                    _out0_7: ExpanderOutputPin::new(
                        &*IO_EXPANDER,
                        GPIOBank::Bank0,
                        7,
                        PinState::High,
                    )
                    .unwrap(),
                    out1_5: ExpanderOutputPin::new(
                        &*IO_EXPANDER,
                        GPIOBank::Bank1,
                        5,
                        PinState::Low,
                    )
                    .unwrap(),
                };

                Mutex::new(pca9535_gpio)
            };
        }

        #[test]
        fn input_pin_is_high() {
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
            let pca9535_gpio = PCA9535_GPIO.lock().unwrap();

            rpi_gpio.out0_4.set_high();

            assert_eq!(pca9535_gpio.in0_4.is_high().unwrap(), true);
        }

        #[test]
        fn input_pin_is_low() {
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
            let pca9535_gpio = PCA9535_GPIO.lock().unwrap();

            rpi_gpio.out0_4.set_low();

            assert_eq!(pca9535_gpio.in0_4.is_low().unwrap(), true);
        }

        #[test]
        fn output_low() {
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
            let mut pca9535_gpio = PCA9535_GPIO.lock().unwrap();

            pca9535_gpio.out1_5.set_low().unwrap();

            assert_eq!(rpi_gpio.in1_5.is_low(), true);
        }

        #[test]
        fn output_high() {
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
            let mut pca9535_gpio = PCA9535_GPIO.lock().unwrap();

            pca9535_gpio.out1_5.set_high().unwrap();

            assert_eq!(rpi_gpio.in1_5.is_high(), true);
        }

        #[test]
        fn pin_conversion() {
            let rpi_gpio = &mut *RPI_GPIO.lock().unwrap();
            let input_pin = ExpanderInputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 3).unwrap();
            let output_pin =
                ExpanderOutputPin::new(&*IO_EXPANDER, GPIOBank::Bank0, 7, PinState::High).unwrap();

            let mut input_to_output = input_pin.into_output_pin(PinState::Low).unwrap();

            input_to_output.set_high().unwrap();

            assert_eq!(rpi_gpio.in0_3.is_high(), true);

            rpi_gpio.out0_7.set_high();

            let output_to_input = output_pin.into_input_pin().unwrap();

            assert_eq!(output_to_input.is_high().unwrap(), true);

            output_to_input.into_output_pin(PinState::Low).unwrap();
            input_to_output.into_input_pin().unwrap();
        }
    }
}
