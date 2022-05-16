extern crate blc;
extern crate genawaiter;

mod lambda;
mod pair_list;

use genawaiter::{
    sync::{gen, Gen, GenBoxed},
    yield_,
};
use lambda_calculus::{beta, Term, Var, NOR};
use std::future::Future;

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

fn tru() -> String {
    lam(lam(var(1)))
}

fn paren(s: String) -> String {
    s
}

fn app(a: String, b: String) -> String {
    paren(cat!("01", a, b))
}

fn pair_fn() -> String {
    lam(lam(lam(app(app(var(0), var(2)), var(1)))))
}

fn pair(a: String, b: String) -> String {
    app(app(pair_fn(), a), b)
}

fn list(l: &[String]) -> String {
    let mut r = fls();
    for s in l {
        r = pair(s.to_string(), r);
    }
    r
}

fn exec_and_print(s: String) {
    println!("{}", exec(s.as_str()).unwrap())
}

fn main() {
    exec_and_print(lam(list(&[tru()])));

    for prog in bitstrings() {
        match exec(prog.as_str()) {
            Some(s) => {
                if s.chars().count() > prog.chars().count() {
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

fn exec(x: &str) -> Option<String> {
    lambda::decode(beta(parse_app(x)?, NOR, 1000)).ok()
}

fn parse_app(input: &str) -> Option<Term> {
    let mut iter = input.chars().peekable();
    let prog = parse(&mut iter)?;
    match iter.peek() {
        _ => Some(lambda_calculus::app(
            prog,
            lambda::encode_bits(&iter.map(|c| c as u8).collect::<Vec<u8>>()),
        )),
        // None => Some(prog),
    }
}

fn parse(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Term> {
    if chars.next()? == '0' {
        if chars.next()? == '0' {
            return Some(lambda_calculus::abs(parse(chars)?));
        }
        return Some(lambda_calculus::app(parse(chars)?, parse(chars)?));
    }
    let mut n: usize = 1;
    while chars.next()? == '1' {
        n += 1;
    }
    Some(Var(n))
}
