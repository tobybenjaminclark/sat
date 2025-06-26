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
    let input = "A ^ B";
    match expr(input) {
        Ok((_rest, ast)) => {
            println!("Parsed Formulae:\n{}\n", ast);

            println!("Tseytin Output:");
            let tast = tseytin(ast);
            for (idx, ast) in tast.clone().iter().enumerate() {
                println!("  [Tseytin Clause {idx}] ‣‣ {ast}");
            }

            println!("\nCNF Clauses:");
            for (idx, ast) in cnf(tast.clone()).iter().enumerate() {
                println!("  [CNF Clause {}] ‣‣ {:?}", idx, ast);
            }

            let sat_clauses = cnf(tast);

            let mut assignment = HashMap::new();
            if dpll(&sat_clauses, &mut assignment) {
                println!("SATISFIABLE");
                for (var, val) in assignment {
                    println!("  {} = {}", var, val);
                }
            } else {
                println!("UNSATISFIABLE");
            }
        },
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
