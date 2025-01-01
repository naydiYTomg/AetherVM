use std::collections::HashMap;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::token::TokenType::{ADD, ADDRESS, AT, CALL, COLON, COMMA, DIV, DOLLAR, DOUBLEKW, EOF, EQ, EXIT, FLOATING, FLOATKW, FUNCDEF, HALT, I16KW, I32KW, I64KW, I8KW, IDENT, INTEGER, JMC, JMP, LBRACE, LPAREN, MINUS, MOVE, MUL, PARAMS, PERCENT, PERIOD, PLUS, RBRACE, REM, RET, RPAREN, SET, SLASH, STAR, SUB, UNDER, UNSIGNEDKW, VARDEF};
use crate::utils::stringutils::StringBuilder;

pub struct Lexer {
    pos: usize,
    input: String,
    cur: char,
    size: usize,
    output: Vec<Token>,
    buffer: StringBuilder
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let pos: usize = 0;
        let cur = (&input).chars().nth(pos).expect("Empty input");
        let size = (&input).len();
        Self {
            pos,
            input,
            cur,
            size,
            output: Vec::new(),
            buffer: StringBuilder::new()
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        while self.has_next() {
            match self.cur {
                '$' =>  self.tokenize_keyword(),
                '.' => { self.output.push(Token::new(PERIOD, String::from("."), self.pos)); self.next() }
                ',' => { self.output.push(Token::new(COMMA, String::from(","), self.pos)); self.next() }
                '_' => { self.output.push(Token::new(UNDER, String::from("_"), self.pos)); self.next() }
                ':' => { self.output.push(Token::new(COLON, String::from(":"), self.pos)); self.next() }
                '(' => { self.output.push(Token::new(LPAREN, String::from("("), self.pos)); self.next() }
                ')' => { self.output.push(Token::new(RPAREN, String::from(")"), self.pos)); self.next() }
                '{' => { self.output.push(Token::new(LBRACE, String::from("{"), self.pos)); self.next() }
                '}' => { self.output.push(Token::new(RBRACE, String::from("}"), self.pos)); self.next() }
                '=' => { self.output.push(Token::new(EQ, String::from("="), self.pos)); self.next() }
                '+' => { self.output.push(Token::new(PLUS, String::from("+"), self.pos)); self.next() }
                '-' => { self.output.push(Token::new(MINUS, String::from("-"), self.pos)); self.next() }
                '*' => { self.output.push(Token::new(STAR, String::from("*"), self.pos)); self.next() }
                '/' => { self.output.push(Token::new(SLASH, String::from("/"), self.pos)); self.next() }
                '%' => { self.output.push(Token::new(PERCENT, String::from("%"), self.pos)); self.next() }
                '\n' | '\t' => { self.next() }
                '@' => {
                    self.output.push(Token::new(AT, String::from("@"), self.pos));
                    self.tokenize_address();
                }
                '\0' => {
                    self.output.push(Token::new(EOF, String::from("\0"), 999999));
                    break
                }
                _ => {
                    if self.cur == '0' && self.peek(1) == 'x' { self.tokenize_hexnum() }
                    else if self.cur.is_digit(10) { self.tokenize_num() }
                    else if self.cur.is_alphabetic() { self.tokenize_ident() }
                    else if self.cur.is_whitespace() { self.next() }
                    else { panic!("Unexpected char {} at pos {}", self.cur, self.pos) }
                }
            }
        }
        let mut res: Vec<Token> = Vec::new();
        self.output.iter().clone().for_each(|x| {
            res.push(x.clone())
        });
        res
    }
    fn tokenize_address(&mut self) {
        self.buffer.clear();
        while self.cur.is_digit(16) {
            self.buffer.push(self.cur);
            self.next();
        }
        let address = self.buffer.pack();
        self.output.push(Token::new(ADDRESS, address, self.pos))
    }
    fn tokenize_keyword(&mut self) {
        self.buffer.clear();
        let mut keywords: HashMap<&str, TokenType> = HashMap::from(
            [("$add", ADD),
                ("$sub", SUB),
                ("$mul", MUL),
                ("$div", DIV),
                ("$rem", REM),
                ("$move", MOVE),
                ("$set", SET),
                ("$jmp", JMP),
                ("$jmc", JMC),
                ("$funcdef", FUNCDEF),
                ("$params", PARAMS),
                ("$vardef", VARDEF),
                ("$ret", RET),
                ("$call", CALL),
                ("$halt", HALT),
                ("$exit", EXIT),
                ("$quit", EXIT),
                ("$i8", I8KW),
                ("$i16", I16KW),
                ("$i32", I32KW),
                ("$i64", I64KW),
                ("$unsigned", UNSIGNEDKW),
                ("$u", UNSIGNEDKW),
                ("$float", FLOATKW),
                ("$double", DOUBLEKW),
            ]
        );
        self.buffer.push(self.cur);
        self.next();
        while self.cur.is_alphabetic() || self.cur.is_digit(10) {
            self.buffer.push(self.cur);
            self.next();
        }
        let keyword = self.buffer.pack();
        if let Some(r#type) = keywords.get_mut(keyword.as_str()) {
            self.output.push(Token::new(r#type.clone(), keyword, self.pos))
        } else {
            panic!("Unexpected keyword {}", keyword)
        }
    }
    fn tokenize_num(&mut self) {
        let mut is_floating = false;
        let mut is_float = false;
        self.buffer.clear();
        loop {
            if self.cur.is_digit(10) {
                self.buffer.push(self.cur);
                self.next()
            } else if self.cur == '.' && !is_floating {
                is_floating = true;
                self.buffer.push(self.cur);
                self.next();
            } else if self.cur == '.' && is_floating {
                panic!("Unexpected [.] in floating number. Number already floating")
            } else if self.cur == 'f' {
                is_float = true;
                self.next();
                break
            } else { break }
        }
        let number = self.buffer.pack();
        if is_floating {
            if is_float {
                self.output.push(Token::new(FLOATING { is_double: false }, number, self.pos))
            } else {
                self.output.push(Token::new(FLOATING { is_double: true }, number, self.pos))
            }
        } else {
            self.output.push(Token::new(INTEGER { unsigned: false, size: 32 }, number, self.pos))
        }
    }
    fn tokenize_hexnum(&mut self) {
        self.buffer.clear();
        self.next();
        self.next();
        while self.cur.is_digit(16) {
            self.buffer.push(self.cur);
            self.next()
        }
        let number = self.buffer.pack();
        self.output.push(Token::new(INTEGER { unsigned: true, size: 32 }, number, self.pos))

    }
    fn tokenize_ident(&mut self) {
        self.buffer.clear();
        loop {
            if self.cur.is_alphabetic() || self.cur == '_' {
                self.buffer.push(self.cur);
                self.next()
            } else {
                break
            }
        }
        let ident = self.buffer.pack();
        self.output.push(Token::new(IDENT, ident, self.pos))
    }
    fn has_next(&self) -> bool {
        self.pos < self.size
    }
    fn next(&mut self) {
        self.pos += 1;
        if let Some(chr) = self.input.clone().chars().nth(self.pos) {
            self.cur = chr
        } else {
            self.cur = '\0'
        }
    }
    fn peek(&self, offset: usize) -> char {
        if let Some(chr) = self.input.clone().chars().nth(self.pos + offset) {
            chr
        } else {
            '\0'
        }
    }
}