use crate::parser::tokeniser::Tokeniser;
use crate::parser::tokeniser_utils::*;
use crate::parser::parser_utils::*;

pub struct Parser {
    pub source: Tokeniser
}

impl Parser {
    fn read_next (&mut self) -> ASTNode {
        let tk = self.source.read();

        match tk {
            Token::Variable(var_id) => {
                let vid = var_id.clone();
                self.read_variable_definition(vid)
            },
            _ => panic!("Unexpected token: {}", stringify_token(tk))
        }
    }

    fn read_variable_definition (&mut self, var_id: usize) -> ASTNode {
        self.expect_keyword("iz");
        let val = self.read_value_atom();
        ASTNode::VariableDeclaration(ASTVariableDeclaration {
            var_id,
            value: Box::new(val)
        })
    }

    fn read_value_atom (&mut self) -> ASTNode {
        let tk = self.source.read();

        match tk {
            Token::Number(n) => ASTNode::Number(n.clone()),
            Token::Variable(var_id) => ASTNode::Variable(var_id.clone()),
            _ => panic!("Expected a value, got {}", stringify_token(tk))
        }
    }

    fn expect_keyword (&mut self, keyword: &str) {
        let tk = self.source.read();

        let matches = match tk {
            Token::Keyword(word) => *word == keyword.to_string(),
            _ => false
        };

        if !matches {
            panic!("Expected keyword \"{}\", got {}", keyword, stringify_token(tk))
        }
    }

    pub fn generate_ast (&mut self) -> Vec<ASTNode> {
        let mut ast = vec![];

        while !self.source.eof {
            ast.push(self.read_next())
        }

        ast
    }

    pub fn new (source: String) -> Parser {
        let source = Tokeniser::new(source);
        Parser {
            source
        }
    }
}
