#[macro_use]
mod chunk;
mod value;
mod vm;
mod compiler;
mod scanner;

use std::io::{Write, BufRead};

use vm::*;

fn main() {
    let mut vm = VM::new();

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&args[1], &mut vm),
        _ => {
            println!("Usage: rlox [path]");
            std::process::exit(64);
        }
    }
}

fn repl(vm: &mut VM) {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();

        if line.is_empty() {
            println!();
            break;
        }

        let _ = vm.interpret(&line); 
    }
}

fn run_file(path: &str, vm: &mut VM) {
    let source = std::fs::read_to_string(path).unwrap();
    let result = vm.interpret(&source);

    match result {
        Err(InterpretError::CompilerError) => std::process::exit(65),
        Err(InterpretError::RuntimeError) => std::process::exit(70),
        _ => {}
    }
}

