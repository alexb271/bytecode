mod compiler;
mod interpreter;
mod opcode;
mod parser;

use compiler::Compiler;
use interpreter::Thread;
use std::io::Write;

pub fn lib_main() {
    let mut input = String::new();
    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        if input.ends_with('\n') {
            input.pop();
        }

        if input.to_lowercase() == "q" {
            break;
        }

        if !input.is_empty() {
            process(&input);
        }

        input.clear();
    }
}

fn process(input: &str) {
    match parser::parse(input) {
        Ok(ast) => {
            let bytecode = Compiler::new().compile(&ast);
            let mut t = Thread::new(bytecode);
            t.exec();
        }
        Err(e) => {
            println!("{e}");
        }
    };
}
