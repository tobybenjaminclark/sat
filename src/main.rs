mod ast;
use ast::expr;

fn main() {
    let input = "¬A ∧ (B v C) → D ⇒ E";
    match expr(input) {
        Ok((_rest, ast)) => println!("{:#?}", ast),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
