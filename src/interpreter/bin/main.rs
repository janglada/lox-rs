use clap::{arg, command, Command};

fn main() {
    let _matches = command!()
        .arg(
            arg!([file] " name to operate on")
                .required(true)

        )
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
                // We don't have syntax yet for optional options, so manually calling `required`
                .required(false)
                // Support non-UTF8 paths
                .allow_invalid_utf8(true),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values")),
        )
        .get_matches();
}