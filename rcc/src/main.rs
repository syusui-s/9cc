#[macro_use]
extern crate indoc;

mod tokenizer;
mod parser;
mod compiler;

fn main() {
    let tokens = tokenizer::tokenize("221+ 212")
        .expect("字句解析に失敗しました");
    let ast = parser::parse(&tokens)
        .expect("構文解析に失敗しました");
    let assembly =  compiler::compile(ast);

    println!("{:?}", assembly);
}
