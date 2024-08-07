use codegen::CodeGen;
use lexer::Literal;

mod ast;
mod codegen;
mod lexer;
mod parser;

fn main() {
    let source = String::from(
        "
let rauf;
let asad;
asad = 20;
rauf = 120;
print(asad + rauf);
",
    );
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = parser::Parser::new(tokens.clone());
    let node = parser.parse();
    let mut codegen = codegen::CodeGen::new(node.clone());
    let asm = codegen.generate();

    println!("{}", asm);
}
