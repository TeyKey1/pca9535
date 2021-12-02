use hal::digital::blocking::InputPin;
use hal::i2c::blocking::{Write, WriteRead};

use super::Expander;
use super::ExpanderError;
use super::Register;

#[derive(Debug)]
pub struct Pca9535Cached<I2C, IP>
where
    I2C: Write + WriteRead,
    IP: InputPin,
{
    address: u8,
    i2c: I2C,
    interrupt_pin: IP,

    input_port_0: u8,
    input_port_1: u8,
    output_port_0: u8,
    output_port_1: u8,
    polarity_inversion_port_0: u8,
    polarity_inversion_port_1: u8,
    configuration_port_0: u8,
    configuration_port_1: u8,
}

impl<I2C: Write + WriteRead, IP: InputPin> Pca9535Cached<I2C, IP> {
    ///Creates a new cached PCA9535 instance.
    ///
    /// The cached registers are initialized to the devices default state.
    ///
    /// # Panics
    /// If given device hardware address is outside of the permittable range of `32-39`.
    pub fn new(i2c: I2C, address: u8, interrupt_pin: IP) -> Self {
        assert!(address > 31 && address < 40);

        Self {
            address,
            i2c,
            interrupt_pin,
            input_port_0: 0x00,
            input_port_1: 0x00,
            output_port_0: 0xFF,
            output_port_1: 0xFF,
            polarity_inversion_port_0: 0x00,
            polarity_inversion_port_1: 0x00,
            configuration_port_0: 0xFF,
            configuration_port_1: 0xFF,
        }
    }

    pub fn init_cache(
        &mut self,
    ) -> Result<(), ExpanderError<<I2C as WriteRead>::Error, <I2C as Write>::Error>> {
        todo!()
    }

    fn get_cached(&self, register: Register) -> u8 {
        match register {
            Register::InputPort0 => self.input_port_0,
            Register::InputPort1 => self.input_port_1,
            Register::OutputPort0 => self.output_port_0,
            Register::OutputPort1 => self.output_port_1,
            Register::PolarityInversionPort0 => self.polarity_inversion_port_0,
            Register::PolarityInversionPort1 => self.polarity_inversion_port_1,
            Register::ConfigurationPort0 => self.configuration_port_0,
            Register::ConfigurationPort1 => self.configuration_port_1,
        }
    }

    fn set_cached(&mut self, register: Register, value: u8) {
        match register {
            Register::InputPort0 => self.input_port_0 = value,
            Register::InputPort1 => self.input_port_1 = value,
            Register::OutputPort0 => self.output_port_0 = value,
            Register::OutputPort1 => self.output_port_1 = value,
            Register::PolarityInversionPort0 => self.polarity_inversion_port_0 = value,
            Register::PolarityInversionPort1 => self.polarity_inversion_port_1 = value,
            Register::ConfigurationPort0 => self.configuration_port_0 = value,
            Register::ConfigurationPort1 => self.configuration_port_1 = value,
        };
    }
}

impl<I2C: Write + WriteRead, IP: InputPin> Expander for Pca9535Cached<I2C, IP> {
    type Error = ExpanderError<<I2C as WriteRead>::Error, <I2C as Write>::Error>;

    /// Writes one byte to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Cached
    /// If the bus write succeeds the written data is cached to avoid the need for bus traffic upon reading the written register.
    fn write_byte(&mut self, register: Register, data: u8) -> Result<(), Self::Error> {
        self.i2c
            .write(self.address, &[register as u8, data])
            .map_err(Self::Error::from_write)?;

        self.set_cached(register, data);
        Ok(())
    }

    /// Reads one byte of given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Cached
    /// This function only creates bus traffic in case the provided interrupt pin is held at a `low` voltage level at the time of the function call and the provided register is an input register. In that case the data is being read from the device, as the devices interrupt output indicates a data change. Otherwise the cached value is returned without causing any bus traffic.
    fn read_byte(&mut self, register: Register, buffer: &mut u8) -> Result<(), Self::Error> {
        if self.interrupt_pin.is_low().unwrap() && register.is_input() {
            let mut buf = [0u8];

            self.i2c
                .write_read(self.address, &[register as u8], &mut buf)
                .map_err(Self::Error::from_write_read)?;

            *buffer = buf[0];
        } else {
            *buffer = self.get_cached(register);
        }

        Ok(())
    }

    /// Writes one halfword to given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Register pairs
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    ///
    /// # Cached
    /// If the bus write succeeds the written data is cached to avoid the need for bus traffic upon reading the written register.
    fn write_halfword(&mut self, register: Register, data: u16) -> Result<(), Self::Error> {
        self.i2c
            .write(
                self.address,
                &[register as u8, (data >> 8) as u8, data as u8],
            )
            .map_err(Self::Error::from_write)?;

        self.set_cached(register, (data >> 8) as u8);
        self.set_cached(register.get_neighbor(), data as u8);

        Ok(())
    }

    /// Reads one halfword of given register
    ///
    /// Only use this function if you really have to. The crate provides simpler ways of interacting with the device for most usecases.
    ///
    /// # Register pairs
    /// please see [`Register`] for more information about the register pairs and how they affect the halfword read and write functions.
    ///
    /// # Cached
    /// This function only creates bus traffic in case the provided interrupt pin is held at a `low` voltage level at the time of the function call and the provided register is an input register. In that case the data is being read from the device, as the devices interrupt output indicates a data change. Otherwise the cached value is returned without causing any bus traffic.
    fn read_halfword(&mut self, register: Register, buffer: &mut u16) -> Result<(), Self::Error> {
        let mut reg_val: [u8; 2] = [0x00; 2];

        if self.interrupt_pin.is_low().unwrap() && register.is_input() {
            self.i2c
                .write_read(self.address, &[register as u8], &mut reg_val)
                .map_err(Self::Error::from_write_read)?;

            self.set_cached(register, reg_val[0]);
            self.set_cached(register.get_neighbor(), reg_val[1]);

            *buffer = (reg_val[0] as u16) << 8 & reg_val[1] as u16;
        } else {
            *buffer = (self.get_cached(register) as u16) << 8
                & self.get_cached(register.get_neighbor()) as u16;
        }

        Ok(())
    }
}
