// Main entry point for the calc expression compiler

mod ast;
mod lexer;
mod parser;
mod sema;
mod codegen;

use clap::Parser;
use inkwell::context::Context;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "calc")]
#[command(about = "calc - the expression compiler", long_about = None)]
struct Args {
    /// Input expression to compile
    #[arg(value_name = "INPUT")]
    input: String,
}

fn run() -> Result<(), String> {
    let args = Args::try_parse().map_err(|e| e.to_string())?;

    // Create lexer
    let lexer = lexer::Lexer::new(&args.input);

    // Create parser
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse().map_err(|e| format!("Syntax error: {}", e))?;

    let ast = ast.ok_or_else(|| "Empty AST".to_string())?;

    // Semantic analysis
    let sema = sema::Sema::new();
    sema.analyze(&ast).map_err(|errors| {
        let error_strings: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
        format!("Semantic errors:\n{}", error_strings.join("\n"))
    })?;

    // Code generation
    let context = Context::create();
    let mut codegen = codegen::CodeGen::new(&context);
    codegen.compile(&ast)?;

    // Print the generated IR
    println!("{}", codegen.print_to_string());

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
