extern crate blc;
extern crate genawaiter;
// extern crate lambda_calculus;

// mod binary;
mod lambda;
mod pair_list;

// use blc::*;
// use blc::execution::Input;
// use lambda_calculus::{parse, DeBruijn};
// use blc::execution::Input;
// use blc::run;
use genawaiter::{
    sync::{gen, Gen, GenBoxed},
    yield_,
};
use lambda_calculus::{abs, app, beta, Term, Var, NOR};
use std::future::Future;

fn main() {
    // let prog = b"0010";
    //     // let prog = b"00100101";
    // print!("{:?}", run(&*prog, Input::Bytes(b"0101")));

    for prog in bitstrings() {
        match exec(prog.as_str()) {
            Some(s) => println!("{} -> {}", prog, s),
            None => (),
        }
    }
}

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
    let mut iter = input.chars();
    Some(app(
        parse(&mut iter)?,
        lambda::encode_bits(&iter.map(|c| c as u8).collect::<Vec<u8>>()),
    ))
}

fn parse(chars: &mut std::str::Chars) -> Option<Term> {
    if chars.next()? == '0' {
        if chars.next()? == '0' {
            return Some(abs(parse(chars)?));
        }
        return Some(app(parse(chars)?, parse(chars)?));
    }
    let mut n: usize = 1;
    while chars.next()? == '1' {
        n += 1;
    }
    Some(Var(n))
}
