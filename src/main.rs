use clap::{arg, Command};
mod converter;

fn cli() -> Command {
    Command::new("ttc")
        .about("Common termianl tools commands")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("converter")
                .short_flag('c')
                .about("All kinds of conversion rules are in converter")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("word-case")
                        .short_flag('w')
                        .about("Convert words to various forms of output")
                        .arg_required_else_help(true)
                        .arg(arg!(<WORD>).required(true).num_args(0..=1))
                        .arg(arg!(--outtype <OUTTYPE>).short('o').value_parser([
                            "Lowercase",
                            "Uppercase",
                            "Camelcase",
                            "Constantcase",
                            "Capitalcase",
                            "Dotcase",
                            "Headercase",
                            "Snakecase",
                        ])),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("converter", sub_matches)) => {
            let converter_command = sub_matches.subcommand().unwrap_or(("case", sub_matches));
            match converter_command {
                ("word-case", sub_matches) => {
                    let word = sub_matches
                        .get_one::<String>("WORD")
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    let out_type = sub_matches
                        .get_one::<String>("outtype")
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    converter::case::case(word, out_type.into());
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{name}`")
                }
            }
        }
        _ => println!("The command was not found\n `ttc -h` Print help"),
    }
}
