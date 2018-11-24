use super::{base64_char_to_decimal, config::Config, error::Base64Error, Base64};

impl<'a> Base64<'a> {
    /// Decode a Base64 value to it's a Vector of u8
    ///
    /// # Return:
    /// The vector of u8 corresponding to the data that was encoded into base64
    ///
    /// # Example:
    /// ```
    /// extern crate base64;
    /// use base64::{Base64, config::STANDARD};
    ///
    /// fn main() {
    ///     let word: &str = "Hello";
    ///     let mut b64 =  Base64::new_encode_bytes(word.as_bytes(), STANDARD);
    ///     let decoded_word = match String::from_utf8(b64.decode_to_bytes()) {
    ///         Ok(value) => value,
    ///         Err(e) => panic!("{}", e),
    ///     };
    ///     println!("Before = {}\nAfter = {}", word, decoded_word);
    /// }
    /// ```
    pub fn decode_to_bytes(&self) -> Vec<u8> {
        decode_bytes(self.conf, &self.to_string())
    }

    /// Loop over Base64 number convert each value to it's corresponding unsigned value and sum all
    /// of those
    ///
    /// # Return:
    /// Result with either the u128 or
    /// [Base64Error::OverflowError](error/enum.Base64Error.html#variant.OverflowError)
    ///
    /// # Example:
    /// ```
    /// extern crate base64;
    /// use base64::{Base64, config::STANDARD};
    ///
    /// fn main() {
    ///     let mut b64 =  Base64::new_encode_unsigned(&8, STANDARD);
    ///     match b64.decode_to_unsigned() {
    ///         Ok(value) => println!("{}", value),
    ///         Err(e) => println!("{}", e),
    ///     }
    /// }
    /// ```
    pub fn decode_to_unsigned(&self) -> Result<u128, Base64Error> {
        let mut dec: u128 = 0;
        // Strip padding from self.value
        let stripped_vec = remove_padding(self.conf.get_padding(), &self.value);
        for (i, ch) in stripped_vec.iter().enumerate() {
            match convert_char_to_decimal(&self.conf, *ch, (stripped_vec.len() - (i + 1)) as u32) {
                Some(val) => match dec.checked_add(val) {
                    // Check possible addition overflow
                    Some(val) => {
                        dec = val;
                    }
                    None => {
                        return Err(Base64Error::OverflowError);
                    }
                },
                None => {
                    return Err(Base64Error::OverflowError);
                }
            }
        }
        Ok(dec)
    }
}

/// Decodes a &str to a Base64 String
fn decode_bytes<'a>(conf: &'a Config, s: &str) -> Vec<u8> {
    //let mut binary: String = String::new();
    let mut binary: Vec<char> = Vec::new();
    for i in s.chars() {
        if conf.get_padding().is_some() && i == conf.get_padding().unwrap() {
            // Skip padding characters
        } else if i != ' ' && i != '\n' {
            // Skip newlines and spaces
            binary.append(
                convert_decimal_to_binary(base64_char_to_decimal(conf.get_character_set(), i))
                    .as_mut(),
            );
        }
    }
    // Add additional 0s to make sure it's divisible by 8
    while binary.len() % 8 != 0 {
        binary.push('0');
    }
    let mut v: Vec<u8> = Vec::new();
    for i in (0..binary.len()).step_by(8) {
        if !is_8bit_all_0s(&binary[i..i + 8]) {
            //Skip padding
            v.push(convert_8bit_to_u8(&binary[i..i + 8]));
        }
    }
    v
}

/// Converts a character in Base64 to it's decimal equivalent which is val * 64^place
/// Param: val, the character value
/// Param: place, the place
/// Return: Either None if any value isn't in the proper bounds or u128
fn convert_char_to_decimal(conf: &Config, val: char, place: u32) -> Option<u128> {
    match 64u128.checked_pow(place) {
        // Check pow overflow
        Some(value) => {
            match (base64_char_to_decimal(conf.get_character_set(), val)).checked_mul(value) {
                Some(val) => Some(val),
                None => None,
            }
        }
        None => None,
    }
}

/// Converts a decimal to binary by getting value % 2 then dividing by 2 until the value is 0
/// Prepend 0s until the binary is of length 6. This is in the reverse order so reverse it.
fn convert_decimal_to_binary(value: u128) -> Vec<char> {
    let mut v = value;
    let mut vec: Vec<char> = Vec::new();
    while v != 0 {
        match v % 2 {
            0 => vec.push('0'),
            1 => vec.push('1'),
            _ => vec.push('0'), // Impossible case
        }
        v /= 2;
    }
    // Prepend 0s so that it's of length 6
    while vec.len() < 6 {
        vec.push('0');
    }
    vec.reverse(); // Flip vector to proper order
    vec
}

/// Converts a 6 bit binary value to a u128
fn convert_8bit_to_u8(s: &[char]) -> u8 {
    let mut value: u8 = 0;
    for (i, c) in s.iter().enumerate() {
        // if it's 1 add 2^place
        if *c == '1' {
            value += 2u8.pow(((s.len() - 1) - i) as u32);
        }
    }
    value
}

/// Checks if all 8 bits (represented as a slice of chars) are all 0s
fn is_8bit_all_0s(s: &[char]) -> bool {
    for c in s {
        if *c != '0' {
            return false;
        }
    }
    true
}

fn remove_padding(pad: Option<char>, v: &[char]) -> Vec<char> {
    if pad.is_some() {
        let mut new_v: Vec<char> = Vec::new();
        for i in v {
            if *i != pad.unwrap() {
                new_v.push(*i);
            }
        }
        new_v
    } else {
        v.to_vec()
    }
}
