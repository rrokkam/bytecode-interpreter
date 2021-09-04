#[rustfmt::skip]
#[derive(Debug, PartialEq)]
enum Kind {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Semicolon,
    Minus, Plus, Slash, Star,

    // Comparison operators
    EqualEqual, Equal,
    GreaterEqual, Greater,
    LessEqual, Less,
    BangEqual, Bang,

    Number, String, Identifier,

    Comment,

    Error,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    position: usize,
    kind: Kind,
}

impl Token {
    fn new(position: usize, kind: Kind) -> Token {
        Token { position, kind }
    }
}

pub struct Tokens<'source> {
    char_indices: std::iter::Peekable<std::str::CharIndices<'source>>,
}

impl<'source> Tokens<'source> {
    fn kind(&mut self, next: char) -> Kind {
        use Kind::*;
        match next {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ',' => Comma,
            '.' => Dot,
            ';' => Semicolon,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' if self.next_matches('/') => self.comment(),
            '/' => Slash,
            '=' if self.next_matches('=') => EqualEqual,
            '=' => Equal,
            '>' if self.next_matches('=') => GreaterEqual,
            '>' => Greater,
            '<' if self.next_matches('=') => LessEqual,
            '<' => Less,
            '!' if self.next_matches('=') => BangEqual,
            '!' => Bang,
            '"' => self.string(),
            c if c.is_numeric() => self.number(),
            c if c.is_alphabetic() => self.identifier(),
            _ => Error,
        }
    }

    fn next_matches(&mut self, expected: char) -> bool {
        self.char_indices
            .next_if(|actual| actual.1 == expected)
            .is_some()
    }

    fn next_until_eq(&mut self, expected: char) {
        while self
            .char_indices
            .next_if(|actual| actual.1 != expected)
            .is_some()
        {}
    }

    fn identifier(&mut self) -> Kind {
        while self
            .char_indices
            .next_if(|(_, c)| c.is_alphabetic())
            .is_some()
        {}
        Kind::Identifier
    }

    fn number(&mut self) -> Kind {
        while self.char_indices.next_if(|(_, c)| c.is_numeric()).is_some() {}
        Kind::Number
    }

    fn string(&mut self) -> Kind {
        self.next_until_eq('"');
        if let Some((_, '"')) = self.char_indices.next() {
            Kind::String
        } else {
            Kind::Error
        }
    }

    fn comment(&mut self) -> Kind {
        self.next_until_eq('\n');
        Kind::Comment
    }
}

impl<'source> Iterator for Tokens<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.char_indices
            .find(|(_, c)| !c.is_whitespace())
            .map(|(i, c)| Token::new(i, self.kind(c)))
    }
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    Tokens {
        char_indices: source.char_indices().peekable(),
    }
}

#[cfg(test)]
mod test {
    use super::{Kind::*, *};

    #[test]
    fn empty() {
        let source = "";
        let mut tokens = tokenize(source);
        assert!(tokens.next().is_none());
    }

    #[test]
    fn single_character() {
        let source = "(),.;{}/-*+";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(0, LeftParen));
        assert_eq!(tokens.next().unwrap(), Token::new(1, RightParen));
        assert_eq!(tokens.next().unwrap(), Token::new(2, Comma));
        assert_eq!(tokens.next().unwrap(), Token::new(3, Dot));
        assert_eq!(tokens.next().unwrap(), Token::new(4, Semicolon));
        assert_eq!(tokens.next().unwrap(), Token::new(5, LeftBrace));
        assert_eq!(tokens.next().unwrap(), Token::new(6, RightBrace));
        assert_eq!(tokens.next().unwrap(), Token::new(7, Slash));
        assert_eq!(tokens.next().unwrap(), Token::new(8, Minus));
        assert_eq!(tokens.next().unwrap(), Token::new(9, Star));
        assert_eq!(tokens.next().unwrap(), Token::new(10, Plus));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn whitespace() {
        let source = " ( ) .\n  *";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(1, LeftParen));
        assert_eq!(tokens.next().unwrap(), Token::new(3, RightParen));
        assert_eq!(tokens.next().unwrap(), Token::new(5, Dot));
        assert_eq!(tokens.next().unwrap(), Token::new(9, Star));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn comparison() {
        let source = "===!!=<>==<=>";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(0, EqualEqual));
        assert_eq!(tokens.next().unwrap(), Token::new(2, Equal));
        assert_eq!(tokens.next().unwrap(), Token::new(3, Bang));
        assert_eq!(tokens.next().unwrap(), Token::new(4, BangEqual));
        assert_eq!(tokens.next().unwrap(), Token::new(6, Less));
        assert_eq!(tokens.next().unwrap(), Token::new(7, GreaterEqual));
        assert_eq!(tokens.next().unwrap(), Token::new(9, Equal));
        assert_eq!(tokens.next().unwrap(), Token::new(10, LessEqual));
        assert_eq!(tokens.next().unwrap(), Token::new(12, Greater));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn number() {
        let source = "123 (534)";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(0, Number));
        assert_eq!(tokens.next().unwrap(), Token::new(4, LeftParen));
        assert_eq!(tokens.next().unwrap(), Token::new(5, Number));
        assert_eq!(tokens.next().unwrap(), Token::new(8, RightParen));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn terminated_string() {
        let source = " \"());3.=\"! ";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(1, String));
        assert_eq!(tokens.next().unwrap(), Token::new(10, Bang));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn unterminated_string() {
        let source = " !\")(;3.=";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(1, Bang));
        assert_eq!(tokens.next().unwrap(), Token::new(2, Error));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn unrecognized_character() {
        let source = "\\%";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(0, Error));
        assert_eq!(tokens.next().unwrap(), Token::new(1, Error));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn comment() {
        let source = "//abc\n//1%#\n32";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(0, Comment));
        assert_eq!(tokens.next().unwrap(), Token::new(6, Comment));
        assert_eq!(tokens.next().unwrap(), Token::new(12, Number));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn identifier() {
        let source = "these are all identifiers;";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(0, Identifier));
        assert_eq!(tokens.next().unwrap(), Token::new(6, Identifier));
        assert_eq!(tokens.next().unwrap(), Token::new(10, Identifier));
        assert_eq!(tokens.next().unwrap(), Token::new(14, Identifier));
        assert_eq!(tokens.next().unwrap(), Token::new(25, Semicolon));
        assert!(tokens.next().is_none());
    }
}
