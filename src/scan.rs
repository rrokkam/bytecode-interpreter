#[rustfmt::skip]
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Semicolon,
    Minus, Plus, Slash, Star,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
}

pub struct Tokens<'source> {
    chars: std::str::Chars<'source>,
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
                    _ => todo!(),
                },
            })
    }
}

pub fn tokenize<'source>(source: &'source str) -> impl Iterator<Item = Token> + 'source {
    Tokens {
        chars: source.chars(),
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
        let source = " \n  \t\n\r\n";
        let mut tokens = tokenize(source);
        assert!(tokens.next().is_none());
    }
}
