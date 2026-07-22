use std::fmt::{Display, Formatter};

#[derive(Clone)]
enum Node {
    Float(f32),
    Variable(String),

    Addition(Addition),
    Multiplication(Multiplication),
}

#[derive(Clone)]
struct Addition {
    items: Vec<Node>,
}

#[derive(Clone)]
struct Multiplication {
    items: Vec<Node>,
}

trait AssociativeList {
    fn symbol(&self) -> &'static str;
    fn from_items(items: &[Node]) -> Node;
    fn items(&self) -> &[Node];

    fn flatten_associative(&self) -> Vec<Node>;

    fn simplify_associative(&self) -> Node;

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
}

impl AssociativeList for Addition {
    fn symbol(&self) -> &'static str {
        "+"
    }

    fn from_items(items: &[Node]) -> Node {
        Node::addition(items.to_vec())
    }

    fn items(&self) -> &[Node] {
        self.items.as_slice()
    }

    fn flatten_associative(&self) -> Vec<Node> {
        let mut new_list: Vec<Node> = Vec::new();
        for i in &self.items {
            match i {
                Node::Addition(other) => {
                    let mut other = other.flatten_associative();
                    new_list.append(&mut other);
                }
                Node::Float(_) | Node::Variable(_) | Node::Multiplication(_) => {
                    new_list.push(i.clone());
                }
            }
        }
        new_list
    }

    fn simplify_associative(&self) -> Node {
        let mut items = self.flatten_associative().to_vec();
        for i in 0..items.len() {
            items[i] = simplify(&items[i]);
        }

        for i in 0..items.len() {
            let mut j = i + 1;
            while j < items.len() {
                let mut removed = false;
                match (&items[i], &items[j]) {
                    (Node::Float(a), Node::Float(b)) => {
                        items[i] = Node::Float(a + b);
                        items.remove(j);
                        removed = true;
                    }
                    (
                        Node::Variable(this_name),
                        Node::Multiplication(Multiplication {
                            items: multiplicands,
                        }),
                    ) => {
                        // TODO: Ensure that items.len() >= 2 for Addition and multiplication
                        if let Node::Variable(other_name) = &multiplicands[1]
                            && *other_name == *this_name
                        {
                            // x + nx = ((1 + n)*x)
                            items[i] = Node::multiplication(vec![
                                simplify(&addition![Node::Float(1.0), multiplicands[0].clone()]),
                                Node::Variable(this_name.clone()),
                            ]);

                            items.remove(j);
                            removed = true;
                        }
                    }
                    _ => {}
                }
                if !removed {
                    j += 1;
                }
            }
        }

        Self::from_items(items.as_slice())
    }
}

impl AssociativeList for Multiplication {
    fn symbol(&self) -> &'static str {
        "*"
    }

    fn from_items(items: &[Node]) -> Node {
        Node::Multiplication(Multiplication {
            items: items.to_vec(),
        })
    }

    fn items(&self) -> &[Node] {
        self.items.as_slice()
    }

    fn flatten_associative(&self) -> Vec<Node> {
        // todo!()
        self.items.clone()
    }

    fn simplify_associative(&self) -> Node {
        // todo!()
        Node::Multiplication(self.clone())
    }
}

impl Node {
    fn addition(items: Vec<Node>) -> Node {
        Node::Addition(Addition { items })
    }
    fn multiplication(items: Vec<Node>) -> Node {
        Node::Multiplication(Multiplication { items })
    }
}

fn simplify(expr: &Node) -> Node {
    match expr {
        Node::Addition(addition) => addition.simplify_associative(),
        Node::Multiplication(multiplication) => multiplication.simplify_associative(),
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
            Node::addition(vec![$($x, )+])
        }
    }

    #[macro_export]
    macro_rules! multiplication {
        {$($x:expr),+ $(,)?} => {
            Node::multiplication(vec![$($x, )+])
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
