use std::{io::Write, iter::Once, error::Error};

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
            eprintln!("Error on input '{line}': {err}")
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
        println!("{:?}", tok?);
    }
    Ok(())
}

fn error(line: u32, message: String) -> String { report(line, "".to_string(), message) }
fn report(line: u32, loc: String, message: String) -> String { format!("[line {line}] Error {loc}: {message}") }

type Token = String;
type TokResult = Result<Token, String>;
struct Tokenizer<'a>
{
    source: &'a str,
}
impl<'a> Tokenizer<'a>
{
    fn new(source: &'a str) -> Self { Self {source} }
}
impl<'a> IntoIterator for Tokenizer<'a>
{
    type Item = TokResult;
    type IntoIter = Once<TokResult>;
    fn into_iter(self) -> Self::IntoIter { std::iter::once(Ok(self.source.to_string())) }
}
// impl<'a> Iterator for Tokenizer<'a>
// {
//     type Item = &'a str;
//     fn next(&mut self) -> Option<Self::Item> 
//     {
//         Some(self.source)
//     }
// }