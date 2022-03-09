#[macro_use]
extern crate lazy_static;
extern crate num_derive;

use clap::{Arg, App};

use std::{env, io};
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let real_args =  &args[1..];
    println!("{:?}", real_args);
    println!("Hello, world!");

    if real_args.len() > 2 {
        println!("Usage: jlox [script]");

        std::process::exit(64);
    } else if real_args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }


}

fn run_file(path: &String)  {

    match File::open(path) {
        // The file is open (no error).
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();

            run(&content);


            // The file is automatically closed when is goes out of scope.
        },
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", path, error);
        },
    }
}

fn run_prompt() -> io::Result<()> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        run(&input);
    }
}

fn run(source: &String) {
    dbg!(source);
}


