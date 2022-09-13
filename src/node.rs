use crate::token::Token;

pub struct Node {
    token: Token,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    pub fn op_node(token: Token, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        Box::new(Self {
            token,
            lhs: Some(lhs),
            rhs: Some(rhs),
        })
    }

    pub fn num_node(token: Token) -> Box<Node> {
        Box::new(Self {
            token,
            lhs: None,
            rhs: None,
        })
    }
}
pub struct SyntaxTree {}
