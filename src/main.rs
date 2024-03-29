mod elang_interpreter;

use elang_interpreter::ElangInterpreter;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();

    let interpreter: ElangInterpreter<'_> = ElangInterpreter::new();

    match (args.len(), args.get(1)) {
        (1, _) => interpreter.launch_interactive_shell(),
        (x, Some(file_name)) if x > 1 => interpreter.interpret_file(file_name),
        _ => {
            const ERROR_MESSAGE: &str = "FATAL: Args in unknown state!";
            return Err(Error::new(ErrorKind::InvalidInput, ERROR_MESSAGE));
        }
    }

    Ok(())
}
