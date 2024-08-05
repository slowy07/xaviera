use codegen::CodeGen;
use lexer::Literal;

mod ast;
mod codegen;
mod lexer;
mod parser;

fn main() {
    let source = String::from("print 2 + 4 * 3 / 2;");
    let mut lexer = lexer::Lexer::new(source);
}
