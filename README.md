# imei
An IMEI number validator implemented in Rust.

```toml
imei = "1"
```

## Example
Basic example:
```rust
fn main() {
    let num = "490154203237518";
    let valid = imei::valid(num);

    println!("{num}: {valid}");
}
```

Result:
```toml
490154203237518: true
```

## Speed
This validator is designed to be as fast and efficient as possible.
It uses small number types, precalculates the character conversions, and only iterates through the imei number once.
The speed test in the tests directory gets the average speed of validation over 10,000,000 cycles.
Typically, the validation process sub 1000th of a milisecond (practically instant!).

## Features
 - The `std` is enabled by default which implements `error::Error` for `imei::Error`;
 - The `serde` feature adds serialization/deserialization for the `Imei` struct.