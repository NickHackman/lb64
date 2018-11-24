use super::{config::Config, decimal_to_base64, decimal_to_base64_char, Base64};

impl<'a> Base64<'a> {
    /// Creates a base64 number equivalent to the provided unsigned value
    ///
    /// # Parameters:
    /// unsigned, the unsigned value to convert
    ///
    /// # Return:
    /// the new base64 number equivalent to the unsigned value passed
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING, URL_SAFE_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let b64 = Base64::new_encode_unsigned(&128, URL_SAFE_NO_PADDING); // Sets b64 to the equivalent Base64 of 128
    ///     println!("{}", b64); // prints "CA"
    ///     let b64 = Base64::new_encode_unsigned(&128, URL_SAFE_PADDING); // Sets b64 to the equivalent Base64 of 128 with padding
    ///     println!("{}", b64); // prints "CA==" so it's divisble by 4
    /// }
    /// ```
    pub fn new_encode_unsigned(unsigned: &u128, conf: &'a Config<'a>) -> Self {
        let mut b64 = {
            if *unsigned > 0 {
                Base64 {
                    value: decimal_to_base64(&conf, *unsigned),
                    conf,
                }
            } else {
                Base64 {
                    value: vec!['A'],
                    conf,
                }
            }
        };
        b64.add_padding(); // pad if necessary
        b64
    }

    /// Sets the base64 value from an unsigned integer u128
    ///
    /// # Parameters:
    /// the unsigned value to set the b64 equivalent to
    ///
    /// # Example:
    /// ```
    /// extern crate base64; // Import/Include crate
    /// use base64::{Base64}; // Base64
    /// use base64::config::{URL_SAFE_NO_PADDING, URL_SAFE_PADDING}; // Constant configs
    ///
    /// fn main() {
    ///     let mut b64 = Base64::default(); // Sets b64 to default which is Standard config and "A"
    ///     println!("{}", b64); // prints "A"
    ///     b64.encode_unsigned(&2);
    ///     println!("{}", b64); // prints "C===" because Standard has padding
    /// }
    /// ```
    pub fn encode_unsigned(&mut self, unsigned: &u128) {
        if *unsigned > 0 {
            self.value = decimal_to_base64(&self.conf, *unsigned);
        } else {
            self.value = vec!['A'];
        }
        self.add_padding();
    }

    /// Encodes the provided bytes slice into Base64
    ///
    /// # Parameters:
    /// The configuration struct
    ///
    /// &[u8] the bytes to convert
    ///
    /// # Returns:
    /// The new Base64 number
    ///
    /// # Example:
    /// ```
    /// extern crate base64;
    /// use base64::{Base64};
    /// use base64::config::MIME; // Include MIME config
    ///
    /// fn main() {
    ///     let word: &str = "Hi";
    ///     let b64 = Base64::new_encode_bytes(word.as_bytes(), MIME);
    ///     println!("{}", b64);
    /// }
    /// ```
    pub fn new_encode_bytes(s: &[u8], conf: &'a Config) -> Self {
        Base64 {
            value: encode_bytes(conf, s).chars().collect(),
            conf,
        }
    }

    /// Sets the Base64 value to the encoded byte value in base64
    ///
    /// # Parameters:
    /// The bytes to encode
    ///
    /// # Example:
    /// ```
    /// extern crate base64;
    /// use base64::{Base64};
    /// use base64::config::MIME; // Include MIME config
    ///
    /// fn main() {
    ///     let word: &str = "Hi";
    ///     let mut b64 = Base64::new_encode_bytes(word.as_bytes(), MIME);
    ///     println!("{}", b64);
    ///     let new_word: &str = "Hi!";
    ///     b64.encode_bytes(new_word.as_bytes()); // Changes the encoded
    ///     println!("{}", b64);
    /// }
    /// ```
    pub fn encode_bytes(&mut self, s: &[u8]) {
        self.value = encode_bytes(self.conf, s).chars().collect();
    }
}

/// Check to see if every byte in a 6 long &str is '?'
fn is_padding(s: &str) -> bool {
    for i in s.as_bytes() {
        if *i != b'?' {
            return false;
        }
    }
    true
}

/// Converts a string of chars to a binary String
fn convert_bytes_to_binary_string(s: &[u8]) -> String {
    let mut binary: String = String::new();
    for c in s.iter() {
        binary.push_str(&convert_u8_to_binary_string(*c));
    }
    while binary.len() % 6 != 0 {
        // Make sure it's divisible by 6 for each base64 character
        binary.push('0');
    }
    while binary.len() % 24 != 0 {
        // Add padding if it isn't divisible by 24
        binary.push('?');
    }
    binary
}

/// Convert a u8 to a String of binary corresponding to it's value
fn convert_u8_to_binary_string(value: u8) -> String {
    const U8_LENGTH: usize = 8;
    let mut binary: String = String::new();
    for i in (0..U8_LENGTH).rev() {
        // Get each bit in the 8 bit binary and convert it to a char
        binary.push((b'0' + ((value >> i) & 1)) as char);
    }
    binary
}

/// Converts a 6 bit binary value to a u128
fn convert_6bit_to_u128(s: &str) -> u128 {
    let mut value: u128 = 0;
    for (i, c) in s.chars().enumerate() {
        // if it's 1 add 2^place
        if c == '1' {
            value += 2u128.pow(((s.len() - 1) - i) as u32);
        }
    }
    value
}

fn encode_bytes<'a>(conf: &'a Config, s: &[u8]) -> String {
    let binary: String = convert_bytes_to_binary_string(s); // Convert all characters to binary
    let mut b64_str: String = String::new();
    let mut count = 0;
    for i in (0..binary.len()).step_by(6) {
        // Loop over binary getting every 6 bits and converting them to a Base64 value
        if conf.get_padding().is_some() && is_padding(&binary[i..i + 6]) {
            // If the config enables padding then
            // Check to see if the values are padding
            match conf.get_padding() {
                Some(c) => b64_str.push(c),
                None => {
                    continue;
                }
            }
            continue;
        }
        // Convert every 6 bits to 1 Base64 value
        let value: u128 = convert_6bit_to_u128(&binary[i..i + 6]);
        let b64_char: char = decimal_to_base64_char(conf.get_character_set(), value);
        if conf.get_line_length().unwrap_or(0) != 0 && count < conf.get_line_length().unwrap() {
            // if the line_length is fixed keep a count
            count += 1;
        } else if conf.get_line_length().unwrap_or(0) != 0
            && count == conf.get_line_length().unwrap()
        {
            // at line_length value add newline
            count = 0;
            b64_str.push('\n');
        }
        b64_str.push(b64_char);
    }
    b64_str
}
