// WORKING add help
// TODO basic parser
// TODO -

#![allow(non_snake_case)]

mod doc;
mod build;

use std::env;

fn help(len: usize, subc: Vec<String>) {
    println!();
    if len <= 1 {
        println!("Please provide an argument");
    } else if subc[1] != "help".to_string() {
        println!("{} isnt a valid subcommand", subc[1]);
    }

    println!("bluestone v0.1.0-indev");
    println!("--- subcommands ---");
    println!("build - builds your bluestone file. (eg `bluestone build main.blstn`)");
    println!("doc - build markdown documentation for your bluestone file. (eg `bluestone doc main.blstn`)");
    println!("version - prints version info");
    println!("--- flags ---");
    println!("-v or --verbose - prints extra debug info");
    println!("-d or --debug - print a lot of debug info (not reccomended to use)");
    println!("\nthanks for using bluestone :D\n");

    let mut verbose = false;
    let mut debug = false;
    if subc.contains(&"-v".to_string()) || subc.contains(&"--verbose".to_string()) {
        verbose = true;
    } else if subc.contains(&"-d".to_string()) || subc.contains(&"--debug".to_string()) {
        debug = true;
    }

    if verbose {
        println!("we are verbose");
    }

    if debug {
        println!("we are debug");
    }

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