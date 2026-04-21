use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::string::String;

#[derive(Debug)]
enum Token{
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Colon,
    String(String),
    Number(f64),
    Null,
    True,
    False,
    Empty
}

impl Display for Token{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
        match self {
            Token::LeftCurlyBracket => {
                print!("{}", '{')
            }
            Token::RightCurlyBracket => {
                print!("{}", '}')
            }
            Token::LeftSquareBracket => {
                print!("{}", '[')
            }
            Token::RightSquareBracket => {
                print!("{}", ']')
            }
            Token::Comma => {
                print!("{}", ',')
            }
            Token::Colon => {
                print!("{}", ':')
            }
            Token::String(string) => {
                print!("\"{}\"", string)
            }
            Token::Number(number) => {
                print!("{}", number)
            }
            Token::Null => {
                print!("null")
            }
            Token::True => {
                print!("{}", "true")
            }
            Token::False => {
                print!("{}", "false")
            }
            Token::Empty => {}
        }
        Ok(())
    }
}

enum JsonValue{
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}


fn main() {
    let json = fs::read_to_string("./test.json").unwrap();

    let mut tokens = Vec::new();

    let chars: Vec<char> = json.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let char = chars[i];
        let token = match char {
            '{' => Token::LeftCurlyBracket,
            '}' => Token::RightCurlyBracket,
            ',' => Token::Comma,
            ':' => Token::Colon,
            '[' => Token::LeftSquareBracket,
            ']' => Token::RightSquareBracket,
            '"' => {
                let mut string = String::new();
                i+=1;
                while i < chars.len() && chars[i] != '"' {
                    string.push(chars[i]);
                    i+=1;
                }
                Token::String(string)
            }
            't' => {
                let mut true_string = String::new();
                let mut j = i;
                while (j-i) < 4 {
                    true_string.push(chars[j]);
                    j+=1;
                }
                let token = if true_string == "true" {
                    i = j;
                    Token::True
                }else {
                    Token::Empty
                };
                i-=1;
                token
            }
            'f' => {
                let mut false_string = String::new();
                let mut j = i;
                while (j-i) < 5 {
                    false_string.push(chars[j]);
                    j+=1;
                }
                let token = if false_string == "false" {
                    i = j;
                    Token::False
                }else {
                    Token::Empty
                };
                i-=1;
                token
            }
            'n' => {
                let mut null_string = String::new();
                let mut j = i;
                while (j-i) < 4 {
                    null_string.push(chars[j]);
                    j+=1;
                }
                let token = if null_string == "null" {
                    i = j;
                    Token::Null
                }else {
                    Token::Empty
                };
                i-=1;
                token
            }
            char => {
                let token = if char.is_numeric() {
                    let mut string = String::new();
                    while i < chars.len() && (chars[i].is_numeric() || chars[i] == '.') {
                        string.push(chars[i]);
                        i+=1;
                    }
                    i-=1;
                    Token::Number(string.parse::<f64>().unwrap())
                } else {
                    Token::Empty
                };
                token
            }
        };

        tokens.push(token);
        i += 1;
    }

    let tokens = tokens.iter().filter(|token: &&Token| if let Token::Empty = token { false } else { true } ).collect::<Vec<&Token>>();

    for token in &tokens {
        print!("{}", token);
    }
}
