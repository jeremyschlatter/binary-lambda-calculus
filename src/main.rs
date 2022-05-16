extern crate blc;
extern crate genawaiter;

mod lambda;
mod pair_list;

use genawaiter::{
    sync::{gen, Gen, GenBoxed},
    yield_,
};
use lambda_calculus::{app, beta, combinators::*, Term, Var, NOR};
use std::{future::Future, iter::Iterator};

macro_rules! cat {
    ($x:expr) => (format!("{}", $x));
    ($x:expr, $($y:expr),+) => (format!("{}{}", $x, cat!($($y),+)));
}

fn var(n: u32) -> String {
    let mut r = "1".to_string();
    for _ in 0..n {
        r.push('1');
    }
    r.push('0');
    paren(r)
}

fn lam(s: String) -> String {
    paren(cat!("00", s))
}

fn fls() -> String {
    lam(lam(var(0)))
}

#[allow(dead_code)]
fn tru() -> String {
    lam(lam(var(1)))
}

fn paren(s: String) -> String {
    s
}

fn my_app(a: String, b: String) -> String {
    paren(cat!("01", a, b))
}

fn pair_fn() -> String {
    lam(lam(lam(my_app(my_app(var(0), var(2)), var(1)))))
}

fn pair(a: String, b: String) -> String {
    my_app(my_app(pair_fn(), a), b)
}

#[allow(dead_code)]
fn list(l: &[String]) -> String {
    let mut r = fls();
    for s in l {
        r = pair(s.to_string(), r);
    }
    r
}

#[allow(dead_code)]
fn exec_and_print(mode: Mode, s: String) {
    println!("{}", exec(mode, s.as_str()).unwrap())
}

fn main() {
    let mode = Mode::Jot;

    // exec_and_print(mode, lam(list(&[tru()])));

    for prog in bitstrings() {
        match exec(mode, prog.as_str()) {
            Some(s) => {
                if s != "" {
                    // if s.chars().count() > prog.chars().count() {
                    println!("{} -> {}", prog, s);
                } else {
                    print!("{}\r", prog);
                }
            }
            None => print!("{}\r", prog),
        }
    }
}

// Based on the recursion example in the genawaiter repo:
//   https://github.com/whatisaphone/genawaiter/blob/45c10c223b92da215e182bc3eff0d5e09bf813f4/examples/recursion.rs
fn length_n_bitstrings(n: i32) -> GenBoxed<String> {
    Gen::new_boxed(|co| async move {
        if n == 0 {
            co.yield_("".to_string()).await;
            return;
        }
        for s in length_n_bitstrings(n - 1) {
            co.yield_(format!("0{}", s)).await;
        }
        for s in length_n_bitstrings(n - 1) {
            co.yield_(format!("1{}", s)).await;
        }
    })
}

fn bitstrings() -> Gen<String, (), impl Future<Output = ()>> {
    gen!({
        for i in 0.. {
            for s in length_n_bitstrings(i) {
                yield_!(s);
            }
        }
    })
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum Mode {
    BLC,
    Jot,
}

fn exec(mode: Mode, x: &str) -> Option<String> {
    lambda::decode(beta(parse_app(mode, x)?, NOR, 1000)).ok()
}

fn parse_app(mode: Mode, input: &str) -> Option<Term> {
    let mut iter = input
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
        .peekable();
    match mode {
        Mode::BLC => {
            let prog = parse_blc(&mut iter)?;
            match iter.peek() {
                _ => Some(app(
                    prog,
                    lambda::encode_bits(&iter.map(|c| c as u8).collect::<Vec<u8>>()),
                )),
                // None => Some(prog),
            }
        }
        Mode::Jot => Some(parse_jot(&mut iter.rev())),
    }
}

fn parse_blc(chars: &mut impl Iterator<Item = bool>) -> Option<Term> {
    Some(if chars.next()? {
        let mut n: usize = 1;
        while chars.next()? {
            n += 1;
        }
        Var(n)
    } else {
        if chars.next()? {
            lambda_calculus::app(parse_blc(chars)?, parse_blc(chars)?)
        } else {
            lambda_calculus::abs(parse_blc(chars)?)
        }
    })
}

fn parse_jot(chars: &mut impl Iterator<Item = bool>) -> Term {
    match chars.next() {
        Some(false) => app!(parse_jot(chars), S(), K()),
        Some(true) => app(S(), app(K(), parse_jot(chars))),
        None => I(),
    }
}
