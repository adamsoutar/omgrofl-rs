use crate::parser::tokeniser_utils::*;

pub fn print_token (tk: &Token) {
    match tk {
        Token::NullToken => println!("Token::NullToken (placeholder)"),
        Token::Keyword(kw) => println!("Token::Keyword: {}", kw),
        Token::Number(n) => println!("Token::Number: {}", n),
        Token::Variable(var_id) => println!("Token::Variable: {}", var_id)
    }
}
