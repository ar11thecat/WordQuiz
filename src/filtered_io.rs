use crate::types::{Command};

use std::io::{self, Write};
use colored::Colorize;

pub fn easy_print(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

pub fn print_cyan(s: &str) {
    println!("{}", s.cyan());
}

pub fn header(s: &str) {
    let total_length: usize = 40;
    
    if s.len() >= total_length {
        println!("{}", s);
        
    } else {
        let left_length: usize = (total_length - s.len() - 2) / 2;
        let right_length: usize = total_length - left_length - s.len() - 2;
        println!("\n{} {} {}",
            "-".repeat(left_length),
            s,
            "-".repeat(right_length)
        );
    }
}

pub fn question(s: &str) {
    print!("\x20> {} ", s);
    io::stdout().flush().unwrap();
}

pub fn warning(s: &str) {
    println!("{}", s.yellow());
}

pub fn new_page() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

pub fn filtered_input() -> Command {
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    
    input = input.trim().to_string();

    // as a io module, filtered_io has no knowledge of the meaning of the commands
    match input.as_str() {
        "-help" | "-skip" | "-quit" => return Command::ChangePhase(input),
        _ => return Command::Normal(input),
    };
}
