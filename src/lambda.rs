//! Lambda encoding for strings of bytes
//!
//! Copied from https://github.com/ljedrz/blc/blob/6a673cf1b3b1a689e799912ab2b9ef852b810c04/src/encoding/lambda.rs

use crate::pair_list::*;
use lambda_calculus::data::boolean::{fls, tru};
use lambda_calculus::term::*;

/// An error that can occur if the input stream of "bits" is not valid binary lambda calculus.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// not a valid term
    NotAList,
}

/// Decode lambda-encoded data as a `String`.
///
/// # Example
/// ```
/// use blc::encoding::binary::from_bits;
/// use blc::encoding::lambda::decode;
///
/// let k = from_bits(b"0000110").unwrap();
///
/// assert_eq!(decode(k).unwrap(), "(λλ2)");
/// ```
pub fn decode(term: Term) -> Result<String, Error> {
    if term == fls() {
        Ok("".into())
    } else if head_ref(&term) == Ok(&fls()) {
        Ok("0".to_string() + &decode(tail(term).unwrap())?) // safe
    } else if head_ref(&term) == Ok(&tru()) {
        Ok("1".to_string() + &decode(tail(term).unwrap())?) // safe
    } else {
        Err(Error::NotAList)
    }
}

fn encode_bit(bit: u8) -> Term {
    match bit {
        1 => tru(),
        0 => fls(),
        _ => unreachable!(),
    }
}

pub fn encode_bits(input: &[u8]) -> Term {
    listify_terms(
        input
            .into_iter()
            .map(|&b| encode_bit(b))
            .collect::<Vec<Term>>(),
    )
}
