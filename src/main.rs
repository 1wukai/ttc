use clap::{arg, Command};
mod converter;
mod datetime;

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
                        .arg(arg!(<WORD>).required(true).num_args(1..=1))
                        .arg(
                            arg!(--outtype <OUTTYPE>)
                                .num_args(1..=1)
                                .short('o')
                                .value_parser([
                                    "Lowercase",
                                    "Uppercase",
                                    "Camelcase",
                                    "Constantcase",
                                    "Capitalcase",
                                    "Dotcase",
                                    "Headercase",
                                    "Snakecase",
                                ]),
                        ),
                )
                .subcommand(
                    Command::new("qr")
                        .short_flag('q')
                        .about("Convert text to QR code")
                        .arg_required_else_help(true)
                        .arg(arg!(<TEXT>).required(true).num_args(1..=1)),
                ),
        )
        .subcommand(
            Command::new("datetime")
                .short_flag('d')
                .about("Output various time displays")
                .arg(
                    arg!(--datetime <DATETIME> "Appointed datetime")
                        .short('d')
                        .num_args(1..=1),
                )
                .arg(
                    arg!(--timestamp <TIMESTAMP> "Appointed timestamp")
                        .short('t')
                        .num_args(1..=1),
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
                ("qr", sub_matches) => {
                    let text = sub_matches
                        .get_one::<String>("TEXT")
                        .map(|s| s.as_str())
                        .unwrap_or("null");
                    converter::qr::exec(text);
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{name}`")
                }
            }
        }
        Some(("datetime", sub_matches)) => {
            let datetime = sub_matches
                .get_one::<String>("datetime")
                .map(|s| s.as_str())
                .unwrap_or("");
            if !datetime.is_empty() {
                datetime::exec(datetime::Arg::Datetime(datetime.to_string()));
                return;
            }
            let timestamp = sub_matches
                .get_one::<String>("timestamp")
                .map(|s| s.parse::<i64>().unwrap_or(0))
                .unwrap_or(0);
            if timestamp > 0 {
                datetime::exec(datetime::Arg::Timestamp(timestamp));
                return;
            }

            datetime::exec(datetime::Arg::Unknown);
        }
        _ => println!("The command was not found\n `ttc -h` Print help"),
    }
}
