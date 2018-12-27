#[macro_use]
extern crate indoc;

mod tokenizer;
mod parser;
mod compiler;

use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
        .expect("標準入力が壊れた");

    let tokens = tokenizer::tokenize(&buffer)
        .expect("字句解析に失敗しました");
    let ast = parser::parse(&tokens)
        .expect("構文解析に失敗しました");
    let assembly =  compiler::compile(ast);

    println!("{}", assembly.value_ref());
}
