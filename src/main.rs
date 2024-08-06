use codegen::CodeGen;
use lexer::Literal;

mod ast;
mod codegen;
mod lexer;
mod parser;

fn main() {
    let source = String::from(
        "print 12 + 3;
         print 18 - 1;
         print 22 + 2;"
    );
    let mut lexer = lexer::Lexer::new(source);
}
