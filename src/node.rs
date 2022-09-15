use crate::error::Result;
use crate::token::{TokenKind, TokenStream};

#[derive(Debug)]
pub struct Node {
    kind: TokenKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn lhs(&self) -> Option<&Box<Node>> {
        self.lhs.as_ref()
    }

    pub fn rhs(&self) -> Option<&Box<Node>> {
        self.rhs.as_ref()
    }

    pub fn op_node(kind: TokenKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        Box::new(Self {
            kind,
            lhs: Some(lhs),
            rhs: Some(rhs),
        })
    }

    pub fn num_node(val: u32) -> Box<Node> {
        Box::new(Self {
            kind: TokenKind::Num(val),
            lhs: None,
            rhs: None,
        })
    }
}

pub fn expr(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    let mut node = mul(tokenstream)?;
    loop {
        if tokenstream.consume(TokenKind::Add) {
            node = Node::op_node(TokenKind::Add, node, mul(tokenstream)?);
        } else if tokenstream.consume(TokenKind::Sub) {
            node = Node::op_node(TokenKind::Sub, node, mul(tokenstream)?);
        } else {
            return Ok(node);
        }
    }
}

pub fn mul(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    let mut node = unary(tokenstream)?;
    loop {
        if tokenstream.consume(TokenKind::Mul) {
            node = Node::op_node(TokenKind::Mul, node, unary(tokenstream)?);
        } else if tokenstream.consume(TokenKind::Div) {
            node = Node::op_node(TokenKind::Div, node, unary(tokenstream)?);
        } else {
            return Ok(node);
        }
    }
}

pub fn unary(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    if tokenstream.consume(TokenKind::Add) {
        return primary(tokenstream);
    }
    if tokenstream.consume(TokenKind::Sub) {
        return Ok(Node::op_node(
            TokenKind::Sub,
            Node::num_node(0),
            primary(tokenstream)?,
        ));
    }
    primary(tokenstream)
}

pub fn primary(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    if tokenstream.consume(TokenKind::Lbr) {
        let node = expr(tokenstream)?;
        tokenstream.expect(TokenKind::Rbr)?;
        return Ok(node);
    }
    return Ok(Node::num_node(tokenstream.expect_number()?));
}
