use std::env;
use std::fs;

mod build;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let code = fs::read_to_string(&argv[1]).unwrap();
    build::build(code);
}
