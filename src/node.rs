use crate::error::Result;
use crate::token::{Operation, TokenStream};

#[derive(Clone, Debug)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(u32),
}

#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    pub fn kind(&self) -> NodeKind {
        self.kind.clone()
    }

    pub fn lhs(&self) -> Option<&Box<Node>> {
        self.lhs.as_ref()
    }

    pub fn rhs(&self) -> Option<&Box<Node>> {
        self.rhs.as_ref()
    }

    pub fn op_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        Box::new(Self {
            kind,
            lhs: Some(lhs),
            rhs: Some(rhs),
        })
    }

    pub fn num_node(val: u32) -> Box<Node> {
        Box::new(Self {
            kind: NodeKind::Num(val),
            lhs: None,
            rhs: None,
        })
    }
}

pub fn expr(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    let mut node = mul(tokenstream)?;
    loop {
        if tokenstream.consume(Operation::Add) {
            node = Node::op_node(NodeKind::Add, node, mul(tokenstream)?);
        } else if tokenstream.consume(Operation::Sub) {
            node = Node::op_node(NodeKind::Sub, node, mul(tokenstream)?);
        } else {
            return Ok(node);
        }
    }
}

pub fn mul(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    let mut node = primary(tokenstream)?;
    loop {
        if tokenstream.consume(Operation::Mul) {
            node = Node::op_node(NodeKind::Mul, node, primary(tokenstream)?);
        } else if tokenstream.consume(Operation::Div) {
            node = Node::op_node(NodeKind::Div, node, primary(tokenstream)?);
        } else {
            return Ok(node);
        }
    }
}

pub fn primary(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    if tokenstream.consume(Operation::LBR) {
        let node = expr(tokenstream)?;
        tokenstream.expect(Operation::RBR)?;
        return Ok(node);
    }
    return Ok(Node::num_node(tokenstream.expect_number()?));
}

//#[test]
//fn testrunner_node() -> Result<()> {
//    use crate::token::tokenize;
//    let mut tokenstream = tokenize("3-2".chars().peekable())?;
//    let node = expr(&mut tokenstream)?;
//    Ok(())
//}
