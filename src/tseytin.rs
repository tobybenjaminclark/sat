use std::collections::HashMap;
use crate::ast::AST;
use crate::ast::AST::{Conjunction, Disjunction, Implication, Negation, Variable};
use crate::vargen::get_fresh_var;

pub fn tseytin(formula: AST) -> () {
    let hashmap: &mut HashMap<AST, String> = &mut HashMap::new();
    let toplevel = _tseytin(formula, hashmap);
    for (k, v) in hashmap {
        println!("{:?} : {:?}", k, v);
    }
}

fn _tseytin(formula: AST, mapping: &mut HashMap<AST, String>) -> AST {
    match formula.clone() {
        Variable(iden)  => { formula }
        Conjunction(l, r) | Disjunction(l, r) | Implication(l, r) => {
            let t_l = _tseytin(*l, mapping);
            let t_r = _tseytin(*r, mapping);
            let variden = get_fresh_var();

            let inner = match formula {
                Conjunction(_, _) => AST::Conjunction(Box::new(t_l), Box::new(t_r)),
                Disjunction(_, _) => AST::Disjunction(Box::new(t_l), Box::new(t_r)),
                Implication(_, _) => AST::Implication(Box::new(t_l), Box::new(t_r)),
                _ => unreachable!(),
            };

            mapping.insert(inner, variden.clone());
            Variable(variden)
        }

        Negation(var) => {
            let variden = get_fresh_var();
            mapping.insert(formula, variden.clone());
            return Variable(variden);
        }
    }
}