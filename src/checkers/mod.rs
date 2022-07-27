use self::{
    athena::Athena,
    checker_result::CheckResult,
    checker_type::{Check, Checker},
    english::EnglishChecker,
    lemmeknow_checker::LemmeKnow,
};

/// The default checker we use which simply calls all other checkers in order.
pub mod athena;
/// The checkerResult struct is used to store the results of a checker.
pub mod checker_result;
/// This is the base checker that all other checkers inherit from.
pub mod checker_type;
/// The default checker we use which simply calls all other checkers in order.
pub mod default_checker;
/// The English Checker is a checker that checks if the input is English
pub mod english;
/// The Human Checker asks humans if the expected plaintext is real plaintext
pub mod human_checker;
/// The LemmeKnow Checker checks if the text matches a known Regex pattern.
pub mod lemmeknow_checker;

/// CheckerTypes is a wrapper enum for Checker
pub enum CheckerTypes {
    /// Wrapper for LemmeKnow Checker
    CheckLemmeKnow(Checker<LemmeKnow>),
    /// Wrapper for English Checker
    CheckEnglish(Checker<EnglishChecker>),
    /// Wrapper for Athena Checker
    CheckAthena(Checker<Athena>),
}

impl CheckerTypes {
    /// This functions calls appropriate check function of Checker
    pub fn check(&self, text: &str) -> CheckResult {
        match self {
            CheckerTypes::CheckLemmeKnow(lemmeknow_checker) => lemmeknow_checker.check(text),
            CheckerTypes::CheckEnglish(english_checker) => english_checker.check(text),
            CheckerTypes::CheckAthena(athena_checker) => athena_checker.check(text),
        }
    }
}

// pub fn check(input: &str) -> bool {
//     // Uses lemmeknow to check if any regexes match
//     // import and call lemmeknow.rs

//     let lemmeknow_result = Checker::<LemmeKnow>::new().check(input);
//     if lemmeknow_result.is_identified {
//         return human_checker::human_checker(&lemmeknow_result);
//     }

//     let english_result = Checker::<EnglishChecker>::new().check(input);
//     if english_result.is_identified {
//         return human_checker::human_checker(&english_result);
//     }

//     false
// }

// test
#[cfg(test)]
mod tests {
    use crate::checkers::{
        athena::Athena,
        checker_type::{Check, Checker},
        CheckerTypes,
    };

    #[test]
    fn test_check_ip_address() {
        let athena = CheckerTypes::CheckAthena(Checker::<Athena>::new());
        assert!(athena.check("192.168.0.1").is_identified);
    }

    #[test]
    fn test_check_goes_to_dictionary() {
        let athena = CheckerTypes::CheckAthena(Checker::<Athena>::new());
        assert!(athena.check("and").is_identified);
    }
}
