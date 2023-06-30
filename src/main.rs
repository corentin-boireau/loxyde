mod token;
mod tokenizer;
mod sloc;
mod error_reporting;

use std::{io::Write, error::Error};
use tokenizer::Tokenizer;

use crate::{sloc::SourceLocation, token::Token};

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
    run(&file_content, Some(file_path)).unwrap_or_else(|err| {
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
        run(&line, None).unwrap_or_else(|err|
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

fn run(source: &str, source_filepath: Option<&str>) -> AnyResult
{
    for tok_info in Tokenizer::new(source)
    {
        if let Some((tok, sloc)) = handle_tok_error(tok_info, source, source_filepath)
        {
            println!("{sloc:?}: {tok:?}");
        }
    }
    Ok(())
}

/// Handles the error contained in tok_info and returns None.
/// Otherwise, i.e. there is no error, returns the content of tok_info.
fn handle_tok_error(tok_info: tokenizer::TokResult, source: &str, source_filepath: Option<&str>) -> Option<(Token, SourceLocation)>
{
    let (tok_res, sloc) = tok_info;

    use tokenizer::TokError::*;
    match tok_res
    {
        Ok(tok) => Some((tok, sloc)),
        Err(e) => 
        { match e {
            UnexpectedCharacter(c) => 
            {
                println!("{}", error_reporting::format_err_message(source, sloc, source_filepath,
                    &format!("error: unexpected character: `{c}`"),
                    "help: you should remove this shit",
                ))
            },
            UnterminatedString => 
            {
                println!("{}", error_reporting::format_err_message(source, SourceLocation{len: 1, ..sloc}, source_filepath,
                    "error: unterminated string",
                    "starting here but never closed",
                ))
                // TODO: Currently error_reporting::format_err_message() doesn't handle multiline error subjects correctly so we manually set the sloc.len to 1.
                // It could be nice to transform the whole subject in a vec of strings representing the actual lines contained in it (or a vec of indices of linebreaks)
            },
            NumberParse(s) => 
            {
                println!("{}", error_reporting::format_err_message(source, sloc, source_filepath,
                    &format!("error: couldn't parse string as number: `{s}`"),
                    "help: supported format is [0-9]*(.[0-9]*)?",
                ))
            },
        }; None },
    }
}