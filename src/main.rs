mod parser;
use crate::parser::parser::Parser;
use crate::parser::parser_debug::*;

fn main() {
    let code = "
4 lol iz 1 2 100
    lool iz 2
brb
".to_string();

    let mut parser = Parser::new(code);
    let ast = parser.read_block();
    print_block(&ast, 0);
}
