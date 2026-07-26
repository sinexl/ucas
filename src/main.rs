mod ast;

use crate::ast::associative::associative_list::{AssociativeList};
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

    fn construct_self(items: &[Node]) -> Self {
        Self {
            items: items.to_vec(),
        }
    }

    fn items(&self) -> &[Node] {
        self.items.as_slice()
    }

    fn flatten(&mut self) {
        let mut new_items = Vec::new();
        for i in &mut self.items {
            match i {
                Node::Addition(other) => {
                    other.flatten();
                    new_items.append(&mut other.items)
                }
                Node::Multiplication(other) => {
                    other.flatten();
                    new_items.push(i.clone())
                }
                Node::Float(_) | Node::Variable(_) => {
                    new_items.push(i.clone());
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

    fn construct_self(items: &[Node]) -> Self {
        Self {
            items: items.to_vec(),
        }
    }

    fn items(&self) -> &[Node] {
        self.items.as_slice()
    }

    fn flatten(&mut self) {
        // todo!()
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

mod shortcuts {
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
        Node::Multiplication(_) => {
            // todo!()
            expr.clone()
        }
        Node::Float(_) => expr.clone(),
        Node::Variable(_) => expr.clone(),
    }
}

fn simplify_addition(this: &Addition) -> Node {
    let mut items = this.items.clone();
    for i in 0..items.len() {
        items[i] = simplify(&items[i]);
    }

    let mut i = 0;
    while i < items.len() {
        let mut j = i + 1;
        while j < items.len() {
            let mut j_removed = false;
            match (&items[i], &items[j]) {
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
                        items[i] = Multiplication::from_items(&[
                            simplify(&addition![Node::Float(1.0), multiplicands[0].clone()]),
                            Node::Variable(this_name.clone()),
                        ]);

                        items.remove(j);
                        j_removed = true;
                    }
                }
                _ => {}
            }
            if !j_removed {
                j += 1;
            }
        }
        i += 1;
    }

    Addition::from_items(items.as_slice())
}

fn main() -> std::io::Result<()> {
    use shortcuts::*;
    let mut expr = addition! {
        variable("x"),
        literal(1.0),
        multiplication! { literal(2.0), variable("x") },
        literal(2.0),

        variable("y"),
        multiplication! { literal(6.0), variable("y") },
    };
    print!("{expr} = \n\t");

    expr = simplify(&expr);

    println!("{expr}");
    Ok(())
}
