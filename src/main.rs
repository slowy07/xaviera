mod lexer;

fn main() {
    let source = "fn main() { print(\"wello xaviera\"); return;}";
    let mut lexer = lexer::Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens().unwrap();
    for token in tokens {
        println!("{:?}", token);
    }
}
