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
    BangEqual, Bang
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
}

pub struct Tokens<'source> {
    chars: std::iter::Peekable<std::str::Chars<'source>>,
}

impl<'source> Iterator for Tokens<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        use TokenKind::*;
        self.chars
            .find(|c| !c.is_ascii_whitespace())
            .map(|c| Token {
                kind: match c {
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
                    '=' if self.chars.next_if_eq(&'=').is_some() => EqualEqual,
                    '=' => Equal,
                    '>' if self.chars.next_if_eq(&'=').is_some() => GreaterEqual,
                    '>' => Greater,
                    '<' if self.chars.next_if_eq(&'=').is_some() => LessEqual,
                    '<' => Less,
                    '!' if self.chars.next_if_eq(&'=').is_some() => BangEqual,
                    '!' => Bang,
                    _ => todo!(),
                },
            })
    }
}

pub fn tokenize<'source>(source: &'source str) -> impl Iterator<Item = Token> + 'source {
    Tokens {
        chars: source.chars().peekable(),
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

        assert_eq!(tokens.next().unwrap(), Token { kind: LeftParen });
        assert_eq!(tokens.next().unwrap(), Token { kind: RightParen });
        assert_eq!(tokens.next().unwrap(), Token { kind: Comma });
        assert_eq!(tokens.next().unwrap(), Token { kind: Dot });
        assert_eq!(tokens.next().unwrap(), Token { kind: Semicolon });
        assert_eq!(tokens.next().unwrap(), Token { kind: LeftBrace });
        assert_eq!(tokens.next().unwrap(), Token { kind: RightBrace });
        assert_eq!(tokens.next().unwrap(), Token { kind: Slash });
        assert_eq!(tokens.next().unwrap(), Token { kind: Minus });
        assert_eq!(tokens.next().unwrap(), Token { kind: Star });
        assert_eq!(tokens.next().unwrap(), Token { kind: Plus });
        assert!(tokens.next().is_none());
    }

    #[test]
    fn whitespace_is_ignored() {
        use TokenKind::*;
        let source = " ( ) .\n  *";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token { kind: LeftParen });
        assert_eq!(tokens.next().unwrap(), Token { kind: RightParen });
        assert_eq!(tokens.next().unwrap(), Token { kind: Dot });
        assert_eq!(tokens.next().unwrap(), Token { kind: Star });
        assert!(tokens.next().is_none());
    }

    #[test]
    fn comparison_operators_match_extra_equal() {
        use TokenKind::*;
        let source = "=!!=<>==<=";
        let mut tokens = tokenize(source);
        assert_eq!(tokens.next().unwrap(), Token { kind: Equal });
        assert_eq!(tokens.next().unwrap(), Token { kind: Bang });
        assert_eq!(tokens.next().unwrap(), Token { kind: BangEqual });
        assert_eq!(tokens.next().unwrap(), Token { kind: Less });
        assert_eq!(tokens.next().unwrap(), Token { kind: GreaterEqual });
        assert_eq!(tokens.next().unwrap(), Token { kind: Equal });
        assert_eq!(tokens.next().unwrap(), Token { kind: LessEqual });
        assert!(tokens.next().is_none());
    }
}
