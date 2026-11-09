use crate::ast::associative::associative_list::AssociativeList;
use crate::ast::node::Node;
use crate::{define_order, propagate_constants_generic};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Multiplication {
    pub items: Vec<Node>,
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

define_order!(Multiplication, AssociativeList::default_cmp);
