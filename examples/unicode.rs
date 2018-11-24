//! Uses unicode characters in order to encode and decode both unsigned and strings into and from
//! Base64. This code panics on errors just to show an example.
extern crate base64; // Include base64 crate

use base64::{config::Config, Base64};

fn main() {
    let character_set = &[
        '😀', '😃', '😄', '😁', '😆', '😅', '😂', '☺', '😊', '😇', '🙃',
        '😉', '😌', '😍', '😘', '😗', '😙', '😚', '😋', '😛', '😝', '😡',
        '😠', '😭', '😢', '😩', '😫', '😖', '😣', '☹', '😈', '😮', '😲',
        '😴', '😺', '😸', '😻', '😹', '😼', '😽', '🙀', '😿', '😾', '😱',
        '😨', '😰', '😎', '🌕', '🌖', '🌗', '🌘', '🌑', '🌒', '🌓', '🌔',
        '☃', '☔', '☂', '🐭', '❤', '▶', '☘', '☀', '⚡',
    ];
    let conf;
    match Config::new(character_set, Some('\u{22D4}'), Some(6)) {
        Ok(config) => conf = config,
        Err(e) => panic!("{}", e), // Panics unecessarily can catch and handle this error
    }; // Create config, panicing on errors

    let s: &str = "Unicode Base64";
    let b64 = Base64::new_encode_bytes(s.as_bytes(), &conf);
    println!("Encoded:\n{}\n", b64);
    match String::from_utf8(b64.decode_to_bytes()) {
        // Match on from_utf8
        Ok(value) => println!("Decoded: \"{}\"\n", value),
        Err(e) => panic!("{}", e),
    }

    let x = 10;
    println!("Number to encode: {}", x);
    let b64_2 = Base64::new_encode_unsigned(&x, &conf); // Reuse previous config
    println!("New Unsigned encoded = {}", b64_2);
    match b64_2.decode_to_unsigned() {
        Ok(value) => println!("Unsigned decoded = {}", value),
        Err(e) => println!("{}", e), // In this case since we know the value is 10, panicing is unecessary, but when the length of the base64 number is greater than 21 u128 overflow is almost certain
    }
}
