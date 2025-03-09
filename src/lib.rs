mod compiler;
mod interpreter;
mod opcode;

#[cfg(test)]
mod tests;

use compiler::Compiler;
use interpreter::Thread;
use std::io::Write;

pub fn lib_main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        eprintln!("Unrecognized command line arguments");
        std::process::exit(1);
    } else if args.len() == 2 {
        let mut compiler = Compiler::new();
        let file_path = args[1].as_str();
        let input = match std::fs::read_to_string(file_path) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error reading file '{file_path}': {e}");
                std::process::exit(1);
            }
        };
        process(&mut compiler, input.as_str());
    } else {
        interactive();
    }
}

pub fn interactive() {
    let mut compiler = Compiler::new();
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
            process(&mut compiler, &input);
        }

        input.clear();
    }
}

fn process(compiler: &mut Compiler, input: &str) {
    match compiler.compile(input, "stdin") {
        Ok(bytecode) => {
            let mut thread = Thread::new(bytecode);
            thread.exec();
            if let Some(val) = thread.return_value() {
                println!("{val}");
            }
        }
        Err(e) => println!("{e}"),
    }
}
