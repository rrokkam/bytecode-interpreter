pub struct Token {}

pub fn tokenize(_source: &str) -> impl Iterator<Item = Token> {
    std::iter::empty()
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
}
