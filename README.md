# zip_codes_plus
[![Docs](https://docs.rs/zip_codes_plus/badge.svg)](https://docs.rs/zip_codes_plus)

Fork of [SkylerLipthay/zip_codes](https://github.com/SkylerLipthay/zip_codes) that is based off of the free ZIP code database provided by [federalgovernmentzipcodes.us](http://federalgovernmentzipcodes.us/) ("primary location only" dataset).

This library generates two [lookup maps](https://crates.io/crates/phf) from a CSV file at compile time. Depending on this library comes at a roughly 11MB binary and memory cost.

[Documentation](https://docs.rs/zip_codes_plus)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
