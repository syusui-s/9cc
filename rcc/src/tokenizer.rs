use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Int64(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizeError {
    ParseIntError,
    IllegalCharacter(char),
}

fn tokenize_number(chars: &mut Peekable<Chars>) -> Result<Token, TokenizeError> {
    use std::num::ParseIntError;

    let mut value = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_numeric() {
            value.push(c);
            chars.next();
            continue;
        }

        break;
    }

    let value = value.parse().map_err(|err| match err {
        ParseIntError { .. } => TokenizeError::ParseIntError,
    })?;

    Ok(Token::Int64(value))
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut chars = input.chars().peekable();
    let mut result = Vec::new();

    while let Some(&ch) = chars.peek() {
        use self::Token::*;

        match ch {
            ' ' => (),
            '+' => result.push(Plus),
            '-' => result.push(Minus),
            '*' => result.push(Asterisk),
            '/' => result.push(Slash),
            number if number.is_numeric() => {
                result.push(tokenize_number(&mut chars)?);
                continue;
            },
            _ => return Err(TokenizeError::IllegalCharacter(ch)),
        }

        chars.next();
    }

    Ok(result)
}

#[test]
fn test_tokenize() {
    use self::Token::*;
    use self::tokenize;

    /*******************************************/

    let result = tokenize("221 + 212");
    let expected = vec![
        Int64(221),
        Plus,
        Int64(212),
    ];
    assert!(result == Ok(expected));

    /*******************************************/

    let result = tokenize("1-2");
    let expected = vec![
        Int64(1),
        Minus,
        Int64(2),
    ];

    assert!(result == Ok(expected));

    /*******************************************/

    let result = tokenize("1+2-3*4/5");
    let expected = vec![
        Int64(1),
        Plus,
        Int64(2),
        Minus,
        Int64(3),
        Asterisk,
        Int64(4),
        Slash,
        Int64(5),
    ];

    assert!(result == Ok(expected));

    /*******************************************/

    let result = tokenize("5 / 2");
    let expected = vec![
        Int64(5),
        Slash,
        Int64(2),
    ];

    assert!(result == Ok(expected));

    /*******************************************/

    let result = tokenize("5 * 2");
    let expected = vec![
        Int64(5),
        Asterisk,
        Int64(2),
    ];

    assert!(result == Ok(expected));

    /*******************************************/

    let result = tokenize("$#!");
    let expected = TokenizeError::IllegalCharacter('$');
    assert!(result == Err(expected));

    /*******************************************/

    let result = tokenize("0 - 12345678901234567890");
    let expected = TokenizeError::ParseIntError;
    assert!(result == Err(expected));
}
