use crate::ast::addition::Addition;
use crate::ast::associative::associative_list::AssociativeList;
use crate::ast::multiplication::Multiplication;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Node {
    // TODO: For now, f32 are used. In future, i plan to use arbitrary precision rational numbers
    Float(f32),
    Variable(Variable),

    Addition(Addition),
    Multiplication(Multiplication),
}

impl Default for Node {
    fn default() -> Self {
        Node::Float(0.0)
    }
}


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Variable {
    pub name: String,
}

impl Display for Variable{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
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
