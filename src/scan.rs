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

pub fn tokenize(source: &str) -> Box<dyn Iterator<Item = Token>> {
    let first = source.chars().next().unwrap_or_default();
    match first {
        '(' => Box::new(std::iter::once(Token {
            kind: TokenKind::LeftParen,
        })),
        _ => Box::new(std::iter::empty()),
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
    fn left_paren_produces_single_token() {
        let source = "(";
        let mut tokens = tokenize(source);
        assert_eq!(
            tokens.next().unwrap(),
            Token {
                kind: TokenKind::LeftParen
            }
        );
    }
}
