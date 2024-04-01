/*
 *  Copyright 2024 Sebastian Pineda (spineda.wpi.alum@wpi.edu)
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

mod elang_interpreter;
mod keywords;

use elang_interpreter::ElangInterpreter;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();

    let interpreter: ElangInterpreter<'_> = ElangInterpreter::new();

    match (args.len(), args.get(1)) {
        (1, _) => interpreter.launch_interactive_shell(),
        (x, Some(file_name)) if x == 2 => interpreter.interpret_file(file_name)?,
        _ => {
            const ERROR_MESSAGE: &str = "FATAL: Args in unknown state!";
            return Err(Error::new(ErrorKind::InvalidInput, ERROR_MESSAGE));
        }
    }

    Ok(())
}
