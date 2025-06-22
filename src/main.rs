mod ast;
mod tseytin;
mod vargen;
mod cnf;

use ast::expr;
use cnf::cnf;
use crate::tseytin::tseytin;

fn main() {
    let input = "¬A ∧ (B v C) → D → E";
    match expr(input) {
        Ok((_rest, ast)) => {
            println!("Parsed Formulae:\n{}\n", ast);

            println!("Tseytin Output:");
            let tast = tseytin(ast);
            for (idx, ast) in tast.clone().iter().enumerate() {
                println!("  [Tseytin Clause {idx}] ‣‣ {ast}");
            }

            println!("\nCNF Clauses:");
            for (idx, ast) in cnf(tast).iter().enumerate() {
                println!("  [CNF Clause {idx}] ‣‣ {ast}");
            }
        },
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
