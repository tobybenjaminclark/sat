
pub enum AST {
    Variable(String),
    Conjunction(Box<AST>, Box<AST>),
    Disjunction(Box<AST>, Box<AST>),
    Implication(Box<AST>, Box<AST>),
    Negation(Box<AST>)
}
