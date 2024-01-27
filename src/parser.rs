use crate::expression::Node;

pub struct Parser<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Parser<'a> {
    fn parse_literal(&mut self) -> Node {
        return Node::Literal(1.0);
    }
}
