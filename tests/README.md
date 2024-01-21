# Testing PCA9535 Library

The tests in this directory are integration tests, which are performed by a Raspberry Pi connected to a PCA9535 IO Expander device.

## Test organization

Types and statics required globally are defined inside the [mod.rs](./common/mod.rs)

The [cached](./cached.rs) contains all tests for cached expanders. It contains the modules `standard` and `pin` which contain the tests for the standard and hal-pin interface.
The same applies for the [immediate](./immediate.rs) expander tests.

## Developing and running tests

If you develop the tests on a different operating system than the Raspberry Pi you can verify your test code by using the custom commands `cargo checktests` or `cargo clippytests`

To run the tests on the Raspberry Pi you can use the standard `cargo test` command.

## Wiring

For more information on how to wire the Raspberry Pi and the PCA9535 for testing, please refer to the [schematics](./Schematics/pca9535_testbench)
