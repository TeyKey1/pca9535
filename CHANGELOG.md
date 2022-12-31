# 1.2.0
**Breaking changes!**
- Updated to embedded hal 1.0.0-alpha.9 (@MajorArkwolf)
- Removed IOPin (currently not supported in embedded hal 1.0.0-alpha.9, will be readded once it is supported again) (@MajorArkwolf)

Thanks to @MajorArkwolf :)

# 1.1.0
- Added `Debug` trait implementation for all types which are accessible by the library user
- Added `Clone` and `Copy` trait implementation for `Polarity` enum
- Internal code cleanup

# 1.0.0
**Breaking changes!**
- Refactored error types and generics. The whole error handling is now simpler and should make more sense in general, as underlying embedded-hal errors are directly passed to the `ExpanderError` enum. Due to those changes certain types need an additional generic for I2C interface. Generics have been refactored as well so some generics are not on the same position like they used to be. Migrating to 1.0 should be relatively simple by adding those missing generics or rearranging them.
- Added `std::error::Error` trait implementation for `ExpanderError`. This is automatically enabled by using the crates `std` feature. The change should allow for easier error handling with existing std solutions and libraries.
- Updated crate to rust 2021 edition

# 0.1.0
- moved the special implementation of writes to any polarity register to the Expander Trait implementation instead of overwriting StandardExpanderInterface Trait functions
