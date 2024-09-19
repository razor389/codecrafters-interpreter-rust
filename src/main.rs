mod scanner;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use env_logger::Env;
use scanner::Scanner;
use token::TokenType;
use env_logger;

fn main() {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

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
                log::info!("Starting to scan tokens in file: {}", filename);
                
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
                        TokenType::SEMICOLON => println!("SEMICOLON ; null"),
                        TokenType::EQUAL => println!("EQUAL = null"),
                        TokenType::EQUAL_EQUAL => println!("EQUAL_EQUAL == null"),
                        TokenType::BANG => println!("BANG ! null"),
                        TokenType::BANG_EQUAL => println!("BANG_EQUAL != null"),
                        TokenType::GREATER => println!("GREATER > null"),
                        TokenType::GREATER_EQUAL => println!("GREATER_EQUAL >= null"),
                        TokenType::LESS => println!("LESS < null"),
                        TokenType::LESS_EQUAL => println!("LESS_EQUAL <= null"),
                        TokenType::SLASH => println!("SLASH / null"),
                        TokenType::STRING => println!("STRING {} {}", token.lexeme, token.literal.clone().unwrap()),
                        TokenType::NUMBER => println!("NUMBER {} {}", token.lexeme, token.literal.clone().unwrap()),
                        // Identifiers and keywords
                        TokenType::IDENTIFIER => println!("IDENTIFIER {} null", token.lexeme),
                        TokenType::AND => println!("AND and null"),
                        TokenType::CLASS => println!("CLASS class null"),
                        TokenType::ELSE => println!("ELSE else null"),
                        TokenType::FALSE => println!("FALSE false null"),
                        TokenType::FOR => println!("FOR for null"),
                        TokenType::FUN => println!("FUN fun null"),
                        TokenType::IF => println!("IF if null"),
                        TokenType::NIL => println!("NIL nil null"),
                        TokenType::OR => println!("OR or null"),
                        TokenType::PRINT => println!("PRINT print null"),
                        TokenType::RETURN => println!("RETURN return null"),
                        TokenType::SUPER => println!("SUPER super null"),
                        TokenType::THIS => println!("THIS this null"),
                        TokenType::TRUE => println!("TRUE true null"),
                        TokenType::VAR => println!("VAR var null"),
                        TokenType::WHILE => println!("WHILE while null"),

                        // End of file
                        TokenType::EOF => println!("EOF  null"),
                    }
                }

                // Check for lexical errors and exit with code 65 if any occurred
                if scanner.has_error() {
                    log::error!("Lexical error detected during scanning.");
                    process::exit(65);
                } else {
                    process::exit(0); // No errors, exit with code 0
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
