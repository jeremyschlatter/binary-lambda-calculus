extern crate blc;
// extern crate lambda_calculus;

mod binary;
mod lambda;
mod pair_list;

// use blc::*;
// use blc::execution::Input;
// use lambda_calculus::{parse, DeBruijn};
use blc::execution::Input;
use blc::run;
use lambda_calculus::{abs, app, beta, Term, Var, NOR};

fn main() {
    let prog = b"0010";
    //     // let prog = b"00100101";
    print!("{:?}", run(&*prog, Input::Bytes(b"0101")));

    //     loop {
    //     }

    for prog in 0..10 {
        print!(
            "{}\n",
            exec(prog).unwrap_or("None".to_string()) // parse_int(prog).map(|t| lambda::decode(beta(t, NOR, 100)))
        );
    }
}

fn exec(x: i32) -> Option<String> {
    lambda::decode(beta(parse_int(x)?, NOR, 100)).ok()
}

fn parse_int(input: i32) -> Option<Term> {
    let s = format!("{:b}", input);
    let mut iter = s.chars();
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
