use crate::token::Token;

#[derive(Clone, Debug)]
pub struct BinaryOperation {
    pub lhs: Box<Expression>,
    pub operator: Token,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Number {
    pub value: i128,
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Boolean {
    pub value: bool,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub value: String,
}

#[derive(Clone, Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    Number(Number),
    Identifier(Identifier),
    Boolean(Boolean),
    Text(Text),
}
