mod tokenizer;
mod parser;

fn main() {
    let tokens = tokenizer::tokenize("221+ 212")
        .expect("字句解析に失敗しました");
    let ast = parser::parse(&tokens)
        .expect("構文解析に失敗しました");

    println!("{:?}", ast);
}
