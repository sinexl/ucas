mod ast;
#[cfg(test)]
mod tests;
pub mod util;

use crate::ast::associative::associative_list::AssociativeList;
use crate::ast::node::{Addition, Multiplication, Node};

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
impl AssociativeList for Addition {
    fn construct_node_variant(self) -> Node {
        Node::Addition(self)
    }

    fn construct_self(items: impl IntoIterator<Item = Node>) -> Self {
        Self {
            items: items.into_iter().collect::<Vec<_>>(),
        }
    }

    fn items(&self) -> &[Node] {
        self.items.as_slice()
    }

    fn flatten(&mut self) {
        let mut new_items = Vec::new();
        for mut i in self.items.drain(..) {
            match i {
                Node::Addition(mut other) => {
                    other.flatten();
                    new_items.append(&mut other.items)
                }
                Node::Multiplication(ref mut other) => {
                    other.flatten();
                    new_items.push(i)
                }
                Node::Float(_) | Node::Variable(_) => {
                    new_items.push(i);
                }
            }
        }
        self.items = new_items;
    }

    fn propagate_constants(&mut self) {
        propagate_constants_generic(&mut self.items, |a, b| a + b);
    }

    fn ensure_sorted(&mut self) {
        self.items.sort()
    }

    fn normalize(mut self) -> Node {
        for i in &mut self.items {
            *i = std::mem::take(i).normalize()
        }
        self.flatten();

        // Clearing identities
        let mut i = 0;
        while i < self.items.len() {
            if let Node::Float(x) = self.items[i]
                && x == 0.0
            {
                self.items.remove(i);
            } else {
                i += 1;
            }
        }

        self.propagate_constants();

        if self.items.len() == 0 {
            return Node::Float(0.0);
        }
        if self.items.len() == 1 {
            return self.items.remove(0); // Move out the vector
        }

        self.ensure_sorted();
        Node::Addition(self)
    }

    fn symbol(&self) -> &'static str {
        "+"
    }
}

impl AssociativeList for Multiplication {
    fn construct_node_variant(self) -> Node {
        Node::Multiplication(self)
    }

    fn construct_self(items: impl IntoIterator<Item = Node>) -> Self {
        Self {
            items: items.into_iter().collect::<Vec<_>>(),
        }
    }

    fn items(&self) -> &[Node] {
        self.items.as_slice()
    }

    fn flatten(&mut self) {
        let mut new_items = Vec::new();

        for mut i in self.items.drain(..) {
            match i {
                Node::Multiplication(ref mut other) => {
                    other.flatten();
                    new_items.append(&mut other.items)
                }

                Node::Float(_) | Node::Variable(_) => new_items.push(i),
                Node::Addition(ref mut addition) => {
                    // Distributive rule is not applied here.
                    addition.flatten();
                    new_items.push(i)
                }
            }
        }

        self.items = new_items;
    }

    fn propagate_constants(&mut self) {
        propagate_constants_generic(&mut self.items, |a, b| a * b);
    }

    fn ensure_sorted(&mut self) {
        self.items.sort()
    }
    fn normalize(mut self) -> Node {
        for i in &mut self.items {
            *i = std::mem::take(i).normalize()
        }
        self.flatten();

        // Clearing identities
        let mut i = 0;
        while i < self.items.len() {
            let mut i_removed = false;
            if let Node::Float(x) = self.items[i] {
                if x == 1.0 {
                    self.items.remove(i);
                    i_removed = true;
                }
                if x == 0.0 {
                    return Node::Float(0.0);
                }
            }

            if !i_removed {
                i += 1;
            }
        }

        self.propagate_constants();

        if self.items.len() == 0 {
            return Node::Float(1.0);
        }
        if self.items.len() == 1 {
            return self.items.remove(0);
        }
        self.ensure_sorted();
        Node::Multiplication(self)
    }

    fn symbol(&self) -> &'static str {
        "*"
    }
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
        let multiplication_items = to_distribute.clone().chain(std::iter::once(i.clone()));
        let multiplication = Multiplication::from_items(multiplication_items);
        items.push(simplify(&multiplication));
    }

    Addition::from_items(items)
}
fn simplify_multiplication(this: &Multiplication) -> Node {
    let items = this.items.iter().map(simplify).collect::<Vec<_>>();
    for (idx, i) in items.iter().enumerate() {
        if let Node::Addition(addition) = i {
            return distribute(&items, addition, idx);
        }
    }
    // todo: other simplifications
    Node::Multiplication(this.clone())
}

fn simplify_addition(this: &Addition) -> Node {
    let mut items = this.items.iter().map(simplify).collect::<Vec<_>>();

    let mut i = 0;
    while i < items.len() {
        let mut j = i + 1;
        while j < items.len() {
            let mut j_removed = false;
            match (&items[i], &items[j]) {
                // x + x = 2x
                (Node::Variable(a), Node::Variable(b)) if a == b => {
                    let j = items.remove(j);
                    j_removed = true;
                    items[i] = multiplication![Node::Float(2.0), j]
                }
                // x + nx = (1 + n)x
                (
                    Node::Variable(this_name),
                    Node::Multiplication(Multiplication {
                        items: multiplicands,
                    }),
                ) => {
                    if let Node::Variable(other_name) = &multiplicands[1]
                        && *other_name == *this_name
                    {
                        // x + nx = ((1 + n)*x)
                        items[i] = multiplication![
                            simplify(&addition![Node::Float(1.0), multiplicands[0].clone()]),
                            Node::Variable(this_name.clone()),
                        ];

                        items.remove(j);
                        j_removed = true;
                    }
                }
                // ax + bx = (a + b)x
                _ => {}
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
    use util::macros::*;
    let (a, b, c, d, e, f) = vars!(a, b, c, d, e, f);
    // let mut expr = multiplication![a, addition!(b, c)];
    let mut expr = multiplication! {
        addition!(a, b),
        addition!(c, d),
        addition!(e, f),
    };
    print!("{expr} = \n\t");

    expr = simplify(&expr);

    println!("{expr}");
    Ok(())
}
