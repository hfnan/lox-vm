macro_rules! is_match_eq {
    ($self: expr, $t1: expr, $t2: expr) => {
        {
            let tt = if $self.is_match('=') {$t1} else {$t2};
            $self.make_token(tt)
        }
    };
}

macro_rules! is_alpha {
    () => {
        'a'..='z' | 'A'..='Z' | '_'   
    };
}

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}
pub struct Token {
    pub t: TokenType,
    pub lexme: String,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma, 
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Assign,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Break,
    Eof,
    Error,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {source: source.chars().collect(), start: 0, current: 0, line: 1}
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::SemiColon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => is_match_eq!(self, TokenType::BangEqual, TokenType::Bang),
            '=' => is_match_eq!(self, TokenType::Equal, TokenType::Assign),
            '<' => is_match_eq!(self, TokenType::LessEqual, TokenType::Less),
            '>' => is_match_eq!(self, TokenType::GreaterEqual, TokenType::Greater),
            '"' => self.string(),
            '0'..='9' => self.number(),
            is_alpha!() => self.identifier(),
            _ => self.error_token("Unexpected charactor."),
        }

    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, kind: TokenType) -> Token {
        Token {t: kind, lexme: self.source[self.start..self.current].iter().collect::<String>(), line: self.line}
    }

    fn error_token(&self, message: &str) -> Token {
        Token {t: TokenType::Error, lexme: message.to_string(), line: self.line}
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {false}
        else if self.source[self.current] != expected {false}
        else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\r' | '\t') => {self.advance();},
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                },
                Some('/') => if let Some('/') = self.peek_next() {
                    while self.peek() != Some('\n') && !self.is_at_end() {self.advance();}
                } else {
                    return;
                },
                _ => return,
            }
        }
    }

    fn identifier(&mut self) -> Token {
        while let Some(is_alpha!() | '0'..='9') = self.peek() {
            self.advance();
        }

        self.make_identifier()
    }

    fn identifier_type(&self, lexme: &str) -> TokenType {
        match lexme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "this" => TokenType::This,
            "true" => TokenType::True,
            
            _ => TokenType::Identifier,
        }
    }

    fn make_identifier(&mut self) -> Token{
        let lexme: String = self.source[self.start..self.current].iter().collect();
        Token {t: self.identifier_type(&lexme), lexme, line: self.line}
    }

    fn number(&mut self) -> Token {
        while let Some('0'..='9') = self.peek() {
            self.advance();
        }

        if let (Some('.'), Some('0'..='9')) = (self.peek(), self.peek_next()) {
            self.advance();
            while let Some('0'..='9') = self.peek() {self.advance();}
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != Some('"') && !self.is_at_end() {
            if let Some('\n') = self.peek() {self.line += 1;}
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }
    
    fn peek(&self) -> Option<char> {
        self.source.get(self.current).cloned()
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {None} else {Some(self.source[self.current + 1])}
    }
}
