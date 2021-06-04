mod parser;
mod interpreter;
use crate::parser::parser::Parser;
use crate::parser::parser_debug::*;
use crate::interpreter::interpreter::Interpreter;

fn main() {
    let code = "
    rtfm
        rofl 65
        4 lol iz 1 2 1
            stfu
        brb
        rofl 66
    brb
".to_string();

    let mut parser = Parser::new(code);
    let ast = parser.read_block();
    print_block(&ast, 0);

    let int = Interpreter::create_and_run(ast);
    // Print out the values of all vars at the end of the program
    int.vars.print();
}
