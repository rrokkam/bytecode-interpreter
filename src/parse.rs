use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Instruction {}

struct Instructions<T>
where
    T: Iterator<Item = Token>,
{
    tokens: std::iter::Peekable<T>,
}

impl<T> Instructions<T>
where
    T: Iterator<Item = Token>,
{
    fn new(tokens: T) -> Self {
        Instructions {
            tokens: tokens.peekable(),
        }
    }
}

impl<T> Iterator for Instructions<T>
where
    T: Iterator<Item = Token>,
{
    type Item = Vec<Instruction>;

    fn next(&mut self) -> Option<Vec<Instruction>> {
        let next = self.tokens.next()?;
        todo!()
    }
}

fn parse(tokens: impl Iterator<Item = Token>) -> impl Iterator<Item = Instruction> {
    Instructions::new(tokens).flatten()
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(input: &str, expected: Vec<Instruction>) {
        assert_eq!(
            parse(crate::scan::tokenize(input)).collect::<Vec<Instruction>>(),
            expected
        );
    }

    #[test]
    fn empty() {
        check("", vec![]);
    }
}
