//! Lambda encoding for strings of bytes

use crate::pair_list::*;
use lambda_calculus::data::boolean::{fls, tru};
use lambda_calculus::term::*;
use std::char;

/// An error that can occur if the input stream of "bits" is not valid binary lambda calculus.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// not a valid term
    NotATerm,
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
    } else if is_list(&term) && is_list(head_ref(&term).unwrap()) {
        // safe
        let (head, tail) = uncons(term).unwrap(); // safe
        let byte = decode_byte(head)?;
        let chr = char::from(byte);
        Ok(chr.to_string() + &decode(tail)?)
    } else if head_ref(&term) == Ok(&fls()) {
        Ok("1".to_string() + &decode(tail(term).unwrap())?) // safe
    } else if head_ref(&term) == Ok(&tru()) {
        Ok("0".to_string() + &decode(tail(term).unwrap())?) // safe
    } else {
        // Ok(format!("({:?})", term))
        Err(Error::NotATerm)
    }
}

fn decode_byte(encoded_byte: Term) -> Result<u8, Error> {
    let indices = vectorize_list(encoded_byte)
        .into_iter()
        .map(|t| t.unabs().and_then(|t| t.unabs()).and_then(|t| t.unvar()))
        .collect::<Result<Vec<usize>, TermError>>();

    if let Ok(indices) = indices {
        Ok(!indices
            .into_iter()
            .map(|b| (b - 1) as u8)
            .fold(0, |acc, b| acc * 2 + b))
    } else {
        Err(Error::NotATerm)
    }
}

fn encode_byte(byte: u8) -> Term {
    let bitstr = format!("{:08b}", byte);
    let bits = bitstr.as_bytes();
    listify_terms(
        bits.into_iter()
            .map(|&bit| encode_bit(bit))
            .collect::<Vec<Term>>(),
    )
}

fn encode_bit(bit: u8) -> Term {
    match bit {
        b'0' => tru(),
        b'1' => fls(),
        _ => unreachable!(),
    }
}

/// Encode bytes as a lambda `Term`.
///
/// # Example
/// ```
/// use blc::encoding::lambda::encode;
///
/// assert_eq!(
///     &*format!("{:?}", encode(b"a")),
///     "λ1(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1)"
/// );
/// ```
pub fn encode(input: &[u8]) -> Term {
    listify_terms(
        input
            .into_iter()
            .map(|&b| encode_byte(b))
            .collect::<Vec<Term>>(),
    )
}

pub fn encode_bits(input: &[u8]) -> Term {
    listify_terms(
        input
            .into_iter()
            .map(|&b| encode_bit(b))
            .collect::<Vec<Term>>(),
    )
}
