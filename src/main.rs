use std::collections::HashMap;
use std::collections::vec_deque::Iter;
use std::fmt::Display;
use std::fs;
use std::string::String;

#[derive(Debug,Clone)]
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

struct Tokenizer{
    tokens: Vec<Token>
}

impl Tokenizer{
    fn new() -> Tokenizer{
        Tokenizer{
            tokens: Vec::new()
        }
    }

    fn tokenize(self:&mut Self, string: String){
        self.tokens.clear();
        let mut tokens  = Vec::new();

        let chars: Vec<char> = string.chars().collect();

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
            match token {
                Token::Empty => {
                //     eat 5star do nothing
                }
                token => {
                    tokens.push(token);
                }
            }
            i += 1;
        }

        self.tokens = tokens;
    }

    pub fn display_tokens(&self) {
        for token in &self.tokens {
            print!("{}", token);
        }
    }
}

#[derive(Debug)]
enum JsonValue{
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            JsonValue::Null => {
                print!("null")
            }
            JsonValue::Bool(val) => {
                print!("{}", val)
            }
            JsonValue::Number(num) => {
                print!("{}", num)
            }
            JsonValue::String(string) => {
                print!("\"{}\"", string)
            }
            JsonValue::Array(arr) => {
                print!("[");
                for (i, value) in arr.iter().enumerate() {
                    if i != 0 {
                        print!(", ");
                    }
                    print!("{}", value);
                }
                print!("]");
            }
            JsonValue::Object(obj) => {
                print!("{{");
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i != 0 {
                        print!(", ");
                    }
                    print!("\"{}\": {}", key, value);
                }
                print!("}}");
            }
        };
        Ok(())
    }
}


struct Parser{
    tokenizer:Tokenizer,
}

impl Parser {
    fn new(string: String) -> Parser{
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(string);
        Parser{
            tokenizer
        }
    }

    fn display_tokens(&self){
        self.tokenizer.display_tokens();
    }

    pub fn parse(&self) -> JsonValue {
        let mut iter = self.tokenizer.tokens.iter().peekable();
        self.parse_value(&mut iter)
    }

    fn parse_value(&self, mut iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> JsonValue{
        match iter.peek() {
            None => {
                JsonValue::Null
            }
            Some(token) => {
                match token {
                    Token::LeftCurlyBracket => {
                        self.parse_object(&mut iter)
                    }
                    Token::LeftSquareBracket => {
                        self.parse_array(&mut iter)
                    }
                    Token::String(string) => {
                        iter.next();
                        JsonValue::String(string.clone())
                    }
                    Token::Number(num) => {
                        iter.next();
                        JsonValue::Number(*num)
                    }
                    Token::True => {
                        iter.next();
                        JsonValue::Bool(true)
                    }
                    Token::False => {
                        iter.next();
                        JsonValue::Bool(false)
                    }
                    _ => {
                        iter.next();
                        JsonValue::Null
                    }
                }
            }
        }
    }

    fn parse_array(&self, iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> JsonValue{

        let mut array = Vec::new();
        iter.next(); // consume left square bracket

        loop {
            match iter.peek() {
                None => {
                    break;
                }
                Some(token) => {
                    match token {
                        Token::RightSquareBracket => {
                            iter.next();
                            break;
                        }
                        Token::Comma => {
                            iter.next();
                        }
                        _ => {
                            array.push(self.parse_value(iter));
                        }
                    };
                }
            }
        }
        
        JsonValue::Array(array)
    }

    fn parse_object(&self, iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> JsonValue {
        let mut object = HashMap::new();
        loop {
            match iter.peek() {
                None => {
                    break;
                }
                Some(token) => {
                    match token {
                        Token::RightCurlyBracket => {
                            iter.next();
                            break;
                        }
                        Token::String(key) => {
                            iter.next();
                            match iter.peek() {
                                Some(Token::Colon) => {
                                    iter.next();
                                    object.insert(key.clone(), self.parse_value(iter));
                                }
                                _ => {
                                    iter.next();
                                }
                            }
                        }
                        _ => {
                            iter.next();
                        }
                    }
                }
            }
        }
        JsonValue::Object(object)
    }

}


fn main() {
    let json = fs::read_to_string("./test.json").unwrap();

    let parser = Parser::new(json);
    // parser.display_tokens();

    let json_value:JsonValue = parser.parse();

    print!("\n{:}",json_value)
}
