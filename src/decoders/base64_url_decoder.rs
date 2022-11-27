///! Decode a base64_url string
///! Performs error handling and returns a string
///! Call base64_url_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///
use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

/// The base64_url decoder, call:
/// `let base64_url_decoder = Decoder::<Base64URLDecoder>::new()` to create a new instance
/// And then call:
/// `result = base64_url_decoder.crack(input)` to decode a base64_url string
/// The struct generated by new() comes from interface.rs
/// ```
/// use ares::decoders::base64_url_decoder::{Base64URLDecoder};
/// use ares::decoders::interface::{Crack, Decoder};
/// use ares::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decode_base64_url = Decoder::<Base64URLDecoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decode_base64_url.crack("aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbS8_ZXhhbXBsZT10ZXN0", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap(), "https://www.google.com/?example=test");
/// ```
pub struct Base64URLDecoder;

impl Crack for Decoder<Base64URLDecoder> {
    fn new() -> Decoder<Base64URLDecoder> {
        Decoder {
            name: "base64_url",
            description: "Modified Base64 for URL variants exist (such as base64url in RFC 4648), where the '+' and '/' characters of standard Base64 are respectively replaced by '-' and '_', so that using URL encoders/decoders is no longer necessary.",
            link: "https://en.wikipedia.org/wiki/Base64#URL_applications",
            tags: vec!["base64_url", "base64", "url", "decoder", "base"],
            expected_runtime: 0.01,
            expected_success: 1.0,
            failure_runtime: 0.01,
            normalised_entropy: vec![1.0, 10.0],
            popularity: 0.9,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying base64_url with text {:?}", text);
        let decoded_text = decode_base64_url_no_error_handling(text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode base64_url because Base64URLDecoder::decode_base64_url_no_error_handling returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode base64_url because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

        results
    }
}

/// helper function
fn decode_base64_url_no_error_handling(text: &str) -> Option<String> {
    // Runs the code to decode base64_url
    // Doesn't perform error handling, call from_base64_url
    base64::decode_config(text.as_bytes(), base64::URL_SAFE)
        .ok()
        .map(|inner| String::from_utf8(inner).ok())?
}

#[cfg(test)]
mod tests {
    use super::Base64URLDecoder;
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
    fn base64_url_decodes_successfully() {
        // This tests if Base64 URL can decode Base64 URL successfully
        // Regular Base64 can't decode this string as it has "_" instead of "/"
        let base64_url_decoder = Decoder::<Base64URLDecoder>::new();
        let result = base64_url_decoder.crack(
            "aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbS8_ZXhhbXBsZT10ZXN0",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "https://www.google.com/?example=test"
        );
    }

    #[test]
    fn base64_url_decodes_regular_base64_successfully() {
        // This tests if Base64 URL can decode regular Base64 successfully
        // Base64 URL can decode Base64 strings if they don't have "+" or "/" in them
        let base64_url_decoder = Decoder::<Base64URLDecoder>::new();
        let result = base64_url_decoder.crack(
            "VGhpcyBpcyBkZWNvZGFibGUgYnkgYm90aCBCYXNlNjQgYW5kIEJhc2U2NCBVUkw=",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "This is decodable by both Base64 and Base64 URL"
        );
    }

    #[test]
    fn base64_url_handles_regular_base64_with_plus_signs() {
        // This tests if Base64 URL can handle regular Base64 with plus signs
        // Base64 URL can't decode Base64 strings that have "+" in them as it's replaced with "-"
        // It should return None
        let base64_url_decoder = Decoder::<Base64URLDecoder>::new();
        let result = base64_url_decoder
            .crack(
                "VGhpcyBpc24ndCA+Pj4+IGRlY29kYWJsZSBieSBCYXNlNjQgVVJM",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base64_url_handles_regular_base64_with_slashes() {
        // This tests if Base64 URL can handle regular Base64 with slashes
        // Base64 URL can't decode Base64 strings that have "/" in them as it's replaced with "_"
        // It should return None
        let base64_url_decoder = Decoder::<Base64URLDecoder>::new();
        let result = base64_url_decoder
            .crack(
                "aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbS8/ZXhhbXBsZT10ZXN0",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base64_url_handles_panics() {
        // This tests if Base64 URL can handle panics
        // It should return None
        let base64_url_decoder = Decoder::<Base64URLDecoder>::new();
        let result = base64_url_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base64_url_handles_panic_if_empty_string() {
        // This tests if Base64 URL can handle an empty string
        // It should return None
        let base64_url_decoder = Decoder::<Base64URLDecoder>::new();
        let result = base64_url_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base64_url_handles_panic_if_emoji() {
        // This tests if Base64 URL can handle an emoji
        // It should return None
        let base64_url_decoder = Decoder::<Base64URLDecoder>::new();
        let result = base64_url_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
