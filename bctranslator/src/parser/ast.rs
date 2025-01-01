

pub trait Node {}
pub trait Expression: Node {}
pub trait Statement: Node {}

pub struct BinaryExpression {
    left: &'static dyn Expression,
    right: &'static dyn Expression,
    operator: String
}
impl Node for BinaryExpression {}
impl Expression for BinaryExpression {}