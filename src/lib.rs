mod expression;
mod parser;

#[cfg(test)]
mod tests {
    use crate::expression::Node;

    #[test]
    fn test_operations() {
        let node = Node::Literal(10.0) * Node::Literal(10.0);
    }
}
