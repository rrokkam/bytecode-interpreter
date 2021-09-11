use std::fmt::{self, Display};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Semicolon,
    Minus, Plus, Slash, Star,

    // Comparison operators
    EqualEqual, Equal,
    GreaterEqual, Greater,
    LessEqual, Less,
    BangEqual, Bang,

    // Literals
    Number, String, Identifier,

    Keyword(KeywordKind),

    Comment,

    Error,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeywordKind {
    // Boolean logic-related keywords
    And, Or, True, False,
    If, Else, For, While,

    // Object-related keywords
    Class, Nil, Super, This, Var,

    // Function-related keywords
    Function, Print, Return,
}

impl Display for KeywordKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use KeywordKind::*;
        let keyword_string = match self {
            And => "and",
            Or => "or",
            True => "true",
            False => "false",
            If => "if",
            Else => "else",
            For => "for",
            While => "while",
            Class => "class",
            Nil => "nil",
            Super => "super",
            This => "this",
            Var => "var",
            Function => "function",
            Print => "print",
            Return => "return",
        };
        write!(f, "{}", keyword_string)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    position: usize,
    kind: Kind,
}

impl Token {
    pub fn new(position: usize, kind: Kind) -> Token {
        Token { position, kind }
    }
}
