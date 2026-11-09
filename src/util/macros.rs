use crate::Node;
use crate::ast::node::Variable;

pub fn number(value: f32) -> Node {
    Node::Float(value)
}
pub fn variable(name: &str) -> Node {
    Node::Variable(Variable {
        name: name.to_string(),
    })
}

#[macro_export]
macro_rules! addition {
    {$($x:expr),+ $(,)?} => {{
        use crate::ast::associative::associative_list::AssociativeList;
        crate::ast::addition::Addition::from_items([$($x, )+])
    }}
}
#[macro_export]
macro_rules! multiplication {
    {$($x:expr),+ $(,)?} => {{
        use crate::ast::associative::associative_list::AssociativeList;
        crate::ast::multiplication::Multiplication::from_items([$($x, )+])
    }}
}

#[macro_export]
macro_rules! vars{
    ( $( $name:expr ),+ $(,)?) => {
        (
            $(crate::util::macros::variable(stringify!($name))),*
        )
    }
}

#[macro_export]
macro_rules! minmax {
    ($a:expr, $b:expr) => {{
        let l_ = $a;
        let r_ = $b;
        if l_ < r_ { (l_, r_) } else { (r_, l_) }
    }};
}
