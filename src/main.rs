// WORKING add help
// TODO basic parser
// TODO add builtin trait. ex: #[builtin(name="print")]
// TODO create actual documentation in form of markdown


// I'll use whatever fucking naming convention I want.
#![allow(non_snake_case)]

mod doc;
mod build;

use std::env;

fn help(len: usize, subc: Vec<String>) {
    if len <= 1 {
        println!("Please provide an argument");
    } else {
        println!("{} isnt a valid subcommand", subc[1]);
    }

    // TODO actually add an help "menu"
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let acceptedArguments = ["build", "b", "doc", "d"];

    if argv.len() != 1 && acceptedArguments.contains(&argv[1].as_str()) {
        match argv[1].as_str() {
            "build" | "b" => build::build(argv),
            "doc" | "d" => doc::doc(),
            _ => help(argv.len(), argv)
        }
    } else {
        help(argv.len(), argv);
    }
}