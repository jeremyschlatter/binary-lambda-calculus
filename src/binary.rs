//! Binary encoding for lambda `Term`s

use self::Error::*;
use lambda_calculus::term::*;

/// An error that can occur if the input stream of "bits" is not valid binary lambda calculus.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// not a valid term
    NotATerm,
}

/// Parse a blc-encoded lambda `Term`.
///
/// # Example
/// ```
/// use blc::encoding::binary::{from_bits, to_bits};
///
/// let k = from_bits(b"0000110");
///
/// assert!(k.is_ok());
/// assert_eq!(to_bits(&k.unwrap()), Vec::from(&b"0000110"[..]));
/// ```
pub fn from_bits(input: &[u8]) -> Result<Term, Error> {
    if let Some((result, _)) = _from_bits(input) {
        Ok(result)
    } else {
        Err(NotATerm)
    }
}

fn _from_bits(input: &[u8]) -> Option<(Term, &[u8])> {
    if input.is_empty() {
        return None;
    }

    if [9, 10, 13, 32].contains(&input[0]) {
        _from_bits(&input[1..]) // skip whitespaces
    } else {
        match &input[0..2] {
            b"00" => {
                if let Some((term, rest)) = _from_bits(&input[2..]) {
                    Some((abs(term), rest))
                } else {
                    None
                }
            }
            b"01" => {
                if let Some((term1, rest1)) = _from_bits(&input[2..]) {
                    if let Some((term2, rest2)) = _from_bits(rest1) {
                        Some((app(term1, term2), rest2))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            b"10" | b"11" => {
                let i = input.iter().take_while(|&b| *b == b'1').count();
                if input[2..].is_empty() {
                    Some((Var(i), &*b""))
                } else {
                    Some((Var(i), &input[i + 1..]))
                }
            }
            _ => None,
        }
    }
}

/// Represent a lambda `Term` in blc.
///
/// # Example
/// ```
/// use blc::encoding::binary::{from_bits, to_bits};
///
/// let k = from_bits(b"0000110");
///
/// assert!(k.is_ok());
/// assert_eq!(to_bits(&k.unwrap()), Vec::from(&b"0000110"[..]));
/// ```
pub fn to_bits(term: &Term) -> Vec<u8> {
    let mut output = Vec::new();
    _to_bits(term, &mut output);

    output
}

fn _to_bits(term: &Term, output: &mut Vec<u8>) {
    match *term {
        Var(i) => {
            for _ in 0..i {
                output.push(b'1')
            }
            output.push(b'0');
        }
        Abs(ref t) => {
            output.extend_from_slice(b"00");
            output.append(&mut to_bits(t));
        }
        App(ref t) => {
            output.extend_from_slice(b"01");
            output.append(&mut to_bits(&t.0));
            output.append(&mut to_bits(&t.1));
        }
    }
}

/// Convert a stream of "bits" into bytes. It is not always reversible with `decompress`, because
/// it produces full bytes, while the length of its input can be indivisible by 8.
///
/// # Example
/// ```
/// use blc::encoding::binary::{compress};
///
/// let succ_compressed = compress(&*b"000000011100101111011010");
/// assert_eq!(succ_compressed, vec![0x1, 0xCB, 0xDA]);
/// ```
pub fn compress(bits: &[u8]) -> Vec<u8> {
    let length = bits.len();
    let mut output = Vec::with_capacity(length / 8 + 1);
    let mut pos = 0;

    while pos <= length - 8 {
        output.push(bits_to_byte(&bits[pos..(pos + 8)]));
        pos += 8;
    }

    if pos != length {
        let mut last_byte = Vec::with_capacity(8);
        last_byte.extend_from_slice(&bits[pos..]);
        for _ in 0..(8 - (length - pos)) {
            last_byte.push(b'0')
        }
        output.push(bits_to_byte(&last_byte));
    }

    output
}

fn bits_to_byte(bits: &[u8]) -> u8 {
    bits.iter().fold(0, |acc, &b| acc * 2 + (b - 48))
}

/// Convert bytes into "bits" suitable for binary lambda calculus purposes.
///
/// # Example
/// ```
/// use blc::encoding::binary::decompress;
///
/// let succ_compressed = vec![0x1, 0xCB, 0xDA];
///
/// assert_eq!(decompress(&succ_compressed), b"000000011100101111011010");
/// ```
pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len() * 8);

    for byte in bytes {
        output.extend_from_slice(format!("{:08b}", byte).as_bytes());
    }

    output
}
