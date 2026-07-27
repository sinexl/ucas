use crate::ast::node::Node;
use crate::simplify;

pub fn run(expr: Node) -> String {
    simplify(&expr).to_string()
}
