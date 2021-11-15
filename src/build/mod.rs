mod tokenize;

pub fn build(argv: Vec<String>) {
    if argv.len() <= 2 {
        println!("please provide a file name");
    } else {
        let tokens = tokenize::tokenize(argv[2].clone());

        for token in tokens {
            println!("{}", token);
        }
    }
}
