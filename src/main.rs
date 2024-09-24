mod scanner;
mod token;
mod parser;
mod expr;
mod interpreter;
mod stmt;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use env_logger::Env;
use scanner::Scanner;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize_file(filename),
        "parse" => parse_file(filename),
        "evaluate" => evaluate_file(filename), 
        "run" => run_file(filename), 
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        }
    }
}

fn tokenize_file(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    if !file_contents.is_empty() {
        log::info!("Starting to scan tokens in file: {}", filename);
        let mut scanner = Scanner::new(file_contents);
        scanner.scan_tokens();

        for token in scanner.get_tokens() {
            println!("{}", token);
        }

        if scanner.has_error() {
            log::error!("Lexical error detected during scanning.");
            process::exit(65);
        }else {
            process::exit(0); // No errors, exit with code 0
        }
    } else {
        println!("EOF  null");
    }
}

fn parse_file(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    if !file_contents.is_empty() {
        let mut scanner = Scanner::new(file_contents);
        let _tokens = scanner.scan_tokens();

        if scanner.has_error() {
            process::exit(65);
        }

        let mut parser = Parser::new(scanner.get_tokens().to_vec());
        let expression = parser.parse_expression();

        if let Some(expr) = expression {
            println!("{}", expr);  // Print the AST
        } else {
            eprintln!("Parse error");
            process::exit(65);
        }
    } else {
        println!("EOF  null");
    }
}

fn evaluate_file(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    if !file_contents.is_empty() {
        let mut scanner = Scanner::new(file_contents);
        let _tokens = scanner.scan_tokens();

        if scanner.has_error() {
            process::exit(65); // Exit with 65 for syntax errors
        }

        let mut parser = Parser::new(scanner.get_tokens().to_vec());
        let expression = parser.parse_expression();

        if let Some(expr) = expression {
            let interpreter = Interpreter::new();
            match interpreter.evaluate(&expr) {
                Ok(literal_value) => {
                    // Convert LiteralValue to string for output
                    println!("{}", interpreter.literal_to_string(literal_value));
                }
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(70); // Exit with 70 for runtime errors
                }
            }
        } else {
            eprintln!("Parse error");
            process::exit(65); // Exit with 65 for syntax errors
        }
    } else {
        println!("EOF  null");
    }
}

fn run_file(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    if !file_contents.is_empty() {
        let mut scanner = Scanner::new(file_contents);
        let _tokens = scanner.scan_tokens();

        if scanner.has_error() {
            process::exit(65);
        }

        let mut parser = Parser::new(scanner.get_tokens().to_vec());
        let statements = parser.parse_statements().unwrap();

        let mut interpreter = Interpreter::new();
        if let Err(error) = interpreter.interpret(statements) {
            eprintln!("{}", error);
            process::exit(70); // Runtime error
        }
    } else {
        println!("EOF null");
    }
}