mod tokenize;

pub fn build(argv: Vec<String>) {
    let mut verbose = false;
    let mut debug = false;
    if argv.contains(&"-v".to_string()) || argv.contains(&"--verbose".to_string()) {
        verbose = true;
    } else if argv.contains(&"-d".to_string()) || argv.contains(&"--debug".to_string()) {
        debug = true;
    }
    if argv.len() <= 2 {
        println!("please provide a file name");
    } else {
        let tokens = tokenize::tokenize(argv[2].clone());

        if debug {
            for token in tokens {
                println!("{}", token);
            }
        }
    }
}
