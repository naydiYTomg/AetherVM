use std::fmt::{write, Display, Formatter};

#[derive(Clone, Debug)]
pub enum TokenType {
    DOLLAR,     // $
    MOVE,       // move
    ADD,        // add
    SUB,        // sub
    MUL,        // mul
    DIV,        // div
    REM,        // rem
    SET,        // set
    JMP,        // jmp
    JMC,        // jmc
    FUNCDEF,    // funcdef
    PARAMS,     // params
    VARDEF,     // vardef
    RET,        // ret
    CALL,       // call
    HALT,       // halt
    EXIT,       // exit/quit
    I8KW,       // i8
    I16KW,      // i16
    I32KW,      // i32
    I64KW,      // i64
    AT,         // @
    EQ,         // =
    UNSIGNEDKW, // unsigned/u
    FLOATKW,    // float
    DOUBLEKW,   // double
    INTEGER { unsigned: bool, size: usize },
    ADDRESS,
    FLOATING { is_double: bool },
    IDENT,
    LPAREN,     // (
    RPAREN,     // )
    LBRACE,     // {
    RBRACE,     // }
    COMMA,      // ,
    PERIOD,     // .
    UNDER,      // _
    COLON,      // :
    PLUS,       // +
    STAR,       // *
    MINUS,      // -
    SLASH,      // /
    PERCENT,    // %
    EOF
}

#[derive(Clone)]
pub struct Token {
    r#type: TokenType,
    value: String,
    pos: usize
}
impl Token {
    pub fn new(r#type: TokenType, value: String, pos: usize) -> Token {
        Self {
            r#type,
            value,
            pos
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token with type [{:?}], value [{}], pos [{}]", self.r#type, self.value, self.pos)
    }
}