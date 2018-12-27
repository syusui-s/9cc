use std::boxed::Box;
use std::iter::Peekable;
use std::slice::Iter;

use tokenizer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Program {
    Assignment(self::LValue),
    Statement(self::Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LValue {
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expr(self::Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Add(Multiply, Box<Expr>),
    Sub(Multiply, Box<Expr>),
    Multiply(self::Multiply),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Multiply {
    Mul(Term, Box<Multiply>),
    Div(Term, Box<Multiply>),
    Term(self::Term),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Int64(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

type ParseResult<T> = Result<T, ParseError>;

pub fn parse(tokens: &[Token]) -> ParseResult<Program> {
    let mut peekable = tokens.iter().peekable();

    parse_program(&mut peekable)
}

fn parse_program(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Program> {
    let result = Program::Statement(Statement::Expr(parse_expr(tokens)?));

    Ok(result)
}

fn parse_expr(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Expr> {
    use self::Token::*;

    let lhs = parse_multiply(tokens)?;

    let expr = match tokens.peek() {
        Some(Plus)  => {
            tokens.next();
            Expr::Add(lhs, Box::new(parse_expr(tokens)?))
        },
        Some(Minus) => {
            tokens.next();
            Expr::Sub(lhs, Box::new(parse_expr(tokens)?))
        },
        _ => Expr::Multiply(lhs),
    };

    Ok(expr)
}

fn parse_multiply(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Multiply> {
    use self::Token::*;

    let lhs = parse_term(tokens)?;

    let multiply = match tokens.peek() {
        Some(Asterisk) => {
            tokens.next();
            Multiply::Mul(lhs, Box::new(parse_multiply(tokens)?))
        },
        Some(Slash) => {
            tokens.next();
            Multiply::Div(lhs, Box::new(parse_multiply(tokens)?))
        },
        _ => Multiply::Term(lhs),
    };

    Ok(multiply)
}

fn parse_term(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Term> {
    use self::Token::*;

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


#[test]
fn test_parse() {
    use tokenizer::Token::*;

    let input = vec![Int64(221), Plus, Int64(212)];

    let result = parse(&input);
    let expected = Program::Statement(
        Statement::Expr(
            Expr::Add(
                Multiply::Term(Term::Int64(221)),
                Box::new(Expr::Multiply(Multiply::Term(Term::Int64(212))))
            )
        )
    );

    assert!(result == Ok(expected));

    /*******************************************/

    let input = vec![Int64(221), Plus, Int64(212), Asterisk, Int64(122)];

    let result = parse(&input);
    println!("{:?}", result);
    let expected = Program::Statement(
        Statement::Expr(
            Expr::Add(
                Multiply::Term(Term::Int64(221)),
                Box::new(Expr::Multiply(Multiply::Mul(
                        Term::Int64(212),
                        Box::new(Multiply::Term(Term::Int64(122)))
                )))
            )
        )
    );

    assert!(result == Ok(expected));

    /*******************************************/

    let input = vec![Int64(221), Plus];

    let result = parse(&input);
    let expected = ParseError::UnexpectedEOF;

    assert!(result == Err(expected));
}
