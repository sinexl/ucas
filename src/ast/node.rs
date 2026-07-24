#[derive(Clone)]
pub enum Node {
    // TODO: For now, f32 are used. In future, i plan to use arbitrary precision rational numbers
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
