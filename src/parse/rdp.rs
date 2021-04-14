use crate::lex::constants;
use crate::lex::Token;
use crate::parse;

use rand::{thread_rng, Rng};
use std::cmp;

pub fn parse_expression(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_NUM
        || token.ttype == constants::TOKEN_LPAREN
        || token.ttype == constants::TOKEN_FMIN
        || token.ttype == constants::TOKEN_FMAX
        || token.ttype == constants::TOKEN_LBRACKET
    {
        parse_simple_expression(token, src);
    } else if token.ttype == constants::TOKEN_FCOUNT {
        parse_fcount(token, src);
    }
    print!("\n");
    parse::match_t(constants::TOKEN_EOF, token, src).unwrap();
}

pub fn parse_simple_expression(token: &mut Token, src: &String) {
    parse_term(token, src);
    parse_simple_expression_tail(token, src);
}

pub fn parse_simple_expression_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_ADDOP {
        let op = token.lexeme.clone();
        let result = token.carry;
        print!(" {} ", op);
        parse::match_t(constants::TOKEN_ADDOP, token, src).unwrap();

        parse_term(token, src);

        if op == "+" {
            token.carry = result + token.carry;
        } else if op == "-" {
            token.carry = result - token.carry;
        }

        parse_simple_expression_tail(token, src);
    }
}

pub fn parse_term(token: &mut Token, src: &String) {
    parse_factor(token, src);
    parse_term_tail(token, src);
}

pub fn parse_term_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_MULOP {
        let op = token.lexeme.clone();
        let result = token.carry;

        parse::match_t(constants::TOKEN_MULOP, token, src).unwrap();

        parse_factor(token, src);
        if op == "*" {
            token.carry = result * token.carry;
        } else if op == "/" {
            token.carry = result / token.carry;
        }

        parse_term_tail(token, src);
    } else if token.ttype == constants::TOKEN_ADDOP
        || token.ttype == constants::TOKEN_COMMA
        || token.ttype == constants::TOKEN_RPAREN
        || token.ttype == constants::TOKEN_EOF
    {
    }
}

pub fn parse_factor(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_NUM {
        token.carry = token.attr;

        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();

        parse_factor_tail(token, src);
        print!("{}", token.carry);
    } else if token.ttype == constants::TOKEN_LBRACKET {
        parse::match_t(constants::TOKEN_LBRACKET, token, src).unwrap();

        let num_dice = token.attr;
        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
        parse::match_t(constants::TOKEN_D, token, src).unwrap();

        let num_sides = token.attr;
        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();

        token.result = roll(num_dice, num_sides);
        token.carry = token.result.0;
        let op = token.lexeme.clone();
        print!("[ {} {} ", token.carry, op);

        parse::match_t(constants::TOKEN_ADDOP, token, src).unwrap();
        let extrema = token.lexeme.clone();
        parse::match_t(constants::TOKEN_EXTREMA, token, src).unwrap();

        if op == "+" && extrema == "MAX" {
            print!("{} ]", token.result.1);
            token.carry += token.result.1;
        } else if op == "-" && extrema == "MAX" {
            print!("{} ]", token.result.1);
            token.carry -= token.result.1;
        } else if op == "+" && extrema == "MIN" {
            print!("{} ]", token.result.2);
            token.carry += token.result.2;
        } else if op == "-" && extrema == "MIN" {
            print!("{} ]", token.result.2);
            token.carry -= token.result.2;
        }

        parse::match_t(constants::TOKEN_RBRACKET, token, src).unwrap();
    } else if token.ttype == constants::TOKEN_LPAREN {
        parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();
        parse_simple_expression(token, src);
        parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
    } else if token.ttype == constants::TOKEN_FMIN {
        parse_fmin(token, src);
    } else if token.ttype == constants::TOKEN_FMAX {
        parse_fmax(token, src);
    } else {
        panic!("SYNERR");
    }
}

pub fn parse_factor_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_D {
        parse::match_t(constants::TOKEN_D, token, src).unwrap();

        token.result = roll(token.carry, token.attr);
        token.carry = token.result.0;

        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
    }
}

pub fn parse_fmin(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FMIN, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();
    print!("min(");
    parse_simple_expression(token, src);
    print!(", ");
    let first = token.carry;

    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();
    parse_simple_expression(token, src);
    let second = token.carry;
    print!(")");
    token.carry = cmp::min(first, second);
    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_fmax(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FMAX, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();
    parse_simple_expression(token, src);
    let first = token.carry;

    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();
    parse_simple_expression(token, src);
    let second = token.carry;
    token.carry = cmp::max(first, second);
    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_fcount(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FCOUNT, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
    parse::match_t(constants::TOKEN_D, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

    parse_condition_list(token, src);

    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_condition_list(token: &mut Token, src: &String) {
    parse_condition(token, src);

    parse_condition_list_tail(token, src);
}

pub fn parse_condition(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_RELOP, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
}

pub fn parse_condition_list_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_COMMA {
        parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

        parse_condition(token, src);

        parse_condition_list_tail(token, src);
    }
}

pub fn roll(n: i32, m: i32) -> (i32, i32, i32) {
    let mut rng = thread_rng();
    let mut sum = 0;
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    let mut intermediate;

    for _ in 1..(n + 1) {
        intermediate = rng.gen_range(1..(m + 1));
        if intermediate > max {
            max = intermediate;
        }

        if intermediate < min {
            min = intermediate;
        }

        sum += intermediate;
    }
    (sum, max, min)
}
