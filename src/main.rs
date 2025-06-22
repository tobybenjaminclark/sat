mod ast;
mod tseytin;
mod vargen;

use ast::expr;
use crate::tseytin::tseytin;

fn main() {
    let input = "¬A ∧ (B v C) → D ⇒ E";
    match expr(input) {
        Ok((_rest, ast)) => {
            println!("{:#?}", ast);
            tseytin(ast);
        },
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
