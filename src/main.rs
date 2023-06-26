mod token;
mod tokenizer;

use std::{io::Write, error::Error};
use tokenizer::Tokenizer;

type AnyResult = Result<(), Box<dyn Error>>;
fn main() -> AnyResult
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 
    {
        println!("Usage: loxyde [script]");
        std::process::exit(64);
    } 
    else if args.len() == 2
    {
        let script_path = &args[1];
        run_file(script_path)?;
    } 
    else 
    {
        run_prompt();
    }
    Ok(())
}

fn run_file(file_path: &str) -> std::io::Result<()>
{
    let file_content = std::fs::read_to_string(file_path)?;
    run(&file_content).unwrap_or_else(|err| {
        eprintln!("Error while running script {file_path}: {err}");
        std::process::exit(65);
    });

    Ok(())
}

fn run_prompt()
{
    let input = std::io::stdin();
    let mut line = String::new();
    
    print_prompt();
    while input.read_line(&mut line).is_ok()
    { 
        run(&line).unwrap_or_else(|err|
            eprintln!("Error on input: {err}")
        );
        line.clear();
        print_prompt();
    }
}
fn print_prompt()
{
    print!("> ");
    let _ = std::io::stdout().flush();
}

fn run(source: &str) -> AnyResult
{
    for tok in Tokenizer::new(source)
    {
        println!("{:?}", tok);
    }
    Ok(())
}

fn error(line: u32, message: String) -> String { report(line, "".to_string(), message) }
fn report(line: u32, loc: String, message: String) -> String { format!("[line {line}] Error {loc}: {message}") }
