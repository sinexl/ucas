use std::fmt::{Display, Formatter};
#[derive(Clone)]
enum Node {
    Addition(Addition),
    Float(f32),
    Variable(String),
}

#[derive(Clone)]
struct Addition {
    items: Vec<Box<Node>>,
}

trait AssociativeList {
    fn items(&self) -> &[Box<Node>];
    fn flatten(&self) -> Vec<Box<Node>>;

    fn construct_node(items: &[Box<Node>]) -> Node;

    fn fmt_associative(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        let items = self.items();
        for i in 0..items.len() {
            write!(f, "{}", items[i])?;
            if i != items.len() - 1 {
                write!(f, " + ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }

    fn simplify_associative(&self) -> Node {
        let mut items = self.flatten().to_vec();
        for i in 0..items.len() {
            items[i] = Box::new(simplify(&items[i]));
        }

        for i in 0..items.len() {
            let mut j = i + 1;
            while j < items.len() {
                let mut removed = false;
                use Node::*;
                match (items[i].as_ref(), items[j].as_ref()) {
                    (Float(a), Float(b)) => {
                        items[i] = Box::new(Float(a + b));
                        items.remove(j);
                        removed = true;
                    }
                    _ => {}
                }
                if !removed {
                    j += 1;
                }
            }
        }

        Self::construct_node(items.as_slice())
    }
}

impl AssociativeList for Addition {
    fn items(&self) -> &[Box<Node>] {
        self.items.as_slice()
    }

    fn flatten(&self) -> Vec<Box<Node>> {
        let mut new_list: Vec<Box<Node>> = Vec::new();
        for i in &self.items {
            match i.as_ref() {
                Node::Addition(other) => {
                    let mut other = other.flatten();
                    new_list.append(&mut other);
                }
                Node::Float(_) | Node::Variable(_) => {
                    new_list.push(i.clone());
                }
            }
        }
        new_list
    }

    fn construct_node(items: &[Box<Node>]) -> Node {
        Node::Addition(Addition {
            items: items.to_vec(),
        })
    }
}

impl Node {
    fn addition(items: Vec<Box<Node>>) -> Node {
        Node::Addition(Addition { items })
    }
}



fn simplify(expr: &Node) -> Node {
    match expr {
        Node::Addition(addition) => {
            addition.simplify_associative()
        }
        Node::Float(_) => expr.clone(),
        Node::Variable(_) => expr.clone(),
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Addition(addition) => addition.fmt_associative(f)?,

            Node::Float(v) => write!(f, "{v}",)?,
            Node::Variable(name) => write!(f, "{name}")?,
        }
        Ok(())
    }
}

mod shortcuts {
    use crate::Node;

    pub fn literal(value: f32) -> Box<Node> {
        Box::new(Node::Float(value))
    }
    pub fn variable(name: &str) -> Box<Node> {
        Box::new(Node::Variable(name.to_string()))
    }

    #[macro_export]
    macro_rules! addition {
        {$($x:expr),+ $(,)?} => {
            Box::new(Node::addition(vec![$($x, )+]))
        }
    }
}

fn main() -> std::io::Result<()> {
    use shortcuts::*;
    let mut expr = addition! {
        addition!{
            literal(2.0),
            addition!{
                addition!(literal(1.0), literal(1.0)),
                variable("x"),
            }
        },
        variable("x"),
        literal(8.0),
        literal(16.0),
    };
    print!("{expr} = \n\t");

    expr = Box::new(simplify(expr.as_ref()));

    println!("{expr}");
    Ok(())
}
