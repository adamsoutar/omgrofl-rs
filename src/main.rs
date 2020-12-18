mod parser;
use crate::parser::parser::Parser;
use crate::parser::parser_debug::*;

fn main() {
    let code = "
lol iz 1
wtf lol iz liek 1
    rofl lol
    lmao lol
brb
".to_string();

    let mut parser = Parser::new(code);
    let ast = parser.generate_ast();
    print_ast(&ast);
}
