use std::fmt::{self, Display};

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

    // Literals
    Number, String, Identifier,

    Keyword(KeywordKind),

    Comment,

    Error,
}

#[rustfmt::skip]
#[derive(Debug, PartialEq)]
enum KeywordKind {
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
    fn new(position: usize, kind: Kind) -> Token {
        Token { position, kind }
    }
}

pub struct Tokens<'source> {
    char_indices: std::iter::Peekable<std::str::CharIndices<'source>>,
}

impl<'source> Tokens<'source> {
    fn scan_token(&mut self, next: char) -> Kind {
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
            '/' => Slash,
            '#' => self.comment(),
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
            c if c.is_alphabetic() => self.identifier_or_keyword(c),
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

    fn identifier_or_keyword(&mut self, current: char) -> Kind {
        use KeywordKind::*;
        let candidate_keyword = match current {
            'a' => And,
            'c' => Class,
            'e' => Else,
            'f' => match self.char_indices.peek() {
                Some((_, 'a')) => False,
                Some((_, 'o')) => For,
                Some((_, 'u')) => Function,
                _ => return self.identifier(),
            },
            'i' => If,
            'n' => Nil,
            'o' => Or,
            'p' => Print,
            'r' => Return,
            's' => Super,
            't' => match self.char_indices.peek() {
                Some((_, 'h')) => This,
                Some((_, 'r')) => True,
                _ => return self.identifier(),
            },
            'v' => Var,
            'w' => While,
            _ => return self.identifier(),
        };
        self.try_keyword(candidate_keyword)
    }

    fn try_keyword(&mut self, kind: KeywordKind) -> Kind {
        let identifier_starts_with_keyword = kind
            .to_string()
            .chars()
            .skip(1)
            .all(|expected| self.next_matches(expected));
        let keyword_is_not_prefix_of_identifier = self
            .char_indices
            .peek()
            .filter(|actual| actual.1.is_alphanumeric())
            .is_none();

        if identifier_starts_with_keyword && keyword_is_not_prefix_of_identifier {
            Kind::Keyword(kind)
        } else {
            self.identifier()
        }
    }

    fn identifier(&mut self) -> Kind {
        while self
            .char_indices
            .next_if(|(_, c)| c.is_alphanumeric())
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
            .map(|(i, c)| Token::new(i, self.scan_token(c)))
    }
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    Tokens {
        char_indices: source.char_indices().peekable(),
    }
}

#[cfg(test)]
mod test {
    use super::{KeywordKind::*, Kind::*, *};

    fn assert_equality(
        actual: impl Iterator<Item = Token>,
        expected: impl IntoIterator<Item = (usize, Kind)>,
    ) {
        // Asserting element-wise results in cleaner error messages.
        for (actual, expected) in actual.zip(expected) {
            assert_eq!(actual, Token::new(expected.0, expected.1));
        }
    }

    #[test]
    fn empty() {
        let source = "";
        let mut tokens = tokenize(source);
        assert!(tokens.next().is_none());
    }

    #[test]
    fn single_character() {
        let source = "(),.;{}/-*+";
        let actual = tokenize(source);
        let expected = [
            (0, LeftParen),
            (1, RightParen),
            (2, Comma),
            (3, Dot),
            (4, Semicolon),
            (5, LeftBrace),
            (6, RightBrace),
            (7, Slash),
            (8, Minus),
            (9, Star),
            (10, Plus),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn whitespace() {
        let source = " ( ) .\n  *";
        let actual = tokenize(source);
        let expected = [(1, LeftParen), (3, RightParen), (5, Dot), (9, Star)];
        assert_equality(actual, expected)
    }

    #[test]
    fn comparison() {
        let source = "===!!=<>==<=>";
        let actual = tokenize(source);
        let expected = [
            (0, EqualEqual),
            (2, Equal),
            (3, Bang),
            (4, BangEqual),
            (6, Less),
            (7, GreaterEqual),
            (9, Equal),
            (10, LessEqual),
            (12, Greater),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn number() {
        let source = "123 (534)";
        let actual = tokenize(source);
        let expected = [(0, Number), (4, LeftParen), (5, Number), (8, RightParen)];
        assert_equality(actual, expected)
    }

    #[test]
    fn terminated_string() {
        let source = " \"());3.=\"! ";
        let actual = tokenize(source);
        let expected = [(1, String), (10, Bang)];
        assert_equality(actual, expected)
    }

    #[test]
    fn unterminated_string() {
        let source = " !\")(;3.=";
        let actual = tokenize(source);
        let expected = [(1, Bang), (2, Error)];
        assert_equality(actual, expected)
    }

    #[test]
    fn unrecognized_character() {
        let source = "\\%";
        let actual = tokenize(source);
        let expected = [(0, Error), (1, Error)];
        assert_equality(actual, expected)
    }

    #[test]
    fn comment() {
        let source = "#abc\n#1%\n32";
        let actual = tokenize(source);
        let expected = [(0, Comment), (5, Comment), (9, Number)];
        assert_equality(actual, expected)
    }

    #[test]
    fn identifier() {
        let source = "these are identifiers";
        let actual = tokenize(source);
        let expected = [(0, Identifier), (6, Identifier), (10, Identifier)];
        assert_equality(actual, expected)
    }

    #[test]
    fn keywords() {
        let source =
            "and or true false if else for while class nil super this var function print return";
        let actual = tokenize(source);
        let expected = [
            (0, Keyword(And)),
            (4, Keyword(Or)),
            (7, Keyword(True)),
            (12, Keyword(False)),
            (18, Keyword(If)),
            (21, Keyword(Else)),
            (26, Keyword(For)),
            (30, Keyword(While)),
            (36, Keyword(Class)),
            (42, Keyword(Nil)),
            (46, Keyword(Super)),
            (52, Keyword(This)),
            (57, Keyword(Var)),
            (61, Keyword(Function)),
            (70, Keyword(Print)),
            (76, Keyword(Return)),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn identifier_or_keyword() {
        let source = "fidentifier tidentifier uidentifier";
        let actual = tokenize(source);
        let expected = [(0, Identifier), (12, Identifier), (24, Identifier)];
        assert_equality(actual, expected)
    }

    #[test]
    fn keyword_substrings_and_superstrings() {
        let source = "an fun classy nile";
        let actual = tokenize(source);
        let expected = [
            (0, Identifier),
            (3, Identifier),
            (7, Identifier),
            (14, Identifier),
        ];
        assert_equality(actual, expected)
    }
}
