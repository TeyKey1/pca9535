extern crate embedded_hal;
extern crate lazy_static;
extern crate pca9535;
extern crate rppal;
extern crate shared_bus;

use lazy_static::lazy_static;
use shared_bus::BusManager;
use std::sync::Mutex;

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
            out1_2: gpio.get(18).unwrap().into_output_low(),
            out1_3: gpio.get(23).unwrap().into_output_low(),
            in1_4: gpio.get(24).unwrap().into_input(),
            in1_6: gpio.get(8).unwrap().into_input(),
            in1_7: gpio.get(7).unwrap().into_input(),
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
    out1_2: OutputPin,
    out1_3: OutputPin,
    in1_4: InputPin,
    in1_6: InputPin,
    in1_7: InputPin,
}

/*struct Pca9535GPIO<'a, T>
where
    T: SyncExpander,
{
    in0_3: ExpanderInputPin<'a, T>,
    in0_4: ExpanderInputPin<'a, T>,
    out0_7: ExpanderOutputPin<'a, T>,
    out1_5: ExpanderOutputPin<'a, T>,
}*/

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

    mod standard {}

    mod pin {}
}

mod cached {
    use crate::{lazy_static::lazy_static, ADDR, I2C_BUS, RPI_GPIO};

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

    mod standard {}

    mod pin {}
}
