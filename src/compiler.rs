use enum_iterator::Sequence;

use crate::{scanner::*, vm::*, chunk::*, value::Value};

#[derive(Default)]
pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
}

#[derive(PartialEq, PartialOrd, Sequence, Clone, Copy)]
pub enum Prec {
    None,
    Assignment,
    Or, 
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

pub struct ParseRule<'a> {
    prefix: Option<fn(&mut Compiler<'a>)>,
    infix: Option<fn(&mut Compiler<'a>)>,
    precedence: Prec,
}

impl<'a> ParseRule<'a> {
    pub fn new(prefix: Option<fn(&mut Compiler<'a>)>, infix: Option<fn(&mut Compiler<'a>)>, precedence: Prec) -> Self {
        Self {prefix, infix, precedence}
    }
}

pub struct Compiler<'a> {
    parser: Parser,
    scanner: Scanner,
    chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &str, chunk: &'a mut Chunk) -> Self {
        Self {
            parser: Parser::default(),
            scanner: Scanner::new(source), 
            chunk,
        }
    }

    pub fn compile(&mut self) -> InterpretResult<()> {
        self.parser.had_error = false;

        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");
        
        self.end_compiler();

        if self.parser.had_error {
            Err(InterpretError::CompilerError)
        } else {
            Ok(())
        }
    }    
    
    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.t != TokenType::Error {break;}

            
            self.error_at_current(&self.parser.current.lexme.clone());
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.parser.current, message);
        self.parser.had_error = true;
        self.parser.panic_mode = true;
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.parser.previous, message);
        self.parser.had_error = true;
        self.parser.panic_mode = true;
    }

    fn error_at(&self, token: &Token, message: &str) {
        if self.parser.panic_mode {return ;}

        eprint!("[line {}] Error", token.line);

        if token.t == TokenType::Eof {
            eprint!(" at end");
        } else if token.t == TokenType::Error {
            // ignore
        } else {
            eprint!(" at '{}'", token.lexme);
        }

        eprintln!(":{message}");
    }

    fn consume(&mut self, t: TokenType, message: &str) {
        if self.parser.current.t == t {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.parser.previous.line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk
    }

    fn end_compiler(&mut self) {
        self.emit_return();

        #[cfg(feature = "debug_print_code")] {
            if !self.parser.had_error {
                self.current_chunk().disassamble("code");
            }
        }
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.t;
        let rule = get_rule(operator_type);
        self.parse_precedence(rule.precedence.next().unwrap());

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            _ => return,
        }
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.t;

        self.parse_precedence(Prec::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
            _ => return,
        }
    }

    fn parse_precedence(&mut self, precedence: Prec) {
        self.advance();
        let prefix_rule = get_rule(self.parser.previous.t).prefix;
        match prefix_rule {
            Some(prefix_rule) => {
                prefix_rule(self);
                while precedence <= get_rule(self.parser.current.t).precedence {
                    self.advance();
                    if let Some(infix_rule) = get_rule(self.parser.previous.t).infix {
                        infix_rule(self);
                    }
                }
            }
            _ => self.error("Exprect expression."),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value: Value = self.parser.previous.lexme.parse().unwrap();
        self.emit_constant(value);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_constant(value);
        if constant > u8::MAX as usize {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant as u8
    }    

    fn expression(&mut self) {
        self.parse_precedence(Prec::Assignment);
    }

}

fn get_rule<'a>(t: TokenType) -> ParseRule<'a> {
        match t {
            TokenType::LeftParen    => ParseRule::new(Some(Compiler::grouping), None, Prec::None),
            TokenType::RightParen   => ParseRule::new(None, None, Prec::None),
            TokenType::LeftBrace    => ParseRule::new(None, None, Prec::None),
            TokenType::RightBrace   => ParseRule::new(None, None, Prec::None),
            TokenType::Comma        => ParseRule::new(None, None, Prec::None), 
            TokenType::Dot          => ParseRule::new(None, None, Prec::None),
            TokenType::Minus        => ParseRule::new(Some(Compiler::unary), Some(Compiler::binary), Prec::Term),
            TokenType::Plus         => ParseRule::new(None, Some(Compiler::binary), Prec::Term),
            TokenType::SemiColon    => ParseRule::new(None, None, Prec::None), 
            TokenType::Slash        => ParseRule::new(None, Some(Compiler::binary), Prec::Factor),
            TokenType::Star         => ParseRule::new(None, Some(Compiler::binary), Prec::Factor),
            TokenType::Bang         => ParseRule::new(None, None, Prec::None),
            TokenType::BangEqual    => ParseRule::new(None, None, Prec::None),
            TokenType::Assign       => ParseRule::new(None, None, Prec::None),
            TokenType::Equal        => ParseRule::new(None, None, Prec::None),
            TokenType::Greater      => ParseRule::new(None, None, Prec::None),
            TokenType::GreaterEqual => ParseRule::new(None, None, Prec::None),
            TokenType::Less         => ParseRule::new(None, None, Prec::None),
            TokenType::LessEqual    => ParseRule::new(None, None, Prec::None),
            TokenType::Identifier   => ParseRule::new(None, None, Prec::None),
            TokenType::String       => ParseRule::new(None, None, Prec::None),
            TokenType::Number       => ParseRule::new(Some(Compiler::number), None, Prec::None),
            TokenType::And          => ParseRule::new(None, None, Prec::None),
            TokenType::Class        => ParseRule::new(None, None, Prec::None),
            TokenType::Else         => ParseRule::new(None, None, Prec::None),
            TokenType::False        => ParseRule::new(None, None, Prec::None),
            TokenType::Fun          => ParseRule::new(None, None, Prec::None),
            TokenType::For          => ParseRule::new(None, None, Prec::None),
            TokenType::If           => ParseRule::new(None, None, Prec::None),
            TokenType::Nil          => ParseRule::new(None, None, Prec::None),
            TokenType::Or           => ParseRule::new(None, None, Prec::None),
            TokenType::Print        => ParseRule::new(None, None, Prec::None),
            TokenType::Return       => ParseRule::new(None, None, Prec::None),
            TokenType::Super        => ParseRule::new(None, None, Prec::None),
            TokenType::This         => ParseRule::new(None, None, Prec::None),
            TokenType::True         => ParseRule::new(None, None, Prec::None),
            TokenType::Var          => ParseRule::new(None, None, Prec::None),
            TokenType::While        => ParseRule::new(None, None, Prec::None),
            TokenType::Eof          => ParseRule::new(None, None, Prec::None),
            TokenType::Error        => ParseRule::new(None, None, Prec::None),
        }
    }

