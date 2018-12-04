#[derive(Debug, Clone, PartialEq)]
enum Token {
    Plus,
    Minus,
    Int64(i64),
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut chars = input.chars().peekable();
    let mut result = Vec::new();

    while let Some(ch) = chars.next() {
        use Token::*;
        use std::num::ParseIntError;

        match ch {
            ' ' => (),
            '+' => result.push(Plus),
            '-' => result.push(Minus),
            number if number.is_numeric() => {
                let mut value = String::new();
                value.push(ch);

                while let Some(&c) = chars.peek() {
                    if c.is_numeric() {
                        value.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let value = value.parse().map_err(|err| match err {
                    ParseIntError { .. } => "Overflow".to_owned(),
                })?;

                result.push(Int64(value))
            },
            _ => return Err(format!("unknown character: {}", ch)),
        }
    }

    Ok(result)
}

fn main() {
    match tokenize("221 + 212") {
        Ok(ok) =>
            println!("OK: {:?}", ok),
        Err(err) =>
            println!("ERROR: {}", err),
    }
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

    let result = tokenize("$#!");
    assert!(result.is_err());

    /*******************************************/

    let result = tokenize("0 - 12345678901234567890");
    assert!(result.is_err());
}
