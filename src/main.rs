mod parser;
use crate::parser::tokeniser::Tokeniser;
use crate::parser::parser::Parser;
use crate::parser::char_stream::CharStream;
use crate::parser::tokeniser_utils::stringify_token;

fn main() {
    let code = "lol iz 1".to_string();

    // let mut tokens = Tokeniser::new(code);
    //
    // while !tokens.eof {
    //     println!("{}", stringify_token(tokens.read()));
    // }

    let mut parser = Parser::new(code);
    parser.generate_ast();
}
