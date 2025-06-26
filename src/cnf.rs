use crate::ast::AST;
use crate::ast::AST::{BiImplication, Conjunction, Disjunction, Implication, Negation, Variable};

pub type Literal = (String, bool);
pub type Clause = Vec<Literal>;
pub type CNF = Vec<Clause>;

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

fn ast_to_literal(ast: &AST) -> Option<Literal> {
    match ast {
        AST::Variable(v) => Some((v.clone(), true)),
        AST::Negation(inner) => match &**inner {
            AST::Variable(v) => Some((v.clone(), false)),
            _ => None,
        },
        _ => None,
    }
}

fn flatten_disjunction(ast: &AST, acc: &mut Clause) {
    match ast {
        AST::Disjunction(l, r) => {
            flatten_disjunction(l, acc);
            flatten_disjunction(r, acc);
        }
        _ => {
            if let Some(lit) = ast_to_literal(ast) {
                acc.push(lit);
            }
        }
    }
}

pub fn convert_ast_cnf(ast_clauses: Vec<AST>) -> CNF {
    ast_clauses
        .into_iter()
        .map(|ast| {
            let mut clause = vec![];
            flatten_disjunction(&ast, &mut clause);
            clause
        })
        .collect()
}

pub fn cnf(nodes: Vec<AST>) -> CNF {
    let mut clauses = vec![];
    for node in nodes {
        clauses.extend(_cnf(node));
    }
    convert_ast_cnf(clauses)
}

