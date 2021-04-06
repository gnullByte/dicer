use dicer::*;
use lex::*;
use std::convert::TryInto;

fn main() {
    let test = String::from(" ** / +++ -   *");
    let len: i32 = test.len().try_into().unwrap();
    let mut pos = 0;

    println!("\"{}\"", test);
    while pos != len {
        let tok = nfa(pos, &test);
        println!("{}", token_type_to_str(tok.ttype));
        pos = tok.f;
    }
}
