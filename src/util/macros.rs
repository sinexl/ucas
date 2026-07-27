use crate::Node;
use crate::ast::node::Variable;

pub fn literal(value: f32) -> Node {
    Node::Float(value)
}
pub fn variable(name: &str) -> Node {
    Node::Variable(Variable {
        name: name.to_string(),
    })
}

#[macro_export]
macro_rules! addition {
    {$($x:expr),+ $(,)?} => {
        Addition::from_items([$($x, )+])
    }
}
#[macro_export]
macro_rules! multiplication {
    {$($x:expr),+ $(,)?} => {
        Multiplication::from_items([$($x, )+])
    }
}

#[macro_export]
macro_rules! vars{
    ( $( $name:expr ),+ $(,)?) => {
        (
            $(crate::util::macros::variable(stringify!($name))),*
        )
    }
}
