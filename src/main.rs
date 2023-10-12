use clap::{arg, Command};
use std::process::{Command as StdCommand, Stdio};
use text_colorizer::*;
mod converter;
mod datetime;

fn cli() -> Command {
    Command::new("ttc")
        .about("Common termianl tools commands")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            arg!(--local <LOCAL> "Execute the local terminal command")
                .short('l')
                .required(false)
                .num_args(1),
        )
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
    let args: Vec<String> = std::env::args().collect();
    let exec_completed = exec_local(&args);
    if exec_completed {
        return;
    }
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
        _ => print_not_found_info(args[1..].join(" ").as_str()),
    }
}

fn print_not_found_info(arg: &str) {
    let print_info = format!("ttc: command not found: {}", arg.blue());
    println!("{}", print_info.red());
}

fn exec_local(args: &Vec<String>) -> bool {
    let mut local_args = vec![];
    let mut local_flag: bool = false;
    for (i, a) in args.iter().enumerate() {
        if i == 1 && (a == "-l" || a == "--local") {
            local_flag = true;
            continue;
        }
        if local_flag {
            local_args.push(a.as_str());
            continue;
        }
    }
    if local_flag {
        let output_result = StdCommand::new("sh")
            .arg("-c")
            .arg(local_args.join(" "))
            .stdout(Stdio::inherit())
            .output();
        match output_result {
            Ok(out) if out.status.success() => {}
            _ => print_not_found_info(local_args.join(" ").as_str()),
        }
    }
    local_flag
}
