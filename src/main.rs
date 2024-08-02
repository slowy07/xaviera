mod lexer;

fn main() {
    let source = "fn main() { print(\"wello xaviera\"); x = 3.14; return;}";
    let mut lexer = lexer::Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens().unwrap();
    println!("Token display: ");
    for token in tokens {
        println!("{:?}", token);
    }
}
