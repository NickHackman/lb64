//! Focuses on making a Base64 number as a type, with extensive documentation, and never panicing
//! code
//!
//! Base64 Library that has encoding and decoding of unsigned decimal values and bytes
//! Along with a full fledged Base64 type in order to store the input of encoding an unsigned
//! decimal value or bytes. Base64 type implements Clone, Eqs, Debug, along with multiple
//! constructors for creation, all start with 'new', for example, from_string is from a Base64
//! compliant &str. Has default Configurations for Base64 numbers see more
//! [here](https://en.wikipedia.org/wiki/Base64) for `Standard`, `URL_SAFE` with and without padding,
//! `IMAP` and `MIME` compliant configs along with full fledged support of creating your own Base64 type.
//! Furthermore, supports random generation of a n long Base64 number.
//! ```
//! extern crate base64;
//!
//! use base64::Base64;
//! use base64::config::{Config, MIME};
//!
//! fn main() {
//!    let s: &str = "Hello!";
//!    let b64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
//!    println!("{}", b64);
//!    let mut v: u128 = 0;
//!    match b64.decode_to_unsigned() {
//!         Ok(value) => v = value,
//!         Err(e) => println!("{}", e),
//!    }
//!    let b64_other = Base64::new_encode_unsigned(&v, MIME);
//!    if b64_other == b64 {
//!         println!("They're equal!");
//!    }
//!    match String::from_utf8(b64.decode_to_bytes()) {
//!         Ok(value) => println!("{}", value), // prints Hello
//!         Err(e) => println!("{}", e),
//!    }
//! }
//! ```
//! See Examples for more.

// Allow use of pow to prevent panics from overflowing
#![feature(no_panic_pow)]
// Prevent trivial casts, require implement debug, completely safe code, and require documentation
#![deny(
    trivial_numeric_casts,
    trivial_casts,
    missing_debug_implementations,
    unsafe_code,
    missing_docs
)]
// Requiring a is_empty function doesn't make sense in this context
#![allow(clippy::len_without_is_empty)]
extern crate rand;

use rand::prelude::*;

use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

/// Creation of custom configs for Base64 numbers containing different characters, with or without
/// padding, with or without a maximum line length. In addition, 5 configs are already defined
/// because of their popularity (`STANDARD`, `MIME`, `IMAP`, `URLSAFE` with and without padding).
pub mod config;
/// Decoding functions for Base64
mod decode;
/// Enconding functions for Base64
mod encode;
/// Enums for Errors that can occur when making a Config or when decoding
pub mod error;

/// Base64 number
///
/// value: a Vec<char> representing the value of the Base64 number
///
/// conf: the config specific for this Base64 number
///
/// Implements Clone, Debug, Eqs, and Compare
#[derive(Eq, Debug, Clone)]
pub struct Base64<'a> {
    value: Vec<char>,
    conf: &'a config::Config<'a>,
}

impl<'a> Base64<'a> {
    /// Creates a default Base64 number equivalent to 0 ("A") with
    /// [STANDARD](../base64/config/constant.STANDARD.html)
    ///
    /// # Returns:
    /// The new Base64 number with the Standard configuration and value of "A"
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    ///
    /// fn main() {
    ///     let b64 = Base64::default(); // Creates new Base64 of value "A"
    ///     println!("{}", b64);
    /// }
    /// ```
    pub fn default() -> Self {
        Base64 {
            value: vec!['A'],
            conf: config::STANDARD,
        }
    }

    /// Creates a random base64 number of at least the provided length.
    ///
    /// # Parameters:
    /// new length of base64 number and the configuration struct, Note if the configuration
    /// specifies padding then the length may be higher if the length specififed isn't divisible by
    /// 4
    ///
    /// #Returns:
    /// the new random base64 number
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING, URL_SAFE_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let b64 = Base64::new_random(5, URL_SAFE_NO_PADDING); // Sets the length to 5 and randomly generates the values
    ///     println!("{}", b64); // Since there's no padding then the length will be 5
    ///     let b64 = Base64::new_random(5, URL_SAFE_PADDING); // Generates a random value of length 5, but with padding
    ///     println!("{}", b64); // Since there's padding then the length will be divisible by 4 therefore length 8
    /// }
    /// ```
    pub fn new_random(len: usize, conf: &'a config::Config<'a>) -> Self {
        let mut val: Vec<char> = Vec::new();
        for _i in 0..len {
            val.push(generate_base64(conf.get_character_set()));
        }
        let mut b64 = Base64 { value: val, conf };
        b64.add_padding(); // Add padding if necessary
        b64
    }

    /// Sets the value of the Base64 number to a random value.  Param: len, length for base64 number
    ///
    /// # Parameters:
    /// The minimum length for the base64 number
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING, URL_SAFE_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let mut b64 = Base64::new_random(5, URL_SAFE_NO_PADDING); // Sets the length to 5 and randomly generates the values
    ///     println!("{}", b64); // Since there's no padding then the length will be 5
    ///     b64.set_random(8);
    ///     println!("{}", b64); // No padding length will now be 8
    /// }
    /// ```
    pub fn set_random(&mut self, len: usize) {
        let mut val: Vec<char> = Vec::new();
        for _i in 0..len {
            val.push(generate_base64(self.conf.get_character_set()));
        }
        self.value = val;
        self.add_padding();
    }

    /// Get length of Base64 number
    ///
    /// # Return:
    /// Return usize of Base64 number
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING}; // Constant config
    ///
    /// fn main() {
    ///     let mut b64 = Base64::new_random(5, URL_SAFE_NO_PADDING); // Sets the length to 5 and randomly generates the values
    ///     println!("{}", b64); // Since there's no padding then the length will be 5
    ///     println!("{}", b64.len()); // Length of 5
    /// }
    /// ```
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Adds the padding character if the Base64 config has padding turned on until the number is
    /// divisible by 4
    fn add_padding(&mut self) {
        if self.conf.get_padding().is_some() {
            while self.len() % 4 != 0 {
                self.value.push(self.conf.get_padding().unwrap());
            }
        }
    }

    /// Sets Base64 to that String if it's valid
    ///
    /// # Return:
    /// If all characters are valid Base64 return Self otherwise a
    /// [Base64Error::InvalidBase64CharacterError](error/enum.Base64Error.html#variant.InvalidBase64CharacterError)
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING, URL_SAFE_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let b64 = Base64::new_from_string("Hello", URL_SAFE_PADDING); // Sets b64 to the string if the string is valid Base64
    ///     // It returns a Result<Base64, String>
    ///     match b64 {
    ///         Ok(value) => println!("{}", 64), // prints "Hello===" adds padding so it's divisible by 4
    ///         Err(e) => println!("{}", e), // The error message stating the first incorrect character
    ///     }
    /// }
    /// ```
    pub fn new_from_string(
        new: &str,
        conf: &'a config::Config<'a>,
    ) -> Result<Self, error::Base64Error> {
        let mut val: Vec<char> = Vec::new();
        for ch in new.chars() {
            if !is_valid_base64('\0', conf.get_character_set(), ch)
                || (conf.get_padding().is_some()
                    && !is_valid_base64(conf.get_padding().unwrap(), conf.get_character_set(), ch))
            {
                return Err(error::Base64Error::InvalidBase64CharacterError);
            } else {
                val.push(ch);
            }
        }
        let mut b64 = Base64 { value: val, conf };
        b64.add_padding();
        Ok(b64)
    }

    /// Takes a new configuration and converts the Base64 number to that representation
    ///
    /// # Example
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING, URL_SAFE_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let mut b64 = Base64::new_encode_unsigned(&63, URL_SAFE_NO_PADDING); // Sets b64 to _
    ///     b64.set_config(URL_SAFE_PADDING); // Changes configs and adds padding to _
    ///     println!("{}", b64); // Prints _===
    /// }
    /// ```
    pub fn set_config(&mut self, conf: &'a config::Config<'a>) {
        // If they aren't the same actually convert
        self.value = self.convert_to_new_config(conf);
        self.conf = conf;
        self.add_padding();
    }

    fn convert_to_new_config(&self, conf: &'a config::Config<'a>) -> Vec<char> {
        let mut v: Vec<char> = Vec::new();
        for i in &self.value {
            if self.conf.get_padding().is_some() && *i == self.conf.get_padding().unwrap() {
                if conf.get_padding().is_some()
                    && conf.get_padding().unwrap() == self.conf.get_padding().unwrap()
                {
                    // Both have padding and their padding is the same
                    continue;
                } else if conf.get_padding().is_some()
                    && conf.get_padding().unwrap() != self.conf.get_padding().unwrap()
                {
                    // They both have padding and they're different
                    v.push(conf.get_padding().unwrap());
                } else {
                    // The new configuration doesn't have padding and therefore skip it
                    continue;
                }
            } else {
                // Convert the current configuration value to it's new equivalent
                v.push(decimal_to_base64_char(
                    conf.get_character_set(),
                    base64_char_to_decimal(self.conf.get_character_set(), *i),
                ));
            }
        }
        v
    }

    /// Sets the Base64 value to a given String
    ///
    /// # Return:
    /// false if any value is invalid
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING, URL_SAFE_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let mut b64 = Base64::new_encode_unsigned(&63, URL_SAFE_NO_PADDING); // Sets b64 to _
    ///     b64.set_config(URL_SAFE_PADDING); // Changes configs and adds padding to _
    ///     println!("{}", b64); // Prints _===
    /// }
    /// ```
    pub fn set_from_string(&mut self, new: &str) -> bool {
        let mut val: Vec<char> = Vec::new();
        for ch in new.chars() {
            if (self.conf.get_padding().is_none()
                && is_valid_base64('\0', self.conf.get_character_set(), ch))
                || (self.conf.get_padding().is_some()
                    && is_valid_base64(
                        self.conf.get_padding().unwrap(),
                        self.conf.get_character_set(),
                        ch,
                    ))
            {
                val.push(ch);
            } else {
                return false;
            }
        }
        self.value = val;
        self.add_padding();
        true
    }

    /// Extends base 64 number by prepending As to it to fit a new size
    ///
    /// # Parameters:
    /// len, the new size of the base64 value
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING, URL_SAFE_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let mut b64 = Base64::new_encode_unsigned(&63, URL_SAFE_NO_PADDING); // Sets b64 to _
    ///     b64.set_config(URL_SAFE_PADDING); // Changes configs and adds padding to _
    ///     println!("{}", b64); // Prints _===
    /// }
    /// ```
    pub fn expand_to(&mut self, len: usize) {
        while self.value.len() < len {
            self.value.push('A');
        }
        self.value.reverse();
        self.add_padding();
    }

    /// Truncates base64 number be removing the most significant values until it fits the new size
    ///
    /// # Parameters:
    /// len, the new size of the base64 value. Must be greater than 0
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_PADDING, URL_SAFE_NO_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let mut b64 = Base64::new_encode_unsigned(&63, URL_SAFE_PADDING); // Sets b64 to _===
    ///     println!("{}", b64); // Prints _===
    ///     b64.truncate_to(2); // This does essentially nothing because padding is required and therefore it must be divisible by 4
    ///     println!("{}", b64); // Prints _=== stil
    ///     let mut b64 = Base64::new_encode_unsigned(&63, URL_SAFE_NO_PADDING); // Sets b64 to _
    ///     b64.truncate_to(1); // Length is already 1 so it remains _
    /// }
    /// ```
    pub fn truncate_to(&mut self, len: usize) {
        if len > 0 && self.value.len() > len {
            self.value.reverse(); // flip to remove most significant values
            while self.value.len() > len {
                self.value.pop();
            }
            self.value.reverse(); // flip back
            self.add_padding(); // Add padding
        }
    }
}

/// Generates values from 0 to 63 and returns the character corresponding to it
fn generate_base64(a: &[char]) -> char {
    decimal_to_base64_char(a, thread_rng().gen_range(0, 64) as u128)
}

/// Checks if a character is a valid value in Base64
/// Param: val, the character to check as a u8
/// Return: true if it's value false otherwise
fn is_valid_base64(pad: char, a: &[char], val: char) -> bool {
    if val == '\n' || val == ' ' || val == pad {
        return true;
    } else {
        for i in a.iter() {
            if val == *i {
                return true;
            }
        }
    }
    false
}

/// Convert decimal value to base64 by mod 64 to get the base64 place and then dividing
/// by 64 to get the value
/// Param: value, the value to convert
/// Return Vector of chars that is the Base64 value
pub(crate) fn decimal_to_base64(conf: &config::Config, mut value: u128) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    while value > 0 {
        let base64_val = value % 64;
        value /= 64;
        v.push(decimal_to_base64_char(conf.get_character_set(), base64_val));
    }
    v.reverse(); // Reverse to get into proper order
    v
}

/// Converts a decimal value to it's base 64 value
/// Param: value, the value to convert
/// Return: the character corresponding to the decimal in Base64
pub(crate) fn decimal_to_base64_char(a: &[char], value: u128) -> char {
    a[value as usize]
}

/// Converts a char to it's corresponding u128 value in base64
/// Param: value, char to convert
/// Return: u128, the value of the char in base64
pub(crate) fn base64_char_to_decimal(a: &[char], c: char) -> u128 {
    for (i, val) in a.iter().enumerate() {
        if c == *val {
            return i as u128;
        }
    }
    0 // Not Possible
}

impl<'a> Display for Base64<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut print: String = String::new();
        for i in &self.value {
            print.push(*i);
        }
        write!(f, "{}", print)
    }
}

impl<'a> PartialEq for Base64<'a> {
    fn eq(&self, other: &Base64) -> bool {
        if self.value.len() != other.value.len() {
            return false;
        } else {
            for i in 0..self.value.len() {
                if self.value[i] != other.value[i] {
                    return false;
                }
            }
        }
        true
    }
}

impl<'a> Ord for Base64<'a> {
    fn cmp(&self, other: &Base64<'a>) -> Ordering {
        if self.value.len() != other.value.len() {
            // Different lengths
            return self.value.len().cmp(&other.value.len());
        } else {
            for i in 0..self.value.len() {
                if self.value[i] != '\n'
                    && other.value[i] != '\n'
                    && self.value[i] != other.value[i]
                {
                    // Convert each to their decimal value then cmp
                    return base64_char_to_decimal(self.conf.get_character_set(), self.value[i])
                        .cmp(&base64_char_to_decimal(
                            other.conf.get_character_set(),
                            other.value[i],
                        ));
                }
            }
        }
        Ordering::Equal
    }
}

impl<'a> PartialOrd for Base64<'a> {
    fn partial_cmp(&self, other: &Base64<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
