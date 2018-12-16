#[macro_use]
extern crate indoc;

mod tokenizer;
mod parser;
mod compiler;

fn main() {
    let tokens = tokenizer::tokenize("1 + 2 + 3 + 4 + 5 + 2 * 3 + 7 + 2 * 4 + 3 * 3 + 10")
        .expect("字句解析に失敗しました");
    let ast = parser::parse(&tokens)
        .expect("構文解析に失敗しました");
    let assembly =  compiler::compile(ast);

    println!("{}", assembly.value_ref());
}
