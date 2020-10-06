use crate::parser::tokeniser::Tokeniser;

pub struct Parser {
    pub source: Tokeniser
}

impl Parser {
    pub fn new (source: String) -> Parser {
        let source = Tokeniser::new(source);
        Parser {
            source
        }
    }
}
