use std::{env, fs};

use crate::parser::parser::Parser;
use crate::parser::parser_debug::*;
use crate::interpreter::interpreter::Interpreter;

mod parser;
mod interpreter;

fn get_env_bool (name: &str) -> bool {
    let val: usize = env::var(name)
        .unwrap_or("0".to_string())
        .parse()
        .expect(&format!("Invalid number in {}, pass 1 or 0", name)[..]);
    val == 1
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Pass an omgrofl file path argument");
    let code = fs::read_to_string(filename)
        .expect("Failed to open code file for reading");

    let dump_ast = get_env_bool("DUMP_AST");
    let dump_vars = get_env_bool("DUMP_VARS");

    let mut parser = Parser::new(code);
    let ast = parser.read_block();

    if dump_ast {
        print_block(&ast, 0);
    }

    let int = Interpreter::create_and_run(ast);

    if dump_vars {
        int.vars.print();
    }
}
