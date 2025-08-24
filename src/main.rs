mod core;

use crate::core::cpu::Cpu;
use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fs;

/// A simple 8086 emulator CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .com program to load
    program_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read the program file into a byte vector
    let program_bytes =
        fs::read(&args.program_path).expect("Could not read program file. Does it exist?");

    // Initialize the CPU and load the program
    let mut cpu = Cpu::new();
    cpu.load_com(&program_bytes, None, None);

    println!("Program loaded. Type 's' to step, 'r' for registers, 'q' to quit.");

    // Set up the interactive line reader
    let mut rl = DefaultEditor::new()?;

    // Main interactive loop
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                _ = rl.add_history_entry(line.as_str());
                match line.trim() {
                    "s" | "step" => {
                        cpu.step();
                    }
                    "r" | "regs" => {
                        println!("{:#?}", cpu.regs);
                    }
                    "q" | "quit" => {
                        println!("Exiting.");
                        break;
                    }
                    _ => {
                        println!("Unknown command. Available: s(tep), r(egs), q(uit)");
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting.");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
