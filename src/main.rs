mod parser;
mod interpreter;
use crate::parser::parser::Parser;
use crate::parser::parser_debug::*;
use crate::interpreter::interpreter::Interpreter;

fn main() {
    let code = "
    rofl 65
    rofl 10
    lol iz 255
    afk lol
    afk 255
    afk 255
    afk 255
    afk 255
    afk 255
    afk 255
    afk 255
    rofl 66
    rofl 10
".to_string();

    let mut parser = Parser::new(code);
    let ast = parser.read_block();
    print_block(&ast, 0);

    let int = Interpreter::create_and_run(ast);
    // Print out the values of all vars at the end of the program
    int.vars.print();
}
