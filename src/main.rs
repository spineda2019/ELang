mod elang_interpreter;

use elang_interpreter::ElangInterpreter;
use std::ops::Index;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();

    let interpreter: ElangInterpreter<'_> = ElangInterpreter::new();

    match args.len() {
        1 => todo!(),
        _ => todo!(),
    }
}
