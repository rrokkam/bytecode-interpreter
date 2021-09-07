use std::fmt::{self, Display};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
            'f' if matches!(self.char_indices.peek(), Some((_, 'a'))) => False,
            'f' if matches!(self.char_indices.peek(), Some((_, 'o'))) => For,
            'f' if matches!(self.char_indices.peek(), Some((_, 'u'))) => Function,
            'i' => If,
            'n' => Nil,
            'o' => Or,
            'p' => Print,
            'r' => Return,
            's' => Super,
            't' if matches!(self.char_indices.peek(), Some((_, 'h'))) => This,
            't' if matches!(self.char_indices.peek(), Some((_, 'r'))) => True,
            'v' => Var,
            'w' => While,
            _ => return self.identifier(),
        };

        let identifier_starts_with_keyword = candidate_keyword
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
            Kind::Keyword(candidate_keyword)
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

    fn check(input: &str, expected: Vec<(usize, Kind)>) {
        assert_eq!(
            tokenize(input).collect::<Vec<_>>(),
            expected
                .iter()
                .map(|(index, kind)| Token::new(*index, *kind))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn empty() {
        let input = "";
        check(input, vec![])
    }

    #[test]
    fn single_character() {
        let input = "(),.;{}/-*+";
        let expected = vec![
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
        check(input, expected)
    }

    #[test]
    fn whitespace() {
        let input = " ( ) .\n  *";
        let expected = vec![(1, LeftParen), (3, RightParen), (5, Dot), (9, Star)];
        check(input, expected)
    }

    #[test]
    fn comparison() {
        let input = "===!!=<>==<=>";
        let expected = vec![
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
        check(input, expected)
    }

    #[test]
    fn number() {
        let input = "123 (534)";
        let expected = vec![(0, Number), (4, LeftParen), (5, Number), (8, RightParen)];
        check(input, expected)
    }

    #[test]
    fn terminated_string() {
        let input = " \"());3.=\"! ";
        let expected = vec![(1, String), (10, Bang)];
        check(input, expected)
    }

    #[test]
    fn unterminated_string() {
        let input = " !\")(;3.=";
        let expected = vec![(1, Bang), (2, Error)];
        check(input, expected)
    }

    #[test]
    fn unrecognized_character() {
        let input = "\\%";
        let expected = vec![(0, Error), (1, Error)];
        check(input, expected)
    }

    #[test]
    fn comment() {
        let input = "#abc\n#1%\n32";
        let expected = vec![(0, Comment), (5, Comment), (9, Number)];
        check(input, expected)
    }

    #[test]
    fn identifier() {
        let input = "these are identifiers";
        let expected = vec![(0, Identifier), (6, Identifier), (10, Identifier)];
        check(input, expected)
    }

    #[test]
    fn keywords() {
        let input =
            "and or true false if else for while class nil super this var function print return";
        let expected = vec![
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
        check(input, expected)
    }

    #[test]
    fn identifier_or_keyword() {
        let input = "fidentifier tidentifier uidentifier";
        let expected = vec![(0, Identifier), (12, Identifier), (24, Identifier)];
        check(input, expected)
    }

    #[test]
    fn keyword_substrings_and_superstrings() {
        let input = "an fun classy nile";
        let expected = vec![
            (0, Identifier),
            (3, Identifier),
            (7, Identifier),
            (14, Identifier),
        ];
        check(input, expected)
    }
}
