use std::sync::mpsc::channel;

use crate::checkers::CheckerTypes;
///! Proposal: https://broadleaf-angora-7db.notion.site/Filtration-System-7143b36a42f1466faea3077bfc7e859e
///! Given a filter object, return an array of decoders/crackers which have been filtered
///
use crate::decoders::base64_decoder::Base64Decoder;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::{Crack, Decoder};
use crate::decoders::reverse_decoder::ReverseDecoder;

use log::trace;
use rayon::prelude::*;

/// The struct which contains all of the decoders
/// Where decoders is crackers, decryptors, etc.
/// This contains a public attribute Components
/// Which contains all of them. See `pub fn run` which is impl'd on
/// the Decoders for the Crack trait in action.
/// Relevant docs: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
pub struct Decoders {
    pub components: Vec<Box<dyn Crack + Sync>>,
}

/// [`Enum`] for our custom results.
/// if our checker succeed, we return `Break` variant contining [`CrackResult`]
/// else we return `Continue` with the decoded results.
pub enum MyResults {
    Break(CrackResult),
    Continue(Vec<CrackResult>),
}

impl MyResults {
    // named with _ to pass dead_code warning
    // as we aren't using it, it's just used in tests
    pub fn _break_value(self) -> Option<CrackResult> {
        match self {
            MyResults::Break(val) => Some(val),
            MyResults::Continue(_) => None,
        }
    }
}

impl Decoders {
    /// Iterate over all of the decoders and run .crack(text) on them
    /// Then if the checker succeed, we short-circuit the iterator
    /// and stop all processing as soon as possible.
    /// We are using Trait Objects
    /// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
    /// Which allows us to have multiple different structs in the same vector
    /// But each struct shares the same `.crack()` method, so it's fine.
    pub fn run(&self, text: &str, checker: CheckerTypes) -> MyResults {
        trace!("Running .crack() on all decoders");
        let (sender, receiver) = channel();
        self.components
            .into_par_iter()
            .try_for_each_with(sender, |s, i| {
                let results = i.crack(text, &checker);
                if results.success {
                    s.send(results).expect("expected no send error!");
                    // returning None short-circuits the iterator
                    // we don't process any further as we got success
                    return None;
                }
                s.send(results).expect("expected no send error!");
                // return Some(()) to indicate that continue processing
                Some(())
            });

        let mut all_results: Vec<CrackResult> = Vec::new();

        while let Ok(result) = receiver.recv() {
            // if we recv success, break.
            if result.success {
                return MyResults::Break(result);
            }
            all_results.push(result)
        }

        MyResults::Continue(all_results)
    }
}

/// Currently takes no args as this is just a spike to get all the basic functionality working
pub fn filter_and_get_decoders() -> Decoders {
    trace!("Filtering and getting all decoders");
    let base64 = Decoder::<Base64Decoder>::new();
    let reversedecoder = Decoder::<ReverseDecoder>::new();
    Decoders {
        components: vec![Box::new(base64), Box::new(reversedecoder)],
    }
}

#[cfg(test)]
mod tests {
    use crate::checkers::{
        athena::Athena,
        checker_type::{Check, Checker},
        CheckerTypes,
    };

    // TODO: when we add a proper filtration system
    // We need to test that.
    use super::filter_and_get_decoders;

    #[test]
    fn it_works() {
        let _decoders = filter_and_get_decoders();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn decoders_can_call_dot_run() {
        let decoders = filter_and_get_decoders();
        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        decoders.run("TXIgUm9ib3QgaXMgZ3JlYXQ=", checker);
        assert_eq!(true, true);
    }
}
