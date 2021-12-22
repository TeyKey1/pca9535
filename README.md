# PCA9535
PCA9535 IO-Expander driver using embedded-hal.

## Features

### Two expander modes:
- Immediate
- Cached
Immediate mode issues an i2c bus transaction on each function call, behaving like a normal i2c device library does.

Cached mode takes advantage of the interrupt pin of the device, which indicates a change in the register value. The driver holds an internal representation of the device's registers and thus it only issues a read if any data changed as indicated by the interrupt pin. This mode reduces read access on the bus significantly compared to immediate mode.

### Two ways of interacting:
- Standard Interface
- HAL Pin Interface
The standard interface offers all needed functions to interact with the GPIO pins of the device.

The HAL Pin Interface offers a way to use the Expander GPIO as embedded-hal GPIO which makes it possible to use them in any other libraries using embedded-hal. The pins are usable across threads using an ExpanderMutex. 

## Usage Example
This is a basic usage example, for more information visit the docs.

Immediate expader using standard interface:
```rust
use pca9535::{GPIOBank, Pca9535Immediate, StandardExpanderInterface};

let i2c = I2c::new().unwrap();

let mut expander = Pca9535Immediate::new(i2c, 32);

expander.pin_into_input(GPIOBank::Bank0, 4).unwrap();
expander.pin_into_output(GPIOBank::Bank1, 6).unwrap();

if expander.pin_is_high(GPIOBank::Bank0, 4).unwrap() {
    expander.pin_set_high(GPIOBank::Bank1, 6).unwrap();
}
```

Cached expander using hal pin interface:
```rust
use std::sync::Mutex;
use embedded_hal::digital::blocking::{InputPin, OutputPin};
use pca9535::{ExpanderInputPin, ExpanderOutputPin, GPIOBank, IoExpander, Pca9535Cached, PinState};

let i2c = I2c::new().unwrap();
let interrupt_pin = Gpio::new().unwrap().get(1).unwrap().into_input();

let expander = Pca9535Cached::new(i2c, 32, &interrupt_pin, true).unwrap();
let io_expander: IoExpander<Mutex<_>, _> = IoExpander::new(expander);

let input_pin = ExpanderInputPin::new(&io_expander, GPIOBank::Bank0, 4).unwrap();
let mut output_pin = ExpanderOutputPin::new(&io_expander, GPIOBank::Bank1, 6, PinState::Low).unwrap();

if input_pin.is_high().unwrap() {
    output_pin.set_high().unwrap();
}
```