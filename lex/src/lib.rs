pub mod constants;
use std::convert::TryInto;

pub struct Token {
    pub ttype: i32,
    pub lexeme: String,
    pub attr: i32,
    pub f: i32,
}

impl Token {
    pub fn new() -> Token {
        Token {
            ttype: constants::TOKEN_UNRECSYM,
            lexeme: String::from(""),
            attr: 0,
            f: 0,
        }
    }
}

pub fn nfa(pos: i32, src: &String) -> Token {
    let mut tok = Token::new();
    tok.f = pos;

    dfa_whitespace(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_catchall(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_die(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    tok.ttype = constants::TOKEN_LEXERR;
    tok
}

pub fn dfa_whitespace(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    }

    while k < len && &src.chars().nth(k.try_into().unwrap()).unwrap() == &' ' {
        k += 1;
    }

    if k > tok.f {
        tok.ttype = constants::TOKEN_WS;
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

pub fn dfa_catchall(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    } else if k == len {
        tok.ttype = constants::TOKEN_EOF;
        return;
    }

    let char = &src.chars().nth(k.try_into().unwrap()).unwrap();

    if char == &'*' || char == &'/' {
        k += 1;
        tok.ttype = constants::TOKEN_MULOP;
    } else if char == &'+' || char == &'-' {
        k += 1;
        tok.ttype = constants::TOKEN_ADDOP;
    } else if char == &',' {
        k += 1;
        tok.ttype = constants::TOKEN_COMMA;
    } else if char == &'(' {
        k += 1;
        tok.ttype = constants::TOKEN_LPAREN;
    } else if char == &')' {
        k += 1;
        tok.ttype = constants::TOKEN_RPAREN;
    }

    if k > tok.f {
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

pub fn dfa_die(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    }

    if &src.chars().nth(k.try_into().unwrap()).unwrap() == &'0' {
        return;
    }

    while k < len && src.chars().nth(k.try_into().unwrap()).unwrap().is_digit(10) {
        k += 1;
    }

    match src.chars().nth(k.try_into().unwrap()) {
        Some('d') => k += 1,
        Some('D') => k += 1,
        _ => return,
    }

    // store temp position to see if we have the required following digits
    let j = k;

    while k < len && src.chars().nth(k.try_into().unwrap()).unwrap().is_digit(10) {
        k += 1;
    }

    if k > j {
        tok.ttype = constants::TOKEN_DIE;
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}
