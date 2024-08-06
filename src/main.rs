use codegen::CodeGen;
use lexer::Literal;

mod ast;
mod codegen;
mod lexer;
mod parser;

fn main() {
    let source = String::from(
        "
print(12 + 3);
print(12 * 200);
print(20 * 20);",
    );
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = parser::Parser::new(tokens.clone());
    let node = parser.parse();
    let mut codegen = codegen::CodeGen::new(node.clone());
    let asm = codegen.generate();

    println!("{}", asm);
}
