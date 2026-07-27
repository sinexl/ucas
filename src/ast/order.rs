use crate::ast::associative::associative_list::AssociativeList;
use crate::ast::node::Node;
use std::cmp::Ordering;

/// A quick reminder of rust binary relations: weakest to strongest
/// PartialEq: not equivalence relation, like Eq but not necessarily reflexive.
/// So it's possible that x != x (I.e. Nan != Nan, so f32: PartialEq)
/// Eq: equivalence relation,
///    Reflexive: x == x,
///    Symmetric: x == y <=> y == x
///    Transitive: if x == y and y == z, then x == z
/// PartialOrd: partial order,
///    Antisymmetric: if x <= y and y <= x then x == y. Conversely, if x != y, then either x < y or y < x
///    Transitivity: x x <= y and y <= z => x <= z.
///    Mathematically, partial order is also reflexive: a <= a. However, this is not necessarily the case in rust.
///    Which makes PartialEq either strict or non-strict partial order.
/// Ord: total order.
///    Like partial order, but either a <= b or b <= a
impl Ord for Node {
    // Float < Variable < Multiplication < Addition
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Node::Float(a) => match other {
                // TODO: This unwrap is not particularly great, but I'll keep it for now.
                //  This is because, unfortunately, f32 is not totally ordered (because of NaN)
                Node::Float(b) => a.partial_cmp(b).unwrap(),

                Node::Variable(_) | Node::Addition(_) | Node::Multiplication(_) => Ordering::Less,
            },
            Node::Variable(a) => match other {
                Node::Float(_) => Ordering::Greater,

                Node::Variable(b) => (*a).cmp(b),

                Node::Addition(_) | Node::Multiplication(_) => Ordering::Less,
            },
            Node::Multiplication(a) => match other {
                Node::Float(_) | Node::Variable(_) => Ordering::Greater,

                Node::Multiplication(b) => a.default_cmp(b),

                Node::Addition(_) => Ordering::Less,
            },
            Node::Addition(a) => match other {
                Node::Float(_) | Node::Variable(_) | Node::Multiplication(_) => Ordering::Greater,

                Node::Addition(b) => a.default_cmp(b),
            },
        }
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

