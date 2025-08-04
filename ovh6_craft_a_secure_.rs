use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Define a struct to represent a script
struct Script {
    name: String,
    commands: Vec<String>,
}

// Define a struct to represent a parser
struct Parser {
    scripts: HashMap<String, Script>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            scripts: HashMap::new(),
        }
    }

    fn parse_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut script_name = "".to_string();
        let mut script_commands = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.splitn(2, ':');
            let command = parts.next().unwrap();
            let argument = parts.next();

            match command.trim() {
                "script" => {
                    script_name = argument.unwrap().trim().to_string();
                }
                "cmd" => {
                    if !script_name.is_empty() {
                        script_commands.push(argument.unwrap().trim().to_string());
                    } else {
                        return Err("Script name is not defined".into());
                    }
                }
                "end" => {
                    if !script_name.is_empty() {
                        self.scripts.insert(script_name.clone(), Script { name: script_name.clone(), commands: script_commands.clone() });
                        script_name.clear();
                        script_commands.clear();
                    }
                }
                _ => return Err("Invalid command".into()),
            }
        }

        Ok(())
    }

    fn execute_script(&self, script_name: &str) -> Result<(), Box<dyn Error>> {
        if let Some(script) = self.scripts.get(script_name) {
            for command in &script.commands {
                println!("Executing command: {}", command);
                // Add your command execution logic here
            }
            Ok(())
        } else {
            Err("Script not found".into())
        }
    }
}

fn main() {
    let mut parser = Parser::new();
    parser.parse_file("example.script").unwrap();

    parser.execute_script("my_script").unwrap();
}