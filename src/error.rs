use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::mem;

/// Possible Configuration errors when either setting or creating a new configuration that may occur
#[derive(Debug)]
pub enum ConfigError {
    ///character set provided isn't of length 64
    /// # Example:
    /// ```
    /// let character_set = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+',
    /// ]; // Throws Error because Length is 63 and not 64
    /// ```
    CharacterSetLengthError,
    /// padding character provided is already used in character set
    /// # Example:
    /// ```
    /// let character_set = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    /// ];
    /// let pad = &Some('/'); // Throws Error because '/' is already taken in the character set
    /// ```
    NotUniquePaddingError,
    /// character set provided has duplicate characters
    /// # Example:
    /// ```
    /// let character_set = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '+',
    /// ]; // Throws Error because '+' is defined twice in the character set
    /// ```
    DuplicateCharacterError,
    /// Character in character set isn't representable
    /// # Example:
    /// ```
    /// let character_set = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '\0',
    /// ]; // Throws Error because '\0' isn't representable
    /// ```
    CharacterSetUnrepresentableCharacter,
    /// Padding character isn't representable
    /// # Example:
    /// ```
    /// let pad = &Some('\n'); // Throws Error because '\n' isn't representable
    /// ```
    PaddingUnrepresentableCharacter,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ConfigError::CharacterSetLengthError => {
                f.write_str("Provided Character set length isn't 64")
            }
            ConfigError::NotUniquePaddingError => {
                f.write_str("Padding character provided is already used in character set")
            }
            ConfigError::DuplicateCharacterError => {
                f.write_str("At least one duplicate character found in character set")
            }
            ConfigError::CharacterSetUnrepresentableCharacter => {
                f.write_str("Character set has at leaset one unrepresentable character")
            }
            ConfigError::PaddingUnrepresentableCharacter => {
                f.write_str("Padding is a character that is unrepresentable")
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::CharacterSetLengthError => "Provided Character set length isn't 64",
            ConfigError::NotUniquePaddingError => {
                "Padding character provided is already used in character set"
            }
            ConfigError::DuplicateCharacterError => {
                "At least one duplicate character found in character set"
            }
            ConfigError::CharacterSetUnrepresentableCharacter => {
                "Character set has at leaset one unrepresentable character"
            }
            ConfigError::PaddingUnrepresentableCharacter => {
                "Padding is a character that is unrepresentable"
            }
        }
    }
}

impl PartialEq for ConfigError {
    fn eq(&self, other: &ConfigError) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

/// Possible errors when decoding Base64 number
#[derive(Debug)]
pub enum Base64Error {
    /// Unsigned Overflow when decoding Base64 number to unsigned
    ///
    /// Only applies to
    /// [Base64::decode_to_unsigned](../struct.Base64.html#method.decode_to_unsigned)
    /// # Example:
    /// ```
    /// use base64::{Base64, config::MIME};
    ///
    /// let b64 = Base64::new_random(999, MIME);
    /// match b64.decode_to_unsigned() {
    ///     Ok(value) => println!("This is impossible"),
    ///     Err(e) => println!("{}", e), // Base64Error::OverflowError occurred
    /// }
    /// ```
    OverflowError,
    /// Invalid character in Base64 provided &str
    ///
    /// Only applies to [Base64::new_from_string](../struct.Base64.html#method.new_from_string)
    /// # Example:
    /// ```
    /// use base64::{Base64, config::MIME};
    ///
    /// match Base64::new_from_string(&"^_^", MIME) {
    ///     Ok(value) => println!("This is impossible"),
    ///     Err(e) => println!("{}", e), // Base64Error::InvalidBase64CharacterError occurred
    /// }
    /// ```
    InvalidBase64CharacterError,
}

impl Display for Base64Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            Base64Error::OverflowError => {
                f.write_str("Unsigned Overflow when decoding Base64 to unsigned")
            }
            Base64Error::InvalidBase64CharacterError => {
                f.write_str("Invalid character in provided Base64 &str")
            }
        }
    }
}

impl std::error::Error for Base64Error {
    fn description(&self) -> &str {
        match *self {
            Base64Error::OverflowError => {
                "Unsigned Overflow occured when decoding Base64 to unsigned"
            }
            Base64Error::InvalidBase64CharacterError => "Invalid character in provided Base64 &str",
        }
    }
}

impl PartialEq for Base64Error {
    fn eq(&self, other: &Base64Error) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}
