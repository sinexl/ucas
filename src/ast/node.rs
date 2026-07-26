use std::fmt::{Display, Formatter};
use crate::ast::associative::associative_list::AssociativeList;

#[derive(Debug, Clone)]
pub enum Node {
    // TODO: For now, f32 are used. In future, i plan to use arbitrary precision rational numbers
    Float(f32),
    Variable(String),

    Addition(Addition),
    Multiplication(Multiplication),
}

impl Default for Node {
    fn default() -> Self {
        Node::Float(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct Addition {
    pub items: Vec<Node>,
}
#[derive(Debug, Clone)]
pub struct Multiplication {
    pub items: Vec<Node>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Addition(addition) => addition.fmt_associative(f)?,
            Node::Multiplication(multiplication) => multiplication.fmt_associative(f)?,

            Node::Float(v) => write!(f, "{v}",)?,
            Node::Variable(name) => write!(f, "{name}")?,
        }
        Ok(())
    }
}
