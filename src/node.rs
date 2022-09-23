use crate::result::CompileError;
use crate::token::{TokenKind, TokenStream};
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub kind: TokenKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
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

    pub fn variable_node(symbol: String, offset: u32) -> Box<Node> {
        Box::new(Self {
            kind: TokenKind::LocalVar { symbol, offset },
            lhs: None,
            rhs: None,
        })
    }
}

pub fn program(tokenstream: &mut TokenStream) -> Result<Vec<Box<Node>>> {
    let mut nodes = Vec::new();

    while let Some(node) = stmt(tokenstream)? {
        nodes.push(node);
    }

    Ok(nodes)
}

pub fn stmt(tokenstream: &mut TokenStream) -> Result<Option<Box<Node>>> {
    if tokenstream.is_empty() {
        return Ok(None);
    }

    if tokenstream.consume(TokenKind::If) {
        if !tokenstream.consume(TokenKind::LRoundBracket) {
            return Err(CompileError::ParseError(Some("stmt LRoundBracket")))?;
        }

        let lhs = Some(expr(tokenstream)?);

        if !tokenstream.consume(TokenKind::RRoundBracket) {
            return Err(CompileError::ParseError(Some("stmt RRoundBracket")))?;
        }

        let then = match stmt(tokenstream)? {
            Some(node) => Some(node),
            None => Err(CompileError::ParseError(Some("expect rhs")))?,
        };

        let rhs = if tokenstream.consume(TokenKind::Else) {
            Some(Box::new(Node {
                kind: TokenKind::Else,
                lhs: then,
                rhs: match stmt(tokenstream)? {
                    Some(node) => Some(node),
                    None => Err(CompileError::ParseError(Some("expect rhs")))?,
                },
            }))
        } else {
            then
        };

        let node = Box::new(Node {
            kind: TokenKind::If,
            lhs,
            rhs,
        });

        return Ok(Some(node));
    }

    if tokenstream.consume(TokenKind::Return) {
        let mut node = Box::new(Node {
            kind: TokenKind::Return,
            lhs: None,
            rhs: None,
        });
        node.lhs = Some(match expr(tokenstream) {
            Ok(node) => node,
            Err(err) => return Err(err)?,
        });

        if let Err(err) = tokenstream.expect(TokenKind::Semicolon) {
            return Err(err)?;
        }

        return Ok(Some(node));
    }

    let node = match expr(tokenstream) {
        Ok(node) => node,
        Err(err) => return Err(err)?,
    };

    if let Err(err) = tokenstream.expect(TokenKind::Semicolon) {
        return Err(err)?;
    }

    Ok(Some(node))
}

pub fn expr(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    Ok(assign(tokenstream)?)
}

pub fn assign(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    let mut node = equality(tokenstream)?;
    if tokenstream.consume(TokenKind::Assign) {
        node = Node::op_node(TokenKind::Assign, node, assign(tokenstream)?);
    }
    Ok(node)
}
pub fn equality(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    let mut node = relational(tokenstream)?;

    loop {
        if tokenstream.consume(TokenKind::Equal) {
            node = Node::op_node(TokenKind::Equal, node, relational(tokenstream)?);
        } else if tokenstream.consume(TokenKind::NEqual) {
            node = Node::op_node(TokenKind::NEqual, node, relational(tokenstream)?);
        } else {
            return Ok(node);
        }
    }
}

pub fn relational(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
    let mut node = add(tokenstream)?;
    loop {
        if tokenstream.consume(TokenKind::Less) {
            node = Node::op_node(TokenKind::Less, node, add(tokenstream)?);
        } else if tokenstream.consume(TokenKind::Greater) {
            node = Node::op_node(TokenKind::Less, add(tokenstream)?, node);
        } else if tokenstream.consume(TokenKind::LessOrEqual) {
            node = Node::op_node(TokenKind::LessOrEqual, node, add(tokenstream)?);
        } else if tokenstream.consume(TokenKind::GreaterOrEqual) {
            node = Node::op_node(TokenKind::LessOrEqual, add(tokenstream)?, node);
        } else {
            return Ok(node);
        }
    }
}

pub fn add(tokenstream: &mut TokenStream) -> Result<Box<Node>> {
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
    if tokenstream.consume(TokenKind::LRoundBracket) {
        let node = expr(tokenstream)?;
        tokenstream.expect(TokenKind::RRoundBracket)?;
        return Ok(node);
    }

    if let Ok(TokenKind::LocalVar { symbol, offset }) = tokenstream.expect_local_variable() {
        return Ok(Node::variable_node(symbol, offset));
    } else {
        return Ok(Node::num_node(tokenstream.expect_number()?));
    }
}

#[test]
fn testrunner_node() -> Result<()> {
    use crate::token::tokenize;
    let test_node = |source: &str, expect: Box<Node>| {
        let mut tokenstream: TokenStream = tokenize(vec![source.to_string()]).unwrap();
        let stmt = stmt(&mut tokenstream).unwrap().unwrap();

        assert_eq!(stmt, expect, "{}", source);
    };

    let expect = Box::new(Node {
        kind: TokenKind::Add,
        lhs: Some(Box::new(Node {
            kind: TokenKind::Num(1),
            lhs: None,
            rhs: None,
        })),
        rhs: Some(Box::new(Node {
            kind: TokenKind::Num(2),
            lhs: None,
            rhs: None,
        })),
    });
    test_node("1+2;", expect);

    let expect = Box::new(Node {
        kind: TokenKind::Assign,
        lhs: Some(Box::new(Node {
            kind: TokenKind::LocalVar {
                symbol: "a".to_string(),
                offset: 8,
            },
            lhs: None,
            rhs: None,
        })),
        rhs: Some(Box::new(Node {
            kind: TokenKind::Num(3),
            lhs: None,
            rhs: None,
        })),
    });
    test_node("a = 3;", expect);

    let expect = Box::new(Node {
        kind: TokenKind::If,
        lhs: Some(Box::new(Node {
            kind: TokenKind::Num(1),
            lhs: None,
            rhs: None,
        })),
        rhs: Some(Box::new(Node {
            kind: TokenKind::Num(4),
            lhs: None,
            rhs: None,
        })),
    });
    test_node("if (1) 4;", expect);
    Ok(())
}
