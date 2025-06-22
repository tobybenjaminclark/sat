use crate::ast::AST;
use crate::ast::AST::{BiImplication, Conjunction, Disjunction, Implication, Negation, Variable};

/* built on https://www.youtube.com/watch?v=v2uW258qIsM */
pub fn _cnf(node: AST) -> Vec<AST> {
    match node.clone() {
        BiImplication(a, b) => {
            match (*a.clone(), *b.clone()) {
                (Variable(p), Negation(q)) => {
                    let mut clauses: Vec<AST>= vec![];
                    clauses.push(Disjunction(Box::from(Variable(p.clone())), q.clone()));
                    clauses.push(Disjunction(Box::from(Negation(Box::from(Variable(p.clone())))), Box::from(Negation(q.clone()))));
                    clauses
                }
                (Variable(pstr), Disjunction(q, r)) => {
                    let mut p = Variable(pstr.to_string());
                    let mut clauses: Vec<AST>= vec![];
                    clauses.push(Disjunction(Negation(p.clone().into()).into(), Disjunction(q.clone(), r.clone()).into()));
                    clauses.push(Disjunction(Negation(q.clone().into()).into(), p.clone().into()));
                    clauses.push(Disjunction(Negation(r.clone().into()).into(), p.clone().into()));
                    clauses
                }
                (Variable(pstr), Conjunction(q, r)) => {
                    let mut p = Variable(pstr.to_string());
                    let mut clauses: Vec<AST>= vec![];
                    clauses.push(Disjunction(Box::from(p.clone()), Box::from(Disjunction(Negation(q.clone()).into(), Negation(r.clone()).into()))));
                    clauses.push(Disjunction(Negation(p.clone().into()).into(), q.clone()));
                    clauses.push(Disjunction(Negation(p.clone().into()).into(), r.clone()));
                    clauses
                }
                (Variable(pstr), BiImplication(q, r)) => {
                    let mut p = Variable(pstr.to_string());
                    let mut clauses: Vec<AST>= vec![];
                    clauses.push(Disjunction(p.clone().into(), Disjunction(q.clone(), r.clone()).into()));
                    clauses.push(Disjunction(Box::from(p.clone()), Box::from(Disjunction(Negation(q.clone()).into(), Negation(r.clone()).into()))));
                    clauses.push(Disjunction(Box::from(q.clone()), Box::from(Disjunction(Negation(p.clone().into()).into(), Negation(r.clone()).into()))));
                    clauses.push(Disjunction(Box::from(r.clone()), Box::from(Disjunction(Negation(p.clone().into()).into(), Negation(q.clone()).into()))));
                    clauses
                }
                (Variable(pstr), Implication(q, r)) => {
                    let mut p = Variable(pstr.to_string());
                    let mut clauses: Vec<AST> = vec![];
                    clauses.push(Disjunction(Box::from(Negation(Box::from(p.clone()))), Box::from(Disjunction(Negation(q.clone()).into(), r.clone()))));
                    clauses.push(Disjunction(q.clone(), p.clone().into()));
                    clauses.push(Disjunction(Box::from(Negation(r.clone())), p.clone().into()));
                    clauses
                }
                _ => unreachable!()
            }
        }
        Variable(v) => {
            vec![node]
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn cnf(nodes: Vec<AST>) -> Vec<AST> {
    let mut clauses = vec![];
    for node in nodes {
        clauses.extend(_cnf(node));
    }
    clauses
}