mod ast;
#[cfg(test)]
mod tests;
pub mod util;

use crate::ast::addition::Addition;
use crate::ast::associative::associative_list::AssociativeList;
use crate::ast::multiplication::Multiplication;
use crate::ast::node::Node;
use crate::util::macros::number;
use std::iter;

// I decided to not put this implementation into AssociativeList because that would require also
// adding .action(f32, f32) to the interface.
fn propagate_constants_generic(items: &mut Vec<Node>, action: impl Fn(f32, f32) -> f32) {
    let mut constant = None;
    let mut out = Vec::new();

    for i in items.drain(..) {
        match i {
            Node::Float(x) => {
                constant = Some(match constant {
                    None => x,
                    Some(c) => action(c, x),
                })
            }
            other => out.push(other),
        }
    }

    if let Some(c) = constant {
        out.push(Node::Float(c));
    }
    *items = out;
}


impl Node {
    fn normalize(self) -> Node {
        match self {
            Node::Float(_) => self,
            Node::Variable(_) => self,
            Node::Addition(addition) => addition.normalize(),
            Node::Multiplication(multiplication) => multiplication.normalize(),
        }
    }
}

fn simplify(expr: &Node) -> Node {
    match expr {
        Node::Addition(addition) => simplify_addition(addition),
        Node::Multiplication(multiplication) => simplify_multiplication(multiplication),
        Node::Float(_) => expr.clone(),
        Node::Variable(_) => expr.clone(),
    }
}

fn distribute(items: &[Node], addition: &Addition, addition_idx: usize) -> Node {
    // Select all items except for the addition into which everything will be distributed.
    let to_distribute = items
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != addition_idx)
        .map(|(_, item)| item)
        .cloned();

    let mut items = Vec::with_capacity(addition.items.len());
    for i in &addition.items {
        let multiplication_items = to_distribute.clone().chain(iter::once(i.clone()));
        let multiplication = Multiplication::from_items(multiplication_items);
        items.push(simplify(&multiplication));
    }

    Addition::from_items(items)
}
fn simplify_multiplication(this: &Multiplication) -> Node {
    let items = this.items.iter().map(simplify).collect::<Vec<_>>();
    for (idx, i) in items.iter().enumerate() {
        if let Node::Addition(addition) = i {
            return simplify(&distribute(&items, addition, idx));
        }
    }
    // todo: other simplifications
    Node::Multiplication(this.clone())
}

fn match_similar_monomials_multiplication<'a>(
    left: &'a Multiplication,
    right: &'a Multiplication,
) -> Option<(f32, f32, Vec<Node>)> {
    let mut l_idx = 0;
    let mut r_idx = 0;

    let a = if let Node::Float(a) = left.items[0] {
        l_idx = 1;
        a
    } else {
        1.0
    };
    let b = if let Node::Float(b) = right.items[0] {
        r_idx = 1;
        b
    } else {
        1.0
    };

    let left_variables = &left.items[l_idx..];
    let right_variables = &right.items[r_idx..];
    if left_variables == right_variables {
        return Some((
            a,
            b,
            left_variables.into_iter().cloned().collect::<Vec<_>>(),
        ));
    }
    None
}

// matches if two multiplication terms are of form:
// left: a*xyzw..., right: b*xyzw.., (monomials) where a and b are numbers, xyzw... are variables.
// If yes, returns pair (a, b) and list of variables xyzw...

fn match_similar_monomials<'a>(left: &'a Node, right: &'a Node) -> Option<(f32, f32, Vec<Node>)> {
    // possible cases:
    // Var + Var
    // Var + n * Var
    // a*Var + b*VarList

    let (left, right) = minmax!(left, right);
    match (left, right) {
        (Node::Variable(a), Node::Variable(b)) => {
            if a == b {
                return Some((1.0, 1.0, vec![Node::Variable(a.clone())]));
            }
            None
        }
        (Node::Variable(a), Node::Multiplication(multiplication)) => {
            match_similar_monomials_multiplication(
                &Multiplication::construct_self([Node::Variable(a.clone())]),
                multiplication,
            )
        }
        (Node::Multiplication(l), Node::Multiplication(r)) => {
            match_similar_monomials_multiplication(l, r)
        }
        (Node::Float(_), _) => None,
        (Node::Addition(_), _) => None,
        (Node::Variable(_), _) => None,
        (&Node::Multiplication(_), _) => None,
    }
}

fn simplify_addition(this: &Addition) -> Node {
    let mut items = this.items.iter().map(simplify).collect::<Vec<_>>();

    let mut i = 0;
    while i < items.len() {
        let mut j = i + 1;
        while j < items.len() {
            let mut j_removed = false;
            // a*xyzw... + b*xyzw... = (a + b)*xyzw...
            if let Some((a, b, vars)) = match_similar_monomials(&items[i], &items[j]) {
                items.remove(j);
                j_removed = true;
                items[i] = Multiplication::from_items(iter::once(number(a + b)).chain(vars));
            }
            if !j_removed {
                j += 1;
            }
        }
        i += 1;
    }

    Addition::from_items(items)
}

fn main() -> std::io::Result<()> {

    let (x, y) = vars!(x, y);
    let expr = simplify(&multiplication!(
        addition!(x.clone(), y.clone()),
        addition!(y.clone(), x.clone()),
        // addition!(y.clone(), x.clone()),
    ));
    println!("{}", expr);
    let expr = simplify(&addition!(
        multiplication!(x.clone(), y.clone()),
        multiplication!(y.clone(), x.clone(), number(2.0))
    ));
    println!("{}", expr);

    Ok(())
}
