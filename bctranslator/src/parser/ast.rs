

pub trait Visitable {
    fn accept(&mut self) -> Result<Vec<u64>, ()>;
}
pub enum BinaryOperation {

}

pub enum Node {
    Statement(Statement),
    Expression(Expression)
}
pub enum Statement {

}
pub enum Expression {
    Binary()
}
impl Visitable for Node {
    fn accept(&mut self) -> Result<Vec<u64>, ()> {
        todo!()
    }
}
pub struct BinaryExpression {
    left: Node,
    op: BinaryOperation,
    right: Node
}
impl Visitable for BinaryExpression {
    fn accept(&mut self) -> Result<Vec<u64>, ()> {
        let mut l_result = self.left.accept()?;
        let mut r_result = self.right.accept()?;
    }
}
