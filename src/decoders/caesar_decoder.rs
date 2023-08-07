//! Decode a caesar cipher string
//! Performs error handling and returns a string
//! Call caesar_decoder.crack to use. It returns option<String> and check with
//! `result.is_some()` to see if it returned okay.

use crate::checkers::{CheckerTypes};
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{info, trace};

/// The caesar decoder, call:
/// `let caesar_decoder = Decoder::<caesarDecoder>::new()` to create a new instance
/// And then call:
/// `result = caesar_decoder.crack(input)` to decode a caesar string
/// The struct generated by new() comes from interface.rs
/// ```
/// use ares::decoders::caesar_decoder::CaesarDecoder;
/// use ares::decoders::interface::{Crack, Decoder};
/// use ares::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decode_caesar = Decoder::<CaesarDecoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decode_caesar.crack("uryyb guvf vf ybat grkg", &checker).unencrypted_text;
/// assert!(result.is_some());
/// // If it succeeds, the 0th element is the plaintext else it'll contain 25 elements
/// // of unsuccessfully decoded text
/// assert_eq!(result.unwrap()[0], "hello this is long text");
/// ```
pub struct CaesarDecoder;

impl Crack for Decoder<CaesarDecoder> {
    fn new() -> Decoder<CaesarDecoder> {
        Decoder {
            name: "Caesar Cipher",
            description: "Caesar cipher, also known as Caesar's cipher, the shift cipher, Caesar's code or Caesar shift, is one of the simplest and most widely known encryption techniques. It is a type of substitution cipher in which each letter in the plaintext is replaced by a letter some fixed number of positions down the alphabet.",
            link: "https://en.wikipedia.org/wiki/Caesar_cipher",
            tags: vec!["caesar", "decryption", "classic", "reciprocal"],
            popularity: 1.0,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Caesar Cipher with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());
        let mut decoded_strings = Vec::new();
        for shift in 1..=25 {
            let decoded_text = caesar(text, shift);
            decoded_strings.push(decoded_text);
            let borrowed_decoded_text = &decoded_strings[decoded_strings.len() - 1];
            if !check_string_success(borrowed_decoded_text, text) {
                info!(
                    "Failed to decode caesar because check_string_success returned false on string {}. This means the string is 'funny' as it wasn't modified.",
                    borrowed_decoded_text
                );
                return results;
            }
            let checker_result = checker.check(borrowed_decoded_text);
            // If checkers return true, exit early with the correct result
            if checker_result.is_identified {
                trace!("Found a match with caesar shift {}", shift);
                results.unencrypted_text = Some(vec![borrowed_decoded_text.to_string()]);
                results.update_checker(&checker_result);
                return results;
            }
        }
        results.unencrypted_text = Some(decoded_strings);
        results
    }
    /// Gets all tags for this decoder
    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }
    /// Gets the name for the current decoder
    fn get_name(&self) -> &str {
        self.name
    }
}

/// Caesar cipher to rotate cipher text by shift and return an owned String.
fn caesar(cipher: &str, shift: u8) -> String {
    cipher
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // modulo the distance to keep character range
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::CaesarDecoder;
    use super::*;
    use crate::{
        checkers::{
            athena::Athena,
            checker_type::{Check, Checker},
            CheckerTypes,
        },
        decoders::interface::{Crack, Decoder},
    };

    // helper for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn empty() {
        assert_eq!(caesar("", 13), "");
    }

    #[test]
    fn caesar_rot_13() {
        assert_eq!(caesar("rust", 13), "ehfg");
    }

    #[test]
    fn caesar_unicode() {
        assert_eq!(caesar("attack at dawn 攻", 5), "fyyfhp fy ifbs 攻");
    }

    #[test]
    fn successful_decoding() {
        let caesar_decoder = Decoder::<CaesarDecoder>::new();

        let result = caesar_decoder.crack("fyyfhp", &get_athena_checker());
        let decoded_str = &result
            .unencrypted_text
            .expect("No unencrypted text for caesar");
        assert_eq!(decoded_str[0], "attack");
    }

    #[test]
    fn successful_decoding_one_step_forward() {
        let caesar_decoder = Decoder::<CaesarDecoder>::new();

        let result = caesar_decoder.crack("buubdl", &get_athena_checker());
        let decoded_str = &result
            .unencrypted_text
            .expect("No unencrypted text for caesar");
        assert_eq!(decoded_str[0], "attack");
    }

    #[test]
    fn successful_decoding_one_step_backward() {
        let caesar_decoder = Decoder::<CaesarDecoder>::new();

        let result = caesar_decoder.crack("zsszbj", &get_athena_checker());
        let decoded_str = &result
            .unencrypted_text
            .expect("No unencrypted text for caesar");
        assert_eq!(decoded_str[0], "attack");
    }

    #[test]
    fn successful_decoding_longer_text() {
        let caesar_decoder = Decoder::<CaesarDecoder>::new();

        let result = caesar_decoder.crack("uryyb guvf vf ybat grkg", &get_athena_checker());
        let decoded_str = &result
            .unencrypted_text
            .expect("No unencrypted text for caesar");
        assert_eq!(decoded_str[0], "hello this is long text");
    }

    #[test]
    fn successful_decoding_longer_text_with_puncuation() {
        let caesar_decoder = Decoder::<CaesarDecoder>::new();

        let result = caesar_decoder.crack("Uryyb! guvf vf ybat grkg?", &get_athena_checker());
        let decoded_str = &result
            .unencrypted_text
            .expect("No unencrypted text for caesar");
        assert_eq!(decoded_str[0], "Hello! this is long text?");
    }

    #[test]
    fn caesar_decode_empty_string() {
        // caesar returns an empty string, this is a valid caesar string
        // but returns False on check_string_success
        let caesar_decoder = Decoder::<CaesarDecoder>::new();
        let result = caesar_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn caesar_decode_fails() {
        let caesar_decoder = Decoder::<CaesarDecoder>::new();
        let result = caesar_decoder
            .crack("#", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            panic!("Decode_caesar did not return an option with Some<t>.")
        } else {
            // If we get here, the test passed
            // Because the caesar_decoder.crack function returned None
            // as it should do for the input
            assert!(true)
        }
    }
}
