#[rustfmt::skip]
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Semicolon,
    Minus, Plus, Slash, Star,

    // Comparison operators
    EqualEqual, Equal,
    GreaterEqual, Greater,
    LessEqual, Less,
    BangEqual, Bang,

    Number,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    indices: std::ops::RangeInclusive<usize>,
}

impl Token {
    fn new(kind: TokenKind, indices: std::ops::RangeInclusive<usize>) -> Token {
        Token { kind, indices }
    }
}

pub struct Tokens<'source> {
    char_indices: std::iter::Peekable<std::str::CharIndices<'source>>,
    end_index: usize,
}

impl<'source> Tokens<'source> {
    fn kind(&mut self, next: char) -> TokenKind {
        use TokenKind::*;
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
            '=' if self.next_matches('=') => EqualEqual,
            '=' => Equal,
            '>' if self.next_matches('=') => GreaterEqual,
            '>' => Greater,
            '<' if self.next_matches('=') => LessEqual,
            '<' => Less,
            '!' if self.next_matches('=') => BangEqual,
            '!' => Bang,
            c if c.is_ascii_digit() => self.number(c),
            _ => todo!(),
        }
    }

    fn next_matches(&mut self, expected: char) -> bool {
        let next = self.char_indices.next_if(|actual| actual.1 == expected);
        if let Some((end, _)) = next {
            self.end_index = end;
        }
        next.is_some()
    }

    fn number(&mut self, _: char) -> TokenKind {
        while self
            .char_indices
            .next_if(|(_, c)| c.is_ascii_digit())
            .is_some()
        {
            self.end_index += 1
        }
        TokenKind::Number
    }
}

impl<'source> Iterator for Tokens<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = self
            .char_indices
            .find(|c| !c.1.is_ascii_whitespace())
            .map(|c| {
                Token::new(
                    self.kind(c.1),
                    // override the very first character's end
                    c.0..=std::cmp::max(c.0, self.end_index),
                )
            })?;
        self.end_index = *token.indices.end() + 1;
        Some(token)
    }
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    Tokens {
        char_indices: source.char_indices().peekable(),
        end_index: 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_source_produces_no_tokens() {
        let source = "";
        let mut tokens = tokenize(source);
        assert!(tokens.next().is_none());
    }

    #[test]
    fn valid_single_characters_produce_single_token() {
        use TokenKind::*;
        let source = "(),.;{}/-*+";
        let mut tokens = tokenize(source);

        assert_eq!(tokens.next().unwrap(), Token::new(LeftParen, 0..=0));
        assert_eq!(tokens.next().unwrap(), Token::new(RightParen, 1..=1));
        assert_eq!(tokens.next().unwrap(), Token::new(Comma, 2..=2));
        assert_eq!(tokens.next().unwrap(), Token::new(Dot, 3..=3));
        assert_eq!(tokens.next().unwrap(), Token::new(Semicolon, 4..=4));
        assert_eq!(tokens.next().unwrap(), Token::new(LeftBrace, 5..=5));
        assert_eq!(tokens.next().unwrap(), Token::new(RightBrace, 6..=6));
        assert_eq!(tokens.next().unwrap(), Token::new(Slash, 7..=7));
        assert_eq!(tokens.next().unwrap(), Token::new(Minus, 8..=8));
        assert_eq!(tokens.next().unwrap(), Token::new(Star, 9..=9));
        assert_eq!(tokens.next().unwrap(), Token::new(Plus, 10..=10));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn whitespace_is_ignored() {
        use TokenKind::*;
        let source = " ( ) .\n  *";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(LeftParen, 1..=1));
        assert_eq!(tokens.next().unwrap(), Token::new(RightParen, 3..=3));
        assert_eq!(tokens.next().unwrap(), Token::new(Dot, 5..=5));
        assert_eq!(tokens.next().unwrap(), Token::new(Star, 9..=9));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn comparison_operators_match_extra_equal() {
        use TokenKind::*;
        let source = "===!!=<>==<=>";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(EqualEqual, 0..=1));
        assert_eq!(tokens.next().unwrap(), Token::new(Equal, 2..=2));
        assert_eq!(tokens.next().unwrap(), Token::new(Bang, 3..=3));
        assert_eq!(tokens.next().unwrap(), Token::new(BangEqual, 4..=5));
        assert_eq!(tokens.next().unwrap(), Token::new(Less, 6..=6));
        assert_eq!(tokens.next().unwrap(), Token::new(GreaterEqual, 7..=8));
        assert_eq!(tokens.next().unwrap(), Token::new(Equal, 9..=9));
        assert_eq!(tokens.next().unwrap(), Token::new(LessEqual, 10..=11));
        assert_eq!(tokens.next().unwrap(), Token::new(Greater, 12..=12));
        assert!(tokens.next().is_none());
    }

    #[test]
    fn numbers_are_grouped() {
        use TokenKind::*;
        let source = "123 (534)";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token::new(Number, 0..=2));
        assert_eq!(tokens.next().unwrap(), Token::new(LeftParen, 4..=4));
        assert_eq!(tokens.next().unwrap(), Token::new(Number, 5..=7));
        assert_eq!(tokens.next().unwrap(), Token::new(RightParen, 8..=8));
        assert!(tokens.next().is_none());
    }
}
