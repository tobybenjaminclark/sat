use std::collections::HashMap;
use crate::ast::AST;
use crate::ast::AST::{BiImplication, Conjunction, Disjunction, Implication, Negation, Variable};
use crate::vargen::get_fresh_var;



pub fn tseytin(formula: AST) -> Vec<AST> {
    let hashmap: &mut HashMap<AST, String> = &mut HashMap::new();
    let toplevel = _tseytin(formula, hashmap);

    let mut clauses: Vec<AST> = vec![];
    for (idx, (k, v)) in hashmap.iter().enumerate() {
        let _ast = BiImplication(Box::from(k.clone()), Box::from(Variable((*v.clone()).parse().unwrap())));
        clauses.push(_ast)
    }

    // fold conjunction over the clauses and return
    let mut fclauses: Vec<AST> = hashmap.iter()
        .map(|(k, v)| BiImplication(Box::new(k.clone()), Box::new(Variable(v.clone()))))
        .collect();

    fclauses.push(toplevel);
    fclauses
}



fn _tseytin(formula: AST, mapping: &mut HashMap<AST, String>) -> AST {
    match formula.clone() {
        Variable(iden)  => {
            formula
        }
        Conjunction(l, r) | Disjunction(l, r) | Implication(l, r) | BiImplication(l, r) => {
            let t_l = _tseytin(*l, mapping);
            let t_r = _tseytin(*r, mapping);
            let variden = get_fresh_var();

            let inner = match formula {
                Conjunction(_, _) => Conjunction(Box::new(t_l), Box::new(t_r)),
                Disjunction(_, _) => Disjunction(Box::new(t_l), Box::new(t_r)),
                Implication(_, _) => Implication(Box::new(t_l), Box::new(t_r)),
                BiImplication(_, _) => BiImplication(Box::new(t_l), Box::new(t_r)),
                _ => unreachable!(),
            };

            mapping.insert(inner, variden.clone());
            Variable(variden)
        }

        Negation(var) => {
            let formula_t = _tseytin(*var, mapping);

            let variden = get_fresh_var();
            mapping.insert(Negation(Box::from(formula_t)), variden.clone());
            return Variable(variden);
        }
    }
}