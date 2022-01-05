mod tokenizer;

pub fn build(code: String) {
    let tokens = tokenizer::Lexer::new(code.as_str());

    for token in tokens {
        println!("Token: {:?}", token);
    }
}