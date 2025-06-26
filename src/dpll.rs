use std::collections::{HashMap, HashSet};
use crate::cnf::{Literal, Clause, CNF};

/*
 * Unit Clause is a clause containing a singular unit, that does not occur elsewhere.
 */
fn find_unit_clause(clauses: &CNF, assignment: &HashMap<String, bool>) -> Option<Literal> {
    for clause in clauses {
        match clause.as_slice() {
            [lit @ (iden, _)] if !assignment.contains_key(iden.as_str()) => { return Some(lit.clone()) }
            _ => continue,
        }
    }
    None
}





fn find_pure_literal(clauses: &CNF, assignment: &HashMap<String, bool>) -> Option<Literal> {
    let mut counts = HashMap::new();

    for clause in clauses {
        for (var, sign) in clause {
            if !assignment.contains_key(var) {
                counts.entry(var).or_insert(HashSet::new()).insert(*sign);
            }
        }
    }

    counts.into_iter()
        .find_map(|(var, signs)| {
            if signs.len() == 1 {
                Some((var.clone(), *signs.iter().next().unwrap()))
            } else {
                None
            }
        })
}



fn simplify(clauses: &CNF, assignment: &HashMap<String, bool>) -> CNF {
    let mut simplified = Vec::new();

    'clause_loop: for clause in clauses {
        let mut new_clause = Vec::new();

        for (var, is_pos) in clause {
            match assignment.get(var) {
                Some(val) if *val == *is_pos => continue 'clause_loop,
                Some(_) => continue,
                None => new_clause.push((var.clone(), *is_pos)),
            }
        }

        simplified.push(new_clause);
    }
    simplified
}





pub fn dpll(clauses: &CNF, assignment: &mut HashMap<String, bool>) -> bool {

    /* Simplify Clauses */
    let mut new_clauses = simplify(&clauses, &assignment);

    /* Check if SAT/UNSAT */
    if new_clauses.is_empty() { return true; }                               /* ← No clauses left, we are SAT */
    if new_clauses.iter().any(|c| c.is_empty()) { return false; }   /* ← Empty Clause, we are UNSAT */

    while let Some((iden, value)) = find_unit_clause(&new_clauses, &assignment) {
        println!("Found unit literal: {:?}", iden);
        assignment.insert(iden, value);
        new_clauses = simplify(&new_clauses, &assignment);
        if new_clauses.iter().any(|c| c.is_empty()) { return false; }
    }

    while let Some((iden, value)) = find_pure_literal(&new_clauses, &assignment) {
        println!("Found pure literal: {:?}", iden);
        assignment.insert(iden, value);
        new_clauses = simplify(&new_clauses, &assignment);
        if new_clauses.iter().any(|c| c.is_empty()) { return false; }
    }

    let (iden, value) = match new_clauses.iter().flatten().find(|(v, _)| !assignment.contains_key(v)).cloned() {
        Some(lit) => lit,
        None => return true,
    };

    /* Try assigning the value to be true first? Optimistic! */
    let mut truth_assignments = assignment.clone();
    truth_assignments.insert(iden.clone(), value);
    if dpll(clauses, &mut truth_assignments) {
        *assignment = truth_assignments;
        return true;
    }

    /* Then, try to assign it as false */
    let mut false_assignments = assignment.clone();
    false_assignments.insert(iden.clone(), !value);
    if dpll(clauses, &mut false_assignments) {
        *assignment = false_assignments;
        return true;
    }

    false
}