mod ast;

use crate::ast::node::{Addition, Multiplication, Node};
use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

trait AssociativeList {
    /// Main constructor. This should (if applicable):
    /// - Perform flattening (AssociativeList::flatten())
    /// - Remove Identities
    /// - Propagate Constants (AssociativeList::propagate_constants())
    /// - Ensure that length of list > 1
    /// - Ensure that the items are sorted according to Node's Ord implementation (AssociativeList::ensure_ordered())
    fn from_items(items: &[Node]) -> Node;
    fn items(&self) -> &[Node];

    fn flatten(&mut self);
    fn propagate_constants(&mut self);
    fn ensure_sorted(&mut self);
    fn simplify(&self) -> Node;

    // If two lists are of different length, compare them by their length.
    // Otherwise, compare their items in lexicographic order.
    // This method assumes that self.items and other.items are already sorted.
    fn default_cmp(&self, other: &Self) -> Ordering {
        let length_order = self.items().len().cmp(&other.items().len());
        // If two lists are of different len
        if length_order != Ordering::Equal {
            return length_order;
        }

        let is_sorted = |arr: &[Node]| arr.windows(2).all(|w| w[0] <= w[1]);
        debug_assert!(is_sorted(self.items()));
        debug_assert!(is_sorted(other.items()));

        self.items().cmp(other.items())
    }

    fn fmt_associative(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.symbol();
        write!(f, "(")?;
        let items = self.items();
        for i in 0..items.len() {
            write!(f, "{}", items[i])?;
            if i != items.len() - 1 {
                write!(f, " {symbol} ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
    fn symbol(&self) -> &'static str;
}
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
    fn from_items(items: &[Node]) -> Node {
        let mut this = Self {
            items: items.to_vec(),
        };
        this.flatten();

        // Clearing identities
        let mut i = 0;
        while i < this.items.len() {
            if let Node::Float(x) = this.items[i]
                && x == 0.0
            {
                this.items.remove(i);
            } else {
                i += 1;
            }
        }

        this.propagate_constants();

        if this.items.len() == 0 {
            return Node::Float(0.0);
        }
        if this.items.len() == 1 {
            return this.items[0].clone();
        }

        this.ensure_sorted();

        Node::Addition(this)
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

    fn simplify(&self) -> Node {
        let mut items = self.items.clone();
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

        Self::from_items(items.as_slice())
    }

    fn symbol(&self) -> &'static str {
        "+"
    }
}

impl AssociativeList for Multiplication {
    fn from_items(items: &[Node]) -> Node {
        let mut this = Self {
            items: items.to_vec(),
        };

        this.flatten();

        // Clearing identities
        let mut i = 0;
        while i < this.items.len() {
            let mut i_removed = false;
            if let Node::Float(x) = this.items[i] {
                if x == 1.0 {
                    this.items.remove(i);
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

        this.propagate_constants();

        if this.items.len() == 0 {
            return Node::Float(1.0);
        }
        if this.items.len() == 1 {
            return this.items[0].clone();
        }
        this.ensure_sorted();

        Node::Multiplication(this)
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

    fn simplify(&self) -> Node {
        // todo!()
        Node::Multiplication(self.clone())
    }

    fn symbol(&self) -> &'static str {
        "*"
    }
}

fn simplify(expr: &Node) -> Node {
    match expr {
        Node::Addition(addition) => addition.simplify(),
        Node::Multiplication(multiplication) => multiplication.simplify(),
        Node::Float(_) => expr.clone(),
        Node::Variable(_) => expr.clone(),
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
