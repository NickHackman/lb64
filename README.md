# lb64

----
A Rust library for Base64 encoding and decoding unsigned integers and bytes.

lb64 strives to be panic and unsafe free with minimal dependencies and excellent documentation.

lb64 provides the ability to both utilize common Base64 configurations, such as `IMAP`, `STANDARD`, `MIME`, and `Base64url` and also provides the utilities to create your own Base64 configuration. It also creates a full fledged Base64 type and creation of a random Base64 number of n length.

See more information on [Wikipedia](https://en.wikipedia.org/wiki/Base64)

**Note**: this crate is **Nightly** only.
*Due to the current use of [overflowing_pow](https://doc.rust-lang.org/nightly/std/primitive.u128.html#method.overflowing_pow) for unsigned integer decoding*

## Why
This library is a fairly simple implementation in order to properly learn how to write code and documentation in Rust and is my first project in Rust. Pull requests are welcome and appreciated for learning purposes.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lb64 = "0.1.0"
```

Add this to your `src/main.rs` or `src/lib.rs`:
```rust
extern crate lb64;
```

## Example
```Rust
extern crate lb64;

use lb64::Base64;
use lb64::config::{Config, MIME};

fn main() {
    let s: &str = "Hello!";
    let b64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
    println!("{}", b64);
    let mut v: u128 = 0;
    match lb64.decode_to_unsigned() {
         Ok(value) => v = value,
         Err(e) => println!("{}", e),
    }
    let lb64_other = Base64::new_encode_unsigned(&v, MIME);
    if lb64_other == b64 {
         println!("They're equal!");
    }
    match String::from_utf8(lb64.decode_to_bytes()) {
         Ok(value) => println!("{}", value), // prints Hello
         Err(e) => println!("{}", e),
    }
 }
```

[Documentation](https://docs.rs/lb64/0.1.0/lb64/)

## License
lb64 is distributed under the [GNU General Public License Version 3](https://www.gnu.org/licenses/gpl-3.0.en.html)
