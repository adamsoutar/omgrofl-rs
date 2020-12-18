mod parser;
mod interpreter;
use crate::parser::parser::Parser;
use crate::parser::parser_debug::*;
use crate::interpreter::interpreter::Interpreter;

fn main() {
    let code = "
4 lol iz 0 2 10
    lool iz lol
    loool iz lol
brb
".to_string();

    let mut parser = Parser::new(code);
    let ast = parser.read_block();
    print_block(&ast, 0);

    let int = Interpreter::create_and_run(ast);
    // Print out the values of all vars at the end of the program
    int.vars.print();
}
