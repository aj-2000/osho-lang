use std::fs;
use std::str::Chars;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Token Type
    pub kind: Kind,

    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,

    pub value: TokenValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Number(f64),
    String(Atom),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof, // end of file
    WhiteSpace,
    Plus,
    Increment, // for '++'
    Decrement, // for '--'
    Minus,
    EqualsTo,
    Identifier,
    Number,
    String,
    Print,
    OpenParen,
    CloseParen,
    Const,
}

struct Lexer<'a> {
    /// Source Text
    source: &'a str,

    /// The remaining characters
    chars: Chars<'a>,

    /// Current position in the source
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer instance
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
            current_pos: 0,
        }
    }

    /// Get all tokens from the source
    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.read_next_token();
            if token.kind == Kind::Eof {
                tokens.push(token);
                break;
            }
            // Skip whitespace tokens
            if token.kind != Kind::WhiteSpace {
                tokens.push(token);
            }
        }
        tokens
    }

    /// Read the next token
    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        let value = self.extract_value(&kind, start, end);
        Token {
            kind,
            start,
            end,
            value,
        }
    }

    /// Read the next kind of token
    fn read_next_kind(&mut self) -> Kind {
        while let Some(c) = self.next_char() {
            match c {
                '+' => return self.handle_plus(),
                '-' => return self.handle_minus(),
                '=' => return Kind::EqualsTo,
                '(' => return Kind::OpenParen,
                ')' => return Kind::CloseParen,
                '"' => return self.read_string(),
                _ if c.is_numeric() => return self.read_number(c),
                _ if c.is_alphabetic() => return self.read_identifier_or_keyword(c),
                _ if c.is_whitespace() => return Kind::WhiteSpace,
                _ => {
                    eprintln!("Unrecognized char: {}", c);
                    continue;
                }
            }
        }
        Kind::Eof
    }

    /// Handle the '+' character and check for '++'
    fn handle_plus(&mut self) -> Kind {
        if let Some(next_char) = self.peek() {
            if next_char == '+' {
                self.next_char(); // Consume the second '+'
                return Kind::Increment;
            }
        }
        Kind::Plus
    }

     /// Handle the '+' character and check for '++'
    fn handle_minus(&mut self) -> Kind {
        if let Some(next_char) = self.peek() {
            if next_char == '-' {
                self.next_char(); // Consume the second '+'
                return Kind::Decrement;
            }
        }
        Kind::Minus
    }

    /// Read a number token
    fn read_number(&mut self, initial: char) -> Kind {
        let mut num_str = initial.to_string();
        while let Some(c) = self.peek() {
            if c.is_digit(10) || c == '.' {
                num_str.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        Kind::Number
    }

    /// Read a string token
    fn read_string(&mut self) -> Kind {
        let mut str_content = String::new();
        while let Some(c) = self.next_char() {
            if c == '"' {
                break;
            } else {
                str_content.push(c);
            }
        }
        Kind::String
    }

    /// Read an identifier or keyword
    fn read_identifier_or_keyword(&mut self, initial: char) -> Kind {
        let mut ident = initial.to_string();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        self.match_keyword(&ident)
    }

    /// Match an identifier to a keyword
    fn match_keyword(&self, ident: &str) -> Kind {
        match ident {
            "print" => Kind::Print,
            "const" => Kind::Const,
            _ => Kind::Identifier,
        }
    }

    /// Extract the value of a token based on its kind
    fn extract_value(&self, kind: &Kind, start: usize, end: usize) -> TokenValue {
        match kind {
            Kind::Number => {
                let num_str = &self.source[start..end];
                if let Ok(num) = num_str.parse::<f64>() {
                    TokenValue::Number(num)
                } else {
                    eprintln!("Invalid number: {}", num_str);
                    TokenValue::None
                }
            }
            Kind::Identifier => {
                let str_content = &self.source[start..end];
                TokenValue::String(Atom::from(str_content))
            }
            Kind::String => {
                let str_content = &self.source[start + 1..end - 1]; // exclude quotes
                TokenValue::String(Atom::from(str_content))
            }
            _ => TokenValue::None,
        }
    }

    /// Get the current offset in the source text
    fn offset(&self) -> usize {
        self.current_pos
    }

    /// Peek at the next character without consuming it
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Consume and return the next character
    fn next_char(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(c) = next {
            self.current_pos += c.len_utf8();
        }
        next
    }
}

fn main() {
    let file_path = "./examples/add-two-numbers.osho";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let tokens = Lexer::new(&contents).get_tokens();

    println!("{:#?}", tokens);
}
