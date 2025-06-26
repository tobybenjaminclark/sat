use std::sync::Mutex;
use once_cell::sync::Lazy;

struct VarGen {
    counter: usize,
}

impl VarGen {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn fresh(&mut self) -> String {
        self.counter += 1;
        format!("λ{}ⁱ", self.counter)
    }
}
static GLOBAL_VAR_GEN: Lazy<Mutex<VarGen>> = Lazy::new(|| Mutex::new(VarGen::new()));

pub fn get_fresh_var() -> String {
    let mut gen = GLOBAL_VAR_GEN.lock().unwrap();
    gen.fresh()
}