# imei
An IMEI number validator implemented in Rust.

Add to Cargo.toml:
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
This validator is designed to be as fast and efficient as possible, it uses small number types, precalculates the character conversions and only iterates through the imei number once. The speed test in the tests directory gets the average speed of validation over 10,000,000 validations. Typically, the validation process sub 100th of a milisecond (practically instant).
