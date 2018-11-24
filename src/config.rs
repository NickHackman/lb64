use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

use super::error::ConfigError;

/// Configuration for Base64 number that consists of
///
/// character_set: the characters the Base64 number can have. First character provided is given
/// value 0 and so on until the 64th character which is value 63
///
/// pad: Optional padding character for the Base64 number
///
/// line_length: Optional maximum line length for the Base64 number
///
/// All characters must be graphically representable characters in [UTF8](https://www.utf8-chartable.de/unicode-utf8-table.pl)
///
/// Implements Equals, Debug, and Clone
#[derive(Eq, Debug, Clone)]
pub struct Config<'a> {
    character_set: &'a [char],
    pad: Option<char>,
    line_length: Option<u8>,
}

impl<'a> Config<'a> {
    /// Creates a config with provided values
    ///
    /// # Parameters:
    /// Character set of the base64 values
    ///
    /// Optional: padding for base64
    ///
    /// Optional: Fixed line length
    ///
    /// # Returns:
    /// Result<Self, base64::error::ConfigError> either the new config or an error of:
    ///     [CharacterSetLengthError](../error/enum.ConfigError.html#variant.CharacterSetLengthError),
    ///     [NotUniquePaddingError](../error/enum.ConfigError.html#variant.NotUniquePaddingError),
    ///     [DuplicateCharacterError](../error/enum.ConfigError.html#variant.DuplicateCharacterError),
    ///     [CharacterSetUnrepresentableCharacter](../error/enum.ConfigError.html#variant.CharacterSetUnrepresentableCharacter),
    ///     or
    ///     [PaddingUnrepresentableCharacter](../error/enum.ConfigError.html#variant.PaddingUnrepresentableCharacter)
    ///
    /// # Example:
    /// ```
    /// extern crate lb64;
    ///
    /// use lb64::config::Config;
    ///
    /// fn main() {
    ///     let character_set = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    /// ];
    ///     match Config::new(character_set, None, None) {
    ///         Ok(conf) => println!("Successful"),
    ///         Err(e) => println!("{}", e),
    ///     }
    /// }
    /// ```
    pub fn new(
        set: &'a [char],
        pad_char: Option<char>,
        len: Option<u8>,
    ) -> Result<Self, ConfigError> {
        if set.len() != 64 {
            Err(ConfigError::CharacterSetLengthError)
        } else if pad_char.is_some() && !check_unique_pad(set, pad_char.unwrap()) {
            Err(ConfigError::NotUniquePaddingError)
        } else if !character_set_is_representable(set) {
            Err(ConfigError::CharacterSetUnrepresentableCharacter)
        } else if pad_char.is_some() && !is_representable(pad_char.unwrap()) {
            Err(ConfigError::PaddingUnrepresentableCharacter)
        } else if are_duplicates(set) {
            Err(ConfigError::DuplicateCharacterError)
        } else {
            Ok(Self {
                character_set: set,
                pad: pad_char,
                line_length: len,
            })
        }
    }

    /// Sets the character set by the provided slice
    ///
    /// # Returns:
    /// A Result<(), base64::error::ConfigError> possible ConfigErrors are
    /// [CharacterSetLengthError](../error/enum.ConfigError.html#variant.CharacterSetLengthError),
    /// [DuplicateCharacterError](../error/enum.ConfigError.html#variant.DuplicateCharacterError),
    /// or [CharacterSetUnrepresentableCharacter](../error/enum.ConfigError.html#variant.CharacterSetUnrepresentableCharacter)
    ///
    /// # Example:
    /// ```
    /// extern crate lb64;
    ///
    /// use lb64::config::Config;
    ///
    /// fn main() {
    ///     let character_set_orig = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    /// ];
    ///     let character_set_new = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
    /// ];
    ///     match Config::new(character_set_orig, None, None) {
    ///         Ok(mut conf) => {
    ///            match conf.set_character_set(character_set_new) {
    ///                 Ok(()) => println!("Successful!"), // Since there are no duplicate characters and character_set_new is of length 64 it's successful
    ///                 Err(e) => println!("{}", e),
    ///            }
    ///         },
    ///         Err(e) => println!("{}", e),
    ///     }
    /// }
    /// ```
    pub fn set_character_set(&mut self, set: &'a [char]) -> Result<(), ConfigError> {
        if set.len() != 64 {
            Err(ConfigError::CharacterSetLengthError)
        } else if are_duplicates(set) {
            Err(ConfigError::DuplicateCharacterError)
        } else if !character_set_is_representable(set) {
            Err(ConfigError::CharacterSetUnrepresentableCharacter)
        } else {
            self.character_set = set;
            Ok(())
        }
    }

    /// Returns the slice of all the characters in the character set
    ///
    /// # Example:
    /// ```
    /// extern crate lb64;
    ///
    /// use lb64::config::MIME;
    ///
    /// fn main() {
    ///     println!("{:?}", MIME.get_character_set()); // Prints a slice containing [A-Z], [a-z], [0-9], +, and /
    /// }
    /// ```
    pub fn get_character_set(&self) -> &[char] {
        self.character_set
    }

    /// Return Line_length field
    ///
    /// # Example:
    /// ```
    /// extern crate lb64;
    ///
    /// use lb64::config::MIME;
    ///
    /// fn main() {
    ///     match MIME.get_line_length() {
    ///         Some(len) => println!("{}", len), // Prints 76
    ///         None => println!("Line length for Mime isn't None"),
    ///     }
    /// }
    /// ```
    pub fn get_line_length(&self) -> Option<u8> {
        self.line_length
    }

    /// Sets the maximum line length for a configuration
    ///
    /// # Example:
    /// ```
    /// extern crate lb64;
    ///
    /// use lb64::config::Config;
    /// use lb64::error::ConfigError;
    ///
    /// fn main() {
    ///     let character_set = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    /// ];
    ///     match Config::new(character_set, None, None) {
    ///         Ok(mut conf) => {
    ///            conf.set_line_length(Some(5)); // Sets the line_length from None to 5
    ///         },
    ///         Err(e) => println!("{}", e),
    ///     }
    /// }
    /// ```
    pub fn set_line_length(&mut self, len: Option<u8>) {
        self.line_length = len;
    }

    /// Return Padding character
    ///
    /// # Example:
    /// ```
    /// extern crate lb64;
    ///
    /// use lb64::config::MIME;
    ///
    /// fn main() {
    ///     match MIME.get_padding() {
    ///         Some(pad) => println!("{}", pad), // Prints =
    ///         None => println!("Padding for Mime isn't None"),
    ///     }
    /// }
    /// ```
    pub fn get_padding(&self) -> Option<char> {
        self.pad
    }

    /// Sets the padding character
    ///
    /// # Returns:
    /// Returns a result of <(), base64::error::ConfigError> the ConfigError is either
    /// [NotUniquePaddingError](../error/enum.ConfigError.html#variant.NotUniquePaddingError) or
    /// [PaddingUnrepresentableCharacter](../error/enum.ConfigError.html#variant.PaddingUnrepresentableCharacter)
    ///
    /// # Example:
    /// ```
    /// extern crate lb64;
    ///
    /// use lb64::config::Config;
    ///
    /// fn main() {
    ///     let character_set = &[
    ///     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    ///     'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    ///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    ///     'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    /// ];
    ///     match Config::new(character_set, None, None) {
    ///         Ok(mut conf) => {
    ///             match conf.set_padding(Some('/')) {
    ///                 Ok(()) => println!("Set padding character successful!"),
    ///                 Err(e) => println!("{}", e), // This occurs because / is already taken
    ///             }
    ///         },
    ///         Err(e) => println!("{}", e),
    ///     }
    /// }
    /// ```
    pub fn set_padding(&mut self, pad_char: Option<char>) -> Result<(), ConfigError> {
        if pad_char.is_some() && !check_unique_pad(self.character_set, pad_char.unwrap()) {
            Err(ConfigError::NotUniquePaddingError)
        } else if pad_char.is_some() && is_representable(pad_char.unwrap()) {
            Err(ConfigError::PaddingUnrepresentableCharacter)
        } else {
            self.pad = pad_char;
            Ok(())
        }
    }
}

/// `MIME` compliant configuration as specified in [RFC 2045](https://tools.ietf.org/html/rfc2045)
///
/// # Specifics:
///
/// Character Set: [A-Z], [a-z], [0-9], +, /
///
/// Padding Character: =
///
/// Maximum Line Length: 76
///
/// # Example:
/// ```
/// extern crate lb64;
/// use lb64::{config, Base64};
///
/// fn main() {
///     let b64 = Base64::new_encode_unsigned(&63, config::MIME); // Creates a MIME Compliant b64 of value 0
///     println!("{}", config::MIME);
///     println!("{}", b64);
///     // Prints:
///     // "/"
/// }
/// ```
pub const MIME: &Config = {
    &Config {
        character_set: &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
        ],
        pad: Some('='),
        line_length: Some(76),
    }
};

/// `IMAP` compliant configuration as specified in [RFC 3501](https://tools.ietf.org/html/rfc3501)
///
/// # Specifics:
///
/// Character Set: [A-Z], [a-z], [0-9], +, ,
///
/// Padding Character: None
///
/// Maximum Line Length: None
///
/// # Example:
/// ```
/// extern crate lb64;
/// use lb64::{config, Base64};
///
/// fn main() {
///     let b64 = Base64::new_encode_unsigned(&63, config::IMAP); // Creates a IMAP Compliant b64 of value 63
///     println!("{}", config::IMAP);
///     println!("{}", b64);
///     // Prints:
///     // ","
/// }
/// ```
pub const IMAP: &Config = {
    &Config {
        character_set: &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', ',',
        ],
        pad: None,
        line_length: None,
    }
};

/// `Base64url` with padding compliant configuration as specified in [RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)
///
/// # Specifics:
///
/// Character Set: [A-Z], [a-z], [0-9], -, _
///
/// Padding Character: =
///
/// Maximum Line Length: No maximum
///
/// # Example:
/// ```
/// extern crate lb64;
/// use lb64::{config, Base64};
///
/// fn main() {
///     let b64 = Base64::new_encode_unsigned(&63, config::URL_SAFE_PADDING); // Creates a base64url Compliant b64 of value 63
///     println!("{}", config::URL_SAFE_PADDING);
///     // Prints:
///     // -, _, =
///     println!("{}", b64);
///     // Prints:
///     // "_"
/// }
/// ```
pub const URL_SAFE_PADDING: &Config = {
    &Config {
        character_set: &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
        ],
        pad: Some('='),
        line_length: None,
    }
};

/// `Base64url` without padding compliant configuration as specified in [RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)
///
/// # Specifics:
///
/// Character Set: [A-Z], [a-z], [0-9], -, _
///
/// Padding Character: None
///
/// Maximum Line Length: No maximum
///
/// # Example:
/// ```
/// extern crate lb64;
/// use lb64::{config, Base64};
///
/// fn main() {
///     let b64 = Base64::new_encode_unsigned(&63, config::URL_SAFE_NO_PADDING); // Creates a base64url Compliant b64 without padding of value 0
///     println!("{}", config::URL_SAFE_NO_PADDING);
///     // Prints:
///     // -, _
///     println!("{}", b64);
///     // Prints:
///     // "_"
/// }
/// ```
pub const URL_SAFE_NO_PADDING: &Config = {
    &Config {
        character_set: &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
        ],
        pad: None,
        line_length: None,
    }
};

/// Base64 `standard` compliant configuration as specified in [RFC 4648](https://tools.ietf.org/html/rfc4648)
///
/// # Specifics:
///
/// Character Set: [A-Z], [a-z], [0-9], +, /
///
/// Padding Character: =
///
/// Maximum Line Length: No maximum
///
/// # Example:
/// ```
/// extern crate lb64;
/// use lb64::{config, Base64};
///
/// fn main() {
///     let b64 = Base64::new_encode_unsigned(&63, config::STANDARD); // Creates a Standard Compliant b64 of value 63
///     println!("{}", config::STANDARD);
///     println!("{}", b64);
/// }
/// ```
pub const STANDARD: &Config = {
    &Config {
        character_set: &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
        ],
        pad: Some('='),
        line_length: None,
    }
};

impl<'a> PartialEq for Config<'a> {
    fn eq(&self, other: &Config) -> bool {
        self.character_set == other.character_set
            && self.pad == other.pad
            && self.line_length == other.line_length
    }
}

impl<'a> Display for Config<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut print: String = String::new();
        for c in self.character_set.iter() {
            print.push(*c);
        }
        match self.pad {
            Some(p) => print.push(p),
            None => print.push_str(&""),
        }
        match self.line_length {
            Some(l) => print.push_str(&l.to_string()),
            None => print.push_str(&""),
        }
        write!(f, "{}", print)
    }
}

/// Checks to see if the provided character is unique in the provided slice
fn check_unique_pad(set: &[char], v: char) -> bool {
    for c in set {
        if *c == v {
            return false;
        }
    }
    true
}

/// Checks for duplicates in slice
fn are_duplicates(set: &[char]) -> bool {
    for i in 0..set.len() {
        for j in (i + 1)..set.len() {
            if set[i] == set[j] {
                return true;
            }
        }
    }
    false
}

/// Checks to see if the character set (slice) are all representable
fn character_set_is_representable(set: &[char]) -> bool {
    for c in set {
        if !is_representable(*c) {
            return false;
        }
    }
    true
}

/// Checks to see if the character is representable
fn is_representable(c: char) -> bool {
    let u: u16 = c as u16;
    u > 31 && u != 127 && u != b' '.into() && !c.is_control()
}
