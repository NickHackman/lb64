extern crate lb64;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)] // Allow imports of everything
    use lb64::config::{Config, IMAP, MIME, STANDARD, URL_SAFE_NO_PADDING, URL_SAFE_PADDING};
    #[allow(unused_imports)] // Allow imports of everything
    use lb64::error::{Base64Error, ConfigError};
    #[allow(unused_imports)] // Allow imports of everything
    use lb64::Base64;

    #[test]
    fn create_from_10() {
        let x: Base64 = Base64::new_encode_unsigned(&10, URL_SAFE_NO_PADDING);
        assert_eq!(x.to_string(), "K");
    }

    #[test]
    fn create_from_0() {
        let x: Base64 = Base64::new_encode_unsigned(&0, URL_SAFE_NO_PADDING);
        assert_eq!(x.to_string(), "A");
    }

    #[test]
    fn create_from_128() {
        let x: Base64 = Base64::new_encode_unsigned(&128, URL_SAFE_NO_PADDING);
        assert_eq!(x.to_string(), "CA");
    }

    #[test]
    fn create_from_65538() {
        let x: Base64 = Base64::new_encode_unsigned(&65538, URL_SAFE_NO_PADDING);
        assert_eq!(x.to_string(), "QAC");
    }

    #[test]
    fn create_from_decimal_60() {
        let x: Base64 = Base64::new_encode_unsigned(&60, URL_SAFE_NO_PADDING);
        assert_eq!(x.to_string(), "8");
    }

    #[test]
    fn decode_to_unsigned_99() {
        let x: Base64 = Base64::new_encode_unsigned(&99, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 99),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn decode_to_unsigned_32() {
        let x: Base64 = Base64::new_encode_unsigned(&32, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 32),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn decode_to_unsigned_27() {
        let x: Base64 = Base64::new_encode_unsigned(&27, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 27),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn decode_to_unsigned_90() {
        let x: Base64 = Base64::new_encode_unsigned(&90, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 90),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn decode_to_unsigned_100() {
        let x: Base64 = Base64::new_encode_unsigned(&100, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 100),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn decode_to_unsigned_zzz() {
        let x = Base64::new_from_string(&"zzz", URL_SAFE_NO_PADDING);
        match x {
            Ok(val) => match val.decode_to_unsigned() {
                Ok(value) => assert_eq!(value, 212211),
                Err(e) => println!("{}\n", e),
            },
            Err(er) => println!("{}\n", er),
        }
    }

    #[test]
    fn decode_to_unsigned_10000() {
        let x: Base64 = Base64::new_encode_unsigned(&10000, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 10000),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn extend_63_2() {
        let mut x: Base64 = Base64::new_encode_unsigned(&63, URL_SAFE_NO_PADDING);
        x.expand_to(2);
        assert_eq!(x.to_string(), "A_");
    }

    #[test]
    fn to_string_abcd() {
        let x = Base64::new_from_string(&"abcd", URL_SAFE_NO_PADDING);
        match x {
            Ok(val) => {
                assert_eq!(val.to_string(), "abcd");
            }
            Err(er) => println!("{}\n", er),
        }
    }

    #[test]
    fn truncate_abcd_3() {
        let x = Base64::new_from_string(&"abcd", URL_SAFE_NO_PADDING);
        match x {
            Ok(mut val) => {
                val.truncate_to(1);
                assert_eq!(val.to_string(), "d");
            }
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn truncate_abcd_3_pad() {
        let x = Base64::new_from_string(&"abcd", URL_SAFE_PADDING);
        match x {
            Ok(mut val) => {
                val.truncate_to(1);
                assert_eq!(val.to_string(), "d===");
            }
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn decode_to_unsigned_60() {
        let x: Base64 = Base64::new_encode_unsigned(&60, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 60),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn decode_to_unsigned_1() {
        let x: Base64 = Base64::new_encode_unsigned(&1, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 1),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn decode_to_unsigned_20769187000000000000000000000000000() {
        let x: Base64 =
            Base64::new_encode_unsigned(&20769187000000000000000000000000000, URL_SAFE_NO_PADDING);
        match x.decode_to_unsigned() {
            Ok(val) => assert_eq!(val, 20769187000000000000000000000000000),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn create_from_string_1997() {
        let x = Base64::new_from_string(&"1997", URL_SAFE_NO_PADDING);
        match x {
            Ok(val) => assert_eq!(val.to_string(), "1997"),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn not_equals_5_6() {
        let x = Base64::new_encode_unsigned(&5, URL_SAFE_NO_PADDING);
        let y = Base64::new_encode_unsigned(&6, URL_SAFE_NO_PADDING);
        assert_eq!(x != y, true);
    }

    #[test]
    fn equals_6_6() {
        let x = Base64::new_encode_unsigned(&6, URL_SAFE_NO_PADDING);
        let y = Base64::new_encode_unsigned(&6, URL_SAFE_NO_PADDING);
        assert_eq!(x == y, true);
    }

    #[test]
    fn less_than_5_6() {
        let x = Base64::new_encode_unsigned(&5, URL_SAFE_NO_PADDING);
        let y = Base64::new_encode_unsigned(&6, URL_SAFE_NO_PADDING);
        assert_eq!(x < y, true);
    }

    #[test]
    fn greater_than_6_5() {
        let x = Base64::new_encode_unsigned(&5, URL_SAFE_NO_PADDING);
        let y = Base64::new_encode_unsigned(&6, URL_SAFE_NO_PADDING);
        assert_eq!(y > x, true);
    }

    #[test]
    fn len_on_0() {
        let x = Base64::new_encode_unsigned(&0, URL_SAFE_NO_PADDING);
        assert_eq!(x.len(), 1);
    }

    #[test]
    fn len_on_4() {
        let x = Base64::new_from_string(&"1997", URL_SAFE_NO_PADDING);
        match x {
            Ok(val) => assert_eq!(val.len(), 4),
            Err(e) => println!("{}\n", e),
        }
    }

    #[test]
    fn random() {
        let x = Base64::new_random(5, URL_SAFE_NO_PADDING);
        assert_eq!(x.len(), 5);
    }

    #[test]
    fn set_config() {
        let mut b64 = Base64::new_encode_unsigned(&62, STANDARD);
        assert_eq!(b64.to_string(), "+===");
        b64.set_config(URL_SAFE_NO_PADDING);
        assert_eq!(b64.to_string(), "-");
    }

    #[test]
    fn config_equals() {
        assert_eq!(URL_SAFE_NO_PADDING == URL_SAFE_NO_PADDING, true);
    }

    #[test]
    fn config_not_equals() {
        assert_eq!(URL_SAFE_PADDING != URL_SAFE_NO_PADDING, true);
    }

    #[test]
    fn config_new_err_len() {
        let character_set = &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+',
        ];
        match Config::new(character_set, None, None) {
            Ok(val) => {
                println!("{}", val);
            }
            Err(e) => {
                assert_eq!(e, ConfigError::CharacterSetLengthError);
            }
        }
    }

    #[test]
    fn config_new_err_not_unique_padding() {
        let character_set = &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
        ];
        match Config::new(character_set, Some('/'), None) {
            Ok(val) => {
                println!("{}", val);
            }
            Err(e) => {
                assert_eq!(e, ConfigError::NotUniquePaddingError);
            }
        }
    }

    #[test]
    fn config_new_err_duplicate_character_set() {
        let character_set = &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '+',
        ];
        match Config::new(character_set, None, None) {
            Ok(val) => {
                println!("{}", val);
            }
            Err(e) => {
                assert_eq!(e, ConfigError::DuplicateCharacterError);
            }
        }
    }

    #[test]
    fn config_new_err_unrepresentable_in_character_set() {
        let character_set = &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '\0', '+', '/',
        ];
        match Config::new(character_set, None, None) {
            Ok(val) => {
                println!("{}", val);
            }
            Err(e) => {
                assert_eq!(e, ConfigError::CharacterSetUnrepresentableCharacter);
            }
        }
    }

    #[test]
    fn config_new_err_unrepresentable_padding() {
        let character_set = &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
        ];
        match Config::new(character_set, Some('\0'), None) {
            Ok(val) => {
                println!("{}", val);
            }
            Err(e) => {
                assert_eq!(e, ConfigError::PaddingUnrepresentableCharacter);
            }
        }
    }

    #[test]
    fn base64_encode_hello_world() {
        let s: &str = "Hello, World";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!("SGVsbG8sIFdvcmxk", b64.to_string());
    }

    #[test]
    fn base64_encode____________() {
        let s: &str = "___________";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!("X19fX19fX19fX18=", b64.to_string());
    }

    #[test]
    fn base64_encode_short_sentence() {
        let s: &str = "This is a short sentence.";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!("VGhpcyBpcyBhIHNob3J0IHNlbnRlbmNlLg==", b64.to_string());
    }

    #[test]
    fn base64_encode_long_sentence() {
        let s: &str = "This is a way longer more long winded sentence.";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!(
            "VGhpcyBpcyBhIHdheSBsb25nZXIgbW9yZSBsb25nIHdpbmRlZCBzZW50ZW5jZS4=",
            b64.to_string()
        );
    }

    #[test]
    fn base64_encode_paragraph() {
        let s: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Dictum fusce ut placerat orci nulla pellentesque. Consequat mauris nunc congue nisi vitae suscipit tellus mauris a.";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!("TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdCwg\nc2VkIGRvIGVpdXNtb2QgdGVtcG9yIGluY2lkaWR1bnQgdXQgbGFib3JlIGV0IGRvbG9yZSBtYWduY\nSBhbGlxdWEuIERpY3R1bSBmdXNjZSB1dCBwbGFjZXJhdCBvcmNpIG51bGxhIHBlbGxlbnRlc3F1ZS\n4gQ29uc2VxdWF0IG1hdXJpcyBudW5jIGNvbmd1ZSBuaXNpIHZpdGFlIHN1c2NpcGl0IHRlbGx1cyB\ntYXVyaXMgYS4=", b64.to_string());
    }

    #[test]
    fn base64_decode_hello_world() {
        let s: &str = "Hello, World";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!(
            "Hello, World",
            String::from_utf8(b64.decode_to_bytes()).unwrap()
        );
    }

    #[test]
    fn base64_decode_short_sentence() {
        let s: &str = "This is a short sentence.";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!(
            "This is a short sentence.",
            String::from_utf8(b64.decode_to_bytes()).unwrap()
        );
    }

    #[test]
    fn base64_decode____________() {
        let s: &str = "___________";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!(
            "___________",
            String::from_utf8(b64.decode_to_bytes()).unwrap()
        );
    }

    #[test]
    fn base64_decode_long_sentence() {
        let s: &str = "This is a way longer more long winded sentence.";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!(
            "This is a way longer more long winded sentence.",
            String::from_utf8(b64.decode_to_bytes()).unwrap()
        );
    }

    #[test]
    fn base64_decode_paragraph() {
        let s: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Dictum fusce ut placerat orci nulla pellentesque. Consequat mauris nunc congue nisi vitae suscipit tellus mauris a.";
        let b64: Base64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
        assert_eq!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Dictum fusce ut placerat orci nulla pellentesque. Consequat mauris nunc congue nisi vitae suscipit tellus mauris a.", String::from_utf8(b64.decode_to_bytes()).unwrap());
    }
}
