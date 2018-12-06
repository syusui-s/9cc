use std::iter::Peekable;
use std::slice::Iter;
use std::str::Chars;
use std::boxed::Box;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Int64(i64),
}

#[derive(Debug, Clone, PartialEq)]
enum TokenizeError {
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

fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut chars = input.chars().peekable();
    let mut result = Vec::new();

    while let Some(&ch) = chars.peek() {
        use Token::*;

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

#[derive(Debug, Clone, PartialEq)]
enum Program {
    Expr(::Expr),
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Add(Term, Box<Expr>),
    Sub(Term, Box<Expr>),
    Term(::Term),
}

/*
enum Multiply {
    Mul(Term, Box<Multiply>),
    Div(Term, Box<Multiply>),
    Term(Term),
}
*/

#[derive(Debug, Clone, PartialEq)]
enum Term {
    Int64(i64),
}

#[derive(Debug, Clone, PartialEq)]
enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

fn parse(tokens: &[Token]) -> Result<Program, ParseError> {
    let mut peekable = tokens.iter().peekable();

    parse_program(&mut peekable)
}

fn parse_program(tokens: &mut Peekable<Iter<Token>>) -> Result<Program, ParseError> {
    let result = Program::Expr(parse_expr(tokens)?);

    Ok(result)
}

fn parse_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParseError> {
    use Token::*;
    use Expr::*;

    let lhs = parse_term(tokens)?;

    if let Some(&tok) = tokens.peek() {
        let expr = match tok {
            Plus  => {
                tokens.next();
                Add(lhs, Box::new(parse_expr(tokens)?))
            },
            Minus => {
                tokens.next();
                Sub(lhs, Box::new(parse_expr(tokens)?))
            },
            tok => return Err(ParseError::UnexpectedToken(tok.clone())),
        };

        Ok(expr)
    } else {
        Ok(Expr::Term(lhs))
    }
}

fn parse_term(tokens: &mut Peekable<Iter<Token>>) -> Result<Term, ParseError> {
    use Token::*;

    if let Some(&tok) = tokens.peek() {
        let term = match tok {
            Int64(v) => {
                tokens.next();
                Term::Int64(*v)
            },
            tok => return Err(ParseError::UnexpectedToken(tok.clone())),
        };

        Ok(term)
    } else {
        Err(ParseError::UnexpectedEOF)
    }
}

fn main() {
    match tokenize("221+ 212") {
        Ok(ok) =>
            println!("OK: {:?}", ok),
        Err(err) =>
            println!("ERROR: {:?}", err),
    }
}

#[test]
fn test_parse() {
    use Token::*;

    let input = vec![Int64(221), Plus, Int64(212)];

    let result = parse(&input);
    let expected = Program::Expr(
        Expr::Add(
            Term::Int64(221),
            Box::new(Expr::Term(Term::Int64(212)))
        )
    );

    println!("{:?}", result);

    assert!(result == Ok(expected));
}

#[test]
fn test_tokenize() {
    use Token::*;

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
