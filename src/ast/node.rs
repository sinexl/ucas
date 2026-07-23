#[derive(Clone)]
pub enum Node {
    Float(f32),
    Variable(String),

    Addition(Addition),
    Multiplication(Multiplication),
}

#[derive(Clone)]
pub struct Addition {
    pub items: Vec<Node>,
}
#[derive(Clone)]
pub struct Multiplication {
    pub items: Vec<Node>,
}
