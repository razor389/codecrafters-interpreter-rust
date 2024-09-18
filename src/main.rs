// main.rs
mod scanner;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};
use scanner::Scanner;
use token::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                scanner.scan_tokens();

                for token in scanner.get_tokens() {
                    match token.token_type {
                        TokenType::LEFT_PAREN => println!("LEFT_PAREN ( null"),
                        TokenType::RIGHT_PAREN => println!("RIGHT_PAREN ) null"),
                        TokenType::LEFT_BRACE => println!("LEFT_BRACE {{ null"),
                        TokenType::RIGHT_BRACE => println!("RIGHT_BRACE }} null"),
                        TokenType::STAR => println!("STAR * null"),
                        TokenType::DOT => println!("DOT . null"),
                        TokenType::COMMA => println!("COMMA , null"),
                        TokenType::PLUS => println!("PLUS + null"),
                        TokenType::MINUS => println!("MINUS - null"),
                        TokenType::EOF => println!("EOF  null"),
                    }
                }
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        }
    }
}
