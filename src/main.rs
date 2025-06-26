mod ast;
mod tseytin;
mod vargen;
mod cnf;
mod dpll;

use std::collections::HashMap;
use ast::expr;
use cnf::cnf;
use crate::dpll::{dpll};
use crate::tseytin::tseytin;

fn main() {
    let input = "(A → B) ∧ ¬B";
    match expr(input) {
        Ok((_rest, ast)) => {
            println!("Parsed Formulae:\n{}\n", ast);

            println!("Tseytin Output:");
            let tast = tseytin(ast);
            for (idx, ast) in tast.clone().iter().enumerate() {
                println!("   [Tseytin Clause {idx}] ‣‣ {ast}");
            }

            println!("\nCNF Clauses:");
            for (idx, clause) in cnf(tast.clone()).iter().enumerate() {
                println!("   [Clause {}] ‣‣ ({})", idx, clause.iter().map(|(v, s)| if *s { v.clone() } else { format!("¬{}", v) }).collect::<Vec<_>>().join(" ∨ "));
            }

            let sat_clauses = cnf(tast);

            let mut assignment = HashMap::new();
            if dpll(&sat_clauses, &mut assignment) {
                println!("\nSolver is SAT");
                for (var, val) in assignment {
                    if var.starts_with("λ") { continue; }
                    println!("   ‣‣ {} as {}", var, if val {"⊤/True"} else {"⊥/False"});
                }
            } else {
                println!("Solver is UNSAT");
            }
        },
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
