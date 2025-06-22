use crate::ast::AST;
use crate::ast::AST::{BiImplication, Conjunction, Disjunction, Implication, Negation, Variable};

/* called after tseytin */
fn to_cnf(ast: AST) -> Vec<AST> {
    match ast {
        Conjunction(l, r) => {
            let mut left_clauses = to_cnf(*l);
            let mut right_clauses = to_cnf(*r);
            left_clauses.append(&mut right_clauses);
            left_clauses
        }
        Disjunction(l, r) => {
            let left = to_cnf(*l);
            let right = to_cnf(*r);
            // Distribute OR over AND:
            let mut new_clauses = vec![];
            for lc in &left {
                for rc in &right {
                    new_clauses.push(distribute_or(lc.clone(), rc.clone()));
                }
            }
            new_clauses
        }
        Negation(ref inner) => vec![ast],
        Variable(_) => vec![ast],
        _ => vec![ast],
    }
}



fn negate(ast: AST) -> AST {
    match ast {
        Negation(inner) => *inner, /* eliminate !!P */
        Conjunction(l, r) => Disjunction(Box::new(negate(*l)), Box::new(negate(*r))),
        Disjunction(l, r) => Conjunction(Box::new(negate(*l)), Box::new(negate(*r))),
        other => Negation(Box::new(other)),
    }
}

fn distribute_or(a: AST, b: AST) -> AST {
    match (a, b) {
        (Conjunction(l1, r1), b) => {
            Conjunction(
                Box::new(distribute_or(*l1, b.clone())),
                Box::new(distribute_or(*r1, b))
            )
        }
        (a, Conjunction(l2, r2)) => {
            Conjunction(
                Box::new(distribute_or(a.clone(), *l2)),
                Box::new(distribute_or(a, *r2))
            )
        }
        (a, b) => Disjunction(Box::new(a), Box::new(b)),
    }
}

pub fn cnf(astvec: Vec<AST>) -> Vec<AST> {
    let mut clauses = vec![];

    for ast in astvec {
        match ast {
            BiImplication(a, b) => {
                let impl1 = Disjunction(Box::new(negate(*a.clone())), b.clone());
                let impl2 = Disjunction(Box::new(negate(*b)), a);

                clauses.extend(to_cnf(impl1));
                clauses.extend(to_cnf(impl2));
            }
            _ => {
                panic!("something went wrong in the tseytin transform");
            }
        }
    }

    clauses
}
