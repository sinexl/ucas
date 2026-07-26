use crate::Node;

pub fn literal(value: f32) -> Node {
    Node::Float(value)
}
pub fn variable(name: &str) -> Node {
    Node::Variable(name.to_string())
}

#[macro_export]
macro_rules! addition {
    {$($x:expr),+ $(,)?} => {
        Addition::from_items(&[$($x, )+])
    }
}
#[macro_export]
macro_rules! multiplication {
    {$($x:expr),+ $(,)?} => {
        Multiplication::from_items(&[$($x, )+])
    }
}
