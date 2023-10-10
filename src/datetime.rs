use chrono::{DateTime, Local, LocalResult, NaiveDateTime};
use std::{fmt::Display, vec};
use text_colorizer::*;

pub fn exec(arg: Arg) {
    let mut args: Vec<Arg> = vec![];
    if arg == Arg::Unknown {
        args = Arg::get_support_arg();
    } else {
        match arg {
            Arg::Datetime(datetime) => {
                let d = NaiveDateTime::parse_from_str(datetime.as_str(), "%Y-%m-%d %H:%M:%S")
                    .unwrap_or(NaiveDateTime::default())
                    .and_local_timezone(Local);
                match d {
                    LocalResult::Single(d) => {
                        args.push(Arg::Datetime(d.format("%Y-%m-%d %H:%M:%S").to_string()));
                        args.push(Arg::Timestamp(d.timestamp()));
                    }
                    _ => {}
                }
            }
            Arg::Timestamp(timestamp) => {
                let d1 = DateTime::from_timestamp(timestamp, 0)
                    .unwrap_or(DateTime::default())
                    .with_timezone(&Local);
                args.push(Arg::Datetime(d1.format("%Y-%m-%d %H:%M:%S").to_string()));
                args.push(Arg::Timestamp(d1.timestamp()));
            }
            _ => {}
        }
    }
    for a in args {
        let res = a.exec();
        println!("{}", res)
    }
}

struct Result {
    arg: String,
    out: String,
}

impl Display for Result {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}: {}", self.arg.bold().black(), self.out);
        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Arg {
    Unknown,
    Datetime(String),
    Timestamp(i64),
}

impl ToString for Arg {
    fn to_string(&self) -> String {
        match self {
            Arg::Datetime(_) => "Datetime".to_string(),
            Arg::Timestamp(_) => "Timestamp".to_string(),
            _ => "UNknown".to_string(),
        }
    }
}
impl Arg {
    fn get_support_arg() -> Vec<Arg> {
        let now = Local::now();
        let date_time = now.format("%Y-%m-%d %H:%M:%S");
        let timestamp = now.timestamp();
        vec![
            Arg::Datetime(date_time.to_string()),
            Arg::Timestamp(timestamp),
        ]
    }
    fn exec(self) -> Result {
        let mut out = String::new();
        let mut res = Result {
            arg: self.to_string(),
            out: "".to_string(),
        };
        match self {
            Arg::Datetime(datetime) => out = datetime,
            Arg::Timestamp(timestamp) => out = format!("{}", timestamp),
            _ => {}
        }
        res.out = out;
        res
    }
}
