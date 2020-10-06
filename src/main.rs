mod parser;
use crate::parser::tokeniser::Tokeniser;
use crate::parser::char_stream::CharStream;
use crate::parser::tokeniser_utils::stringify_token;

fn main() {
    let code = "
        lol iz 1
        wtf lool iz liek 2
            rofl gay loool
        brb
    ".to_string();

    let char_stream = CharStream::new(code);
    let mut tokens = Tokeniser::new(char_stream);

    while !tokens.eof {
        println!("{}", stringify_token(tokens.read()));
    }
}
