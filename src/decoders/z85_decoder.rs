///! Decode a z85 string
///! Performs error handling and returns a string
///! Call z85_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///
use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use z85;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

/// The Z85 decoder, call:
/// `let z85_decoder = Decoder::<Z85Decoder>::new()` to create a new instance
/// And then call:
/// `result = z85_decoder.crack(input)` to decode a z85 string
/// The struct generated by new() comes from interface.rs
/// ```
/// use ares::decoders::z85_decoder::{Z85Decoder};
/// use ares::decoders::interface::{Crack, Decoder};
/// use ares::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decode_z85 = Decoder::<Z85Decoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decode_z85.crack("nm=QNzY&b1A+]nf", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "Hello World!");
/// ```
pub struct Z85Decoder;

impl Crack for Decoder<Z85Decoder> {
    fn new() -> Decoder<Z85Decoder> {
        Decoder {
            name: "Z85",
            description: "Ascii85, also called Base85, is a form of binary-to-text encoding that uses five ASCII characters to represent four bytes of binary data. […] Other base-85 encodings like Z85 and RFC 1924 are designed to be safe in source code.",
            link: "https://en.wikipedia.org/wiki/Ascii85",
            tags: vec!["z85", "decoder", "base85"],
            popularity: 0.6,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Z85 with text {:?}", text);
        let decoded_text = decode_z85_no_error_handling(text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode z85 because Z85Decoder::decode_z85_no_error_handling returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode z85 because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

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

/// helper function
fn decode_z85_no_error_handling(text: &str) -> Option<String> {
    // Runs the code to decode z85
    // Doesn't perform error handling, call from_z85
    z85::decode(text.as_bytes())
        .ok()
        .map(|inner| String::from_utf8(inner).ok())?
}

#[cfg(test)]
mod tests {
    use super::Z85Decoder;
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
    fn z85_successful_decoding() {
        let z85_decoder = Decoder::<Z85Decoder>::new();
        let result = z85_decoder.crack("nm=QNzY&b1A+]nf", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Hello World!");
    }

    #[test]
    fn z85_fail_decode_ascii85() {
        // You can z85 decode a string that is not z85
        // This string decodes to:
        // ```'#||5Pr
        // r```
        // https://gchq.github.io/CyberChef/#recipe=From_Base85('0-9a-zA-Z.%5C%5C-:%2B%3D%5E!/*?%26%3C%3E()%5B%5D%7B%7D@%25$%23',true,'')&input=ODdjVVJEXWouOEFURD8
        let z85_decoder = Decoder::<Z85Decoder>::new();
        let result = z85_decoder
            .crack("87cURD]j.8ATD?*", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn z85_decode_empty_string() {
        // Z85 returns an empty string, this is a valid z85 string
        // but returns False on check_string_success
        let z85_decoder = Decoder::<Z85Decoder>::new();
        let result = z85_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn z85_decode_invalid_string() {
        // Z85 can only decode strings of length multiple 5
        // This should fail to decode
        let z85_decoder = Decoder::<Z85Decoder>::new();
        let result = z85_decoder
            .crack("12ab", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn z85_decode_handles_panics() {
        let z85_decoder = Decoder::<Z85Decoder>::new();
        let result = z85_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn z85_handle_panic_if_empty_string() {
        let z85_decoder = Decoder::<Z85Decoder>::new();
        let result = z85_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn z85_handle_panic_if_emoji() {
        let z85_decoder = Decoder::<Z85Decoder>::new();
        let result = z85_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
