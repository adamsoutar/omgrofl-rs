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
            Token::Keyword(keyword) => {
                let kw = keyword.clone();
                self.read_keyword(&kw)
            },
            Token::Number(n) => {
                if *n == 4 {
                    self.read_for_loop()
                } else {
                    panic!("Unexpected non-4 number: {}", n);
                }
            }
            _ => panic!("Unexpected token: {}", stringify_token(tk))
        }
    }

    fn read_keyword (&mut self, keyword: &String) -> ASTNode {
        match &keyword[..] {
            "wtf" => {
                let left = Box::new(self.read_value_atom());
                self.expect_keyword("iz");
                let operator = self.read_operator();
                let right = Box::new(self.read_value_atom());
                ASTNode::IfDeclaration(
                    ASTIfDeclaration {
                        left, operator, right
                    }
                )
            },
            "brb" => ASTNode::ArglessStatement(Statement::Brb),
            "rtfm" => ASTNode::ArglessStatement(Statement::Rtfm),
            "tldr" => ASTNode::ArglessStatement(Statement::Tldr),
            "rofl" => self.stmt_with_value(Statement::Rofl),
            "lmao" => self.stmt_with_value(Statement::Lmao),
            "roflmao" => self.stmt_with_value(Statement::Roflmao),
            _ => unimplemented!("Keyword \"{}\"", keyword)
        }
    }

    fn stmt_with_value (&mut self, stmt: Statement) -> ASTNode {
        ASTNode::StatementWithArg(ASTStatementWithArg {
            statement: stmt,
            arg: Box::new(self.read_value_atom())
        })
    }

    fn read_for_loop (&mut self) -> ASTNode {
        let var_id = self.expect_variable();
        self.expect_keyword("iz");
        let initial_value = Box::new(self.read_value_atom());
        self.expect_number(2);
        let target_value = Box::new(self.read_value_atom());
        ASTNode::ForLoopDeclaration(ASTForLoopDeclaration {
            var_id,
            initial_value,
            target_value
        })
    }

    fn expect_number (&mut self, check_n: u8) {
        match self.source.read() {
            Token::Number(n) => {
                if *n != check_n {
                    panic!("Expected number {} but got number {}", check_n, n);
                }
            },
            _ => panic!("Expected number {} but got a diff. token", check_n)
        }
    }

    fn expect_variable (&mut self) -> usize {
        match self.source.read() {
            Token::Variable(var_id) => var_id.clone(),
            _ => panic!("Expected a variable but got a diff. token")
        }
    }

    fn read_operator (&mut self) -> Operator {
        let tk = self.source.read();
        if let Token::Keyword(kw) = tk {
            match &kw[..] {
                "uber" => Operator::Uber,
                "liek" => Operator::Liek,
                "nope" => {
                    let op2 = self.read_operator();
                    if op2 == Operator::Uber {
                        Operator::NopeUber
                    } else {
                        Operator::NopeLiek
                    }
                },
                _ => panic!("Expected an operator but got \"{}\"", kw)
            }
        } else {
            panic!("Expected an operator but didn't get one")
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
