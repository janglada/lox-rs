use clap::{arg, command};



use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let matches = command!()
        .arg(arg!([file] " the file to compile").required(true))
        .arg(
            arg!(
                -o --output <FILENAME> "Write output to <filename>"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .get_matches();

    let mut out_writer = match matches.value_of("output") {
        Some(x) => {
            println!("{:?}", x);
            let path = Path::new(x);
            Box::new(File::create(&path).unwrap()) as Box<dyn Write>
        }
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    if let Some(name) = matches.value_of("file") {
        compile_file(&name.to_string(), &mut out_writer)
    }
}

fn compile_file(path: &String, _write: &mut Box<dyn Write>) {
    match File::open(path) {
        // The file is open (no error).
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();

            // let mut chunk = Chunk::new();
            // let mut compiler = Compiler::new(content.as_str(), &mut chunk);
            // if !compiler.compile() {
            //     //return InterpretResult::CompileError;
            // } else {
            //     chunk.to_bytes()
            //     chunk.disassemble_chunk(write);
            //     file.flush();
            //
            // }

            // The file is automatically closed when is goes out of scope.
        }
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", path, error);
        }
    }
}
