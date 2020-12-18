use crate::parser::parser_utils::*;

#[derive(PartialEq)]
pub enum BlockDecision {
    None,
    Break
}

pub fn test_values (left: u8, operator: Operator, right: u8) -> bool {
    match operator {
        Operator::Liek => left == right,
        Operator::NopeLiek => left != right,
        Operator::Uber => left > right,
        Operator::NopeUber => left <= right
    }
}
