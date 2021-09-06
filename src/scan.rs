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

    // Boolean logic-related keywords
    And, Or, True, False,
    If, Else, For, While,

    // Object-related keywords
    Class, Nil, Super, This, Var,

    // Function-related keywords
    Function, Print, Return,

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
        use Kind::*;
        match current {
            'a' => self.try_keyword("nd", And),
            'c' => self.try_keyword("lass", Class),
            'e' => self.try_keyword("lse", Else),
            'f' => match self.char_indices.peek() {
                Some((_, 'a')) => self.try_keyword("alse", False),
                Some((_, 'o')) => self.try_keyword("or", For),
                Some((_, 'u')) => self.try_keyword("unction", Function),
                _ => self.identifier(),
            },
            'i' => self.try_keyword("f", If),
            'n' => self.try_keyword("il", Nil),
            'o' => self.try_keyword("r", Or),
            'p' => self.try_keyword("rint", Print),
            'r' => self.try_keyword("eturn", Return),
            's' => self.try_keyword("uper", Super),
            't' => match self.char_indices.peek() {
                Some((_, 'h')) => self.try_keyword("his", This),
                Some((_, 'r')) => self.try_keyword("rue", True),
                _ => self.identifier(),
            },
            'v' => self.try_keyword("ar", Var),
            'w' => self.try_keyword("hile", While),
            _ => self.identifier(),
        }
    }

    fn try_keyword(&mut self, keyword: &str, keyword_kind: Kind) -> Kind {
        let identifier_starts_with_keyword = keyword
            .chars()
            .find(|&expected| !self.next_matches(expected))
            .is_none();
        let keyword_is_not_prefix_of_identifier = self
            .char_indices
            .peek()
            .filter(|actual| actual.1.is_alphanumeric())
            .is_none();

        if identifier_starts_with_keyword && keyword_is_not_prefix_of_identifier {
            keyword_kind
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

    fn assert_equality(
        actual: impl Iterator<Item = Token>,
        expected: impl IntoIterator<Item = Token>,
    ) {
        // this method could be replaced by `assert_eq!(actual, expected)`,
        // but asserting element-wise like this gives a cleaner error message.
        for (actual, expected) in actual.zip(expected) {
            assert_eq!(actual, expected);
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
            Token::new(0, LeftParen),
            Token::new(1, RightParen),
            Token::new(2, Comma),
            Token::new(3, Dot),
            Token::new(4, Semicolon),
            Token::new(5, LeftBrace),
            Token::new(6, RightBrace),
            Token::new(7, Slash),
            Token::new(8, Minus),
            Token::new(9, Star),
            Token::new(10, Plus),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn whitespace() {
        let source = " ( ) .\n  *";
        let actual = tokenize(source);
        let expected = [
            Token::new(1, LeftParen),
            Token::new(3, RightParen),
            Token::new(5, Dot),
            Token::new(9, Star),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn comparison() {
        let source = "===!!=<>==<=>";
        let actual = tokenize(source);
        let expected = [
            Token::new(0, EqualEqual),
            Token::new(2, Equal),
            Token::new(3, Bang),
            Token::new(4, BangEqual),
            Token::new(6, Less),
            Token::new(7, GreaterEqual),
            Token::new(9, Equal),
            Token::new(10, LessEqual),
            Token::new(12, Greater),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn number() {
        let source = "123 (534)";
        let actual = tokenize(source);
        let expected = [
            Token::new(0, Number),
            Token::new(4, LeftParen),
            Token::new(5, Number),
            Token::new(8, RightParen),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn terminated_string() {
        let source = " \"());3.=\"! ";
        let actual = tokenize(source);
        let expected = [
            Token::new(1, String),
            Token::new(10, Bang),
        ];
        assert_equality(actual, expected)

    }

    #[test]
    fn unterminated_string() {
        let source = " !\")(;3.=";
        let actual = tokenize(source);
        let expected = [
            Token::new(1, Bang),
            Token::new(2, Error),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn unrecognized_character() {
        let source = "\\%";
        let actual = tokenize(source);
        let expected = [
            Token::new(0, Error),
            Token::new(1, Error),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn comment() {
        let source = "//abc\n//1%#\n32";
        let actual = tokenize(source);
        let expected = [
            Token::new(0, Comment),
            Token::new(6, Comment),
            Token::new(12, Number),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn identifier() {
        let source = "these are all identifiers;";
        let actual = tokenize(source);
        let expected = [
            Token::new(0, Identifier),
            Token::new(6, Identifier),
            Token::new(10, Identifier),
            Token::new(14, Identifier),
            Token::new(25, Semicolon),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn keywords() {
        let source =
            "and or true false if else for while class nil super this var function print return";
        let actual = tokenize(source);
        let expected = [
            Token::new(0, And),
            Token::new(4, Or),
            Token::new(7, True),
            Token::new(12, False),
            Token::new(18, If),
            Token::new(21, Else),
            Token::new(26, For),
            Token::new(30, While),
            Token::new(36, Class),
            Token::new(42, Nil),
            Token::new(46, Super),
            Token::new(52, This),
            Token::new(57, Var),
            Token::new(61, Function),
            Token::new(70, Print),
            Token::new(76, Return),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn identifier_or_keyword() {
        let source = "fidentifier tidentifier uidentifier";
        let actual = tokenize(source);
        let expected = [
            Token::new(0, Identifier),
            Token::new(12, Identifier),
            Token::new(24, Identifier),
        ];
        assert_equality(actual, expected)
    }

    #[test]
    fn keyword_substrings_and_superstrings() {
        let source = "an fun classy nile";
        let actual = tokenize(source);
        let expected = [
            Token::new(0, Identifier),
            Token::new(3, Identifier),
            Token::new(7, Identifier),
            Token::new(14, Identifier),
        ];
        assert_equality(actual, expected)
    }
}
