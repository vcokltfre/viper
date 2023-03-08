use std::fs;

mod lexer;
mod parser;
mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let data = fs::read_to_string(filename).expect("Unable to read file.");

    let mut lex = lexer::Lexer::new(filename.to_string(), data.to_string());
    let lexer_result = lex.tokenise();

    if lexer_result.is_err() {
        println!("Error: {}", lexer_result.err().unwrap());
        return;
    }

    let mut parser = parser::Parser::new(
        lexer_result.unwrap(),
        data.lines().map(|s| s.to_string()).collect(),
    );

    let parser_result = parser.parse();

    if parser_result.is_err() {
        println!("Error: {}", parser_result.err().unwrap());
        return;
    }

    // TODO: Parser
    println!("{:?}", parser_result.unwrap());
}
