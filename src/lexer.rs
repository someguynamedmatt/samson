use std::iter::Peekable;
use std::str::Chars;
use std::result::Result;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    Meta,
    SelectKey,
    FromKey,
    AllOp,
    Identifier(String),
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        if c == '.' {
            match get_meta(&mut iter) {
                Ok(meta) => tokens.push(meta),
                Err(err) => return Err(err),
            }
        } else if c == '*' {
            iter.next();
            tokens.push(Token::AllOp);
        } else if c.is_alphabetic() {
            match get_key_or_string(&mut iter) {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            }
        } else if c.is_whitespace() {
            iter.next();
        } else {
            return Err(String::from("Unexpected input."));
        }
    }

    return Result::Ok(tokens);
}

fn get_key_or_string(iter: &mut Peekable<Chars>) -> Result<Token, String> {
    let string_res = get_str_literal(iter);

    match string_res {
        Ok(string) => {
            let lc_string = string.to_lowercase();
            match lc_string.as_ref() {
                "select" => return Ok(Token::SelectKey),
                "from" => return Ok(Token::FromKey),
                identifier => return Ok(Token::Identifier(identifier.to_string())),
            }
        },
        Err(err) => {
            return Err(err);
        }
    }
}

fn get_meta(iter: &mut Peekable<Chars>) -> Result<Token, String> {
    iter.next(); 
    return Result::Ok(Token::Meta);
}

fn get_str_literal(iter: &mut Peekable<Chars>) -> Result<String, String> {
    let mut buff = String::new();
    while let Some(c) = iter.peek() {
        if c.is_alphabetic() {
            buff.push(*c);
            iter.next();
        } else if c.is_whitespace() {
            return Ok(buff);
        } else {
            break;
        }
    }

    return Ok(buff);
}
