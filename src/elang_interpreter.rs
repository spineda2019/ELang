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

use std::io::{stdin, stdout, BufRead, BufReader, Error, Stdin, Stdout, Write};

enum CommunicationProtocol {
    Udp,
    Tcp,
}

enum MessageFormat {
    Raw,
    Scpi,
}

pub struct ElangInterpreter<'a> {
    port: Option<u16>,
    address: Option<&'a str>,
    message_format: Option<MessageFormat>,
    communication_protocol: Option<CommunicationProtocol>,
}

impl<'a> ElangInterpreter<'a> {
    pub fn new() -> Self {
        Self {
            port: None,
            address: None,
            message_format: None,
            communication_protocol: None,
        }
    }

    pub fn launch_interactive_shell(&self) -> Result<(), Error> {
        let mut input: String = String::new();
        let mut stdout: Stdout = stdout();
        let stdin: Stdin = stdin();

        loop {
            input.clear();
            print!("Elang Interpretter >> ");
            stdout.flush()?;
            stdin.read_line(&mut input)?;
            println!("Read: {}", &input);
        }
    }

    pub fn interpret_file(&self, file_name: &str) -> Result<(), Error> {
        let file: std::fs::File = std::fs::File::open(file_name)?;
        let file_reader: BufReader<&std::fs::File> = BufReader::new(&file);

        for line in file_reader.lines() {
            match line {
                Ok(x) => self.interpret_line(&x),
                Err(y) => return Err(y),
            }
        }

        Ok(())
    }

    fn interpret_line(&self, line: &str) {}
}
