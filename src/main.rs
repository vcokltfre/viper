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
    let result = lex.tokenise();

    if result.is_err() {
        println!("Error: {}", result.err().unwrap());
        return;
    }

    let mut parser = parser::Parser::new(result.unwrap());

    // TODO: Parser
    println!("{:?}", parser.parse());
}
