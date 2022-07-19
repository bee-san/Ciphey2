mod lemmeknow;
mod english;

pub fn check(input: &str) -> bool {
    // Uses lemmeknow to check if any regexes match
    // import and call lemmeknow.rs
    if lemmeknow::CheckLemmeKnow(input) {
        return true;
    }

    if english::CheckEnglish(input) {
        return true;
    }

    false
}

// test
#[cfg(test)]
mod tests {
    use crate::checkers::check;

    #[test]
    fn test_check_IP_address() {
        assert_eq!(true, check("192.168.0.1"));
    }
}