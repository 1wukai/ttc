use std::fmt::Display;
use text_colorizer::*;

/// case
/// 将传入的单词转化为不同的拼接方式进行返回
/// Example
///
pub fn case(word: &str, out_type: Type) {
    match out_type {
        Type::UNKnown => {
            for i in Type::get_support_type() {
                println!("{}", i.convert(word))
            }
        }
        _ => println!("{}", out_type.convert(word)),
    }
}

pub struct Result {
    case: Type,
    out: String,
}

impl Display for Result {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}: {}", self.case.to_string().bold().black(), self.out);
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Type {
    UNKnown,
    Lowercase,
    Uppercase,
    Camelcase,
    Constantcase,
    Capitalcase,
    Dotcase,
    Headercase,
    Nocase,
    Paramcase,
    Pascalcase,
    Pathcase,
    Sentencecase,
    Snakecase,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Lowercase => "Lowercase".to_string(),
            Type::Uppercase => "Uppercase".to_string(),
            Type::Camelcase => "Camelcase".to_string(),
            Type::Constantcase => "Constantcase".to_string(),
            Type::Capitalcase => "Capitalcase".to_string(),
            Type::Dotcase => "Dotcase".to_string(),
            Type::Headercase => "Headercase".to_string(),
            Type::Snakecase => "Snakecase".to_string(),
            Type::Nocase => "Nocase".to_string(),
            Type::Paramcase => "Paramcase".to_string(),
            Type::Pascalcase => "Pascalcase".to_string(),
            Type::Pathcase => "Pathcase".to_string(),
            Type::Sentencecase => "Sentencecase".to_string(),
            _ => "UNknown".to_string(),
        }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "Lowercase" => Type::Lowercase,
            "Uppercase" => Type::Uppercase,
            "Camelcase" => Type::Camelcase,
            "Constantcase" => Type::Constantcase,
            "Capitalcase" => Type::Capitalcase,
            "Dotcase" => Type::Dotcase,
            "Headercase" => Type::Headercase,
            "Snakecase" => Type::Snakecase,
            "Nocase" => Type::Nocase,
            "Paramcase" => Type::Paramcase,
            "Pascalcase" => Type::Pascalcase,
            "Pathcas" => Type::Pathcase,
            "Sentencecase" => Type::Sentencecase,
            _ => Type::UNKnown,
        }
    }
}

impl Type {
    pub fn get_support_type() -> Vec<Type> {
        vec![
            Type::Lowercase,
            Type::Uppercase,
            Type::Camelcase,
            Type::Constantcase,
            Type::Capitalcase,
            Type::Dotcase,
            Type::Headercase,
            Type::Snakecase,
            // Type::Nocase,
            // Type::Paramcase,
            // Type::Pascalcase,
            // Type::Pathcase,
            // Type::Sentencecase,
        ]
    }
    fn convert(self, word: &str) -> Result {
        let mut res = String::new();
        let mut front_is_alphabetic = true;
        match self {
            Type::Lowercase => {
                for (i, c) in word.chars().enumerate() {
                    match c.is_alphabetic() {
                        true => {
                            res.push(c.to_lowercase().next().unwrap_or('?'));
                            front_is_alphabetic = true
                        }
                        false if i != word.chars().count() - 1 && front_is_alphabetic => {
                            res.push(' ');
                            front_is_alphabetic = false
                        }
                        _ => {}
                    }
                }
            }
            Type::Uppercase => {
                for (i, c) in word.chars().enumerate() {
                    match c.is_alphabetic() {
                        true => {
                            res.push(c.to_uppercase().next().unwrap_or('?'));
                            front_is_alphabetic = true
                        }
                        false if i != word.chars().count() - 1 && front_is_alphabetic => {
                            res.push(' ');
                            front_is_alphabetic = false
                        }
                        _ => {}
                    }
                }
            }
            Type::Camelcase => {
                let mut camle_flag = false;
                for c in word.chars() {
                    match c.is_alphabetic() {
                        true => {
                            if camle_flag {
                                res.push(c.to_uppercase().next().unwrap_or('?'));
                                camle_flag = false
                            } else {
                                res.push(c.to_lowercase().next().unwrap_or('?'))
                            }
                        }
                        _ => camle_flag = true,
                    }
                }
            }
            Type::Constantcase => {
                for (i, c) in word.chars().enumerate() {
                    match c.is_alphabetic() {
                        true => {
                            res.push(c.to_uppercase().next().unwrap_or('?'));
                            front_is_alphabetic = true
                        }
                        false if i != word.chars().count() - 1 && front_is_alphabetic => {
                            res.push('_');
                            front_is_alphabetic = false
                        }
                        _ => {}
                    }
                }
            }
            Type::Capitalcase => {
                let mut camle_flag = true;
                for (i, c) in word.chars().enumerate() {
                    match c.is_alphabetic() {
                        true => {
                            if camle_flag {
                                res.push(c.to_uppercase().next().unwrap_or('?'));
                                camle_flag = false
                            } else {
                                res.push(c.to_lowercase().next().unwrap_or('?'))
                            }
                            front_is_alphabetic = true
                        }
                        false if i != word.chars().count() - 1 && front_is_alphabetic => {
                            res.push(' ');
                            camle_flag = true;
                            front_is_alphabetic = false
                        }
                        _ => {}
                    }
                }
            }
            Type::Dotcase => {
                for c in word.chars() {
                    match c.is_alphabetic() {
                        true => {
                            res.push(c.to_lowercase().next().unwrap_or('?'));
                            front_is_alphabetic = true
                        }
                        false if front_is_alphabetic => {
                            res.push('.');
                            front_is_alphabetic = false
                        }
                        _ => {}
                    }
                }
            }
            Type::Headercase => {
                let mut header_flag = true;
                for (i, c) in word.chars().enumerate() {
                    match c.is_alphabetic() {
                        true => {
                            if header_flag {
                                res.push(c.to_uppercase().next().unwrap_or('?'));
                                header_flag = false
                            } else {
                                res.push(c.to_lowercase().next().unwrap_or('?'));
                            }
                            front_is_alphabetic = true
                        }
                        false if i != word.chars().count() - 1 && front_is_alphabetic => {
                            res.push('-');
                            header_flag = true;
                            front_is_alphabetic = false
                        }
                        _ => {}
                    }
                }
            }
            Type::Snakecase => {
                for c in word.chars() {
                    match c.is_alphabetic() {
                        true => {
                            res.push(c.to_lowercase().next().unwrap_or('?'));
                            front_is_alphabetic = true
                        }
                        false if front_is_alphabetic => {
                            res.push('_');
                            front_is_alphabetic = false
                        }
                        _ => {}
                    }
                }
            }
            _ => res = word.to_string(),
        };
        Result {
            case: self,
            out: res,
        }
    }
}
