use crate::ast::associative::associative_list::AssociativeList;
use crate::ast::node::Node;
use crate::{define_order, propagate_constants_generic};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Addition {
    pub items: Vec<Node>,
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

define_order!(Addition, AssociativeList::default_cmp);
