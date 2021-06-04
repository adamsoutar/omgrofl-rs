use crate::parser::tokeniser::Tokeniser;
use crate::parser::tokeniser_utils::*;
use crate::parser::parser_utils::*;
#[allow(unused_imports)]
use crate::parser::tokeniser_debug::*;

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
                if n == 4 {
                    self.read_for_loop()
                } else {
                    let nc = n.clone();
                    self.croak(format!("Unexpected non-4 number: {}", nc));
                    unreachable!();
                }
            }
            _ => {
                let st = stringify_token(&tk);
                self.croak(format!("Unexpected token: {}", st));
                unreachable!();
            }
        }
    }

    fn read_keyword (&mut self, keyword: &String) -> ASTNode {
        match &keyword[..] {
            "wtf" => {
                let left = Box::new(self.read_value_atom());
                self.expect_keyword("iz");
                let operator = self.read_operator();
                let right = Box::new(self.read_value_atom());
                let body = self.read_block();
                ASTNode::IfDeclaration(
                    ASTIfDeclaration {
                        left, operator, right, body
                    }
                )
            },
            "rtfm" => {
                let body = self.read_block();
                ASTNode::InfiniteLoopDeclaration(body)
            },
            "tldr" => ASTNode::ArglessStatement(Statement::Tldr),
            "stfu" => ASTNode::ArglessStatement(Statement::Stfu),
            "rofl" => self.stmt_with_value(Statement::Rofl),
            "lmao" => self.stmt_with_value(Statement::Lmao),
            "roflmao" => self.stmt_with_value(Statement::Roflmao),
            "n00b" => self.stmt_with_value(Statement::N00b),
            "l33t" => self.stmt_with_value(Statement::L33t),
            "haxor" => self.stmt_with_value(Statement::Haxor),
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
        let body = self.read_block();
        ASTNode::ForLoopDeclaration(ASTForLoopDeclaration {
            var_id,
            initial_value,
            target_value,
            body
        })
    }

    fn expect_number (&mut self, check_n: u8) {
        match self.source.read() {
            Token::Number(n) => {
                if n != check_n {
                    let nc = n.clone();
                    self.croak(format!("Expected number {} but got number {}", check_n, nc));
                }
            },
            _ => self.croak(format!("Expected number {} but got a diff. token", check_n))
        }
    }

    fn expect_variable (&mut self) -> usize {
        match self.source.read() {
            Token::Variable(var_id) => var_id.clone(),
            _ => {
                self.croak(format!("Expected a variable but got a diff. token"));
                unreachable!();
            }
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
                _ => {
                    let kwc = kw.clone();
                    self.croak(format!("Expected an operator but got \"{}\"", kwc));
                    // Compiler is not smart enough to tell self.croak always panics
                    unreachable!();
                }
            }
        } else {
            self.croak(format!("Expected an operator but didn't get one"));
            unreachable!();
        }
    }

    fn read_variable_definition (&mut self, var_id: usize) -> ASTNode {
        let tk = self.source.read();
        let next_kw = match tk {
            Token::Keyword(kw) => kw,
            _ => {
                self.croak("Expected \"to\" or \"iz\" keywords but got a different token.".to_string());
                unreachable!();
            }
        };

        if next_kw == "iz" {
            let val = self.read_value_atom();
            ASTNode::VariableDeclaration(ASTVariableDeclaration {
                var_id,
                value: Box::new(val)
            })
        } else if next_kw == "to" {
            self.expect_keyword("/dev/null");
            ASTNode::VariableDeclaration(ASTVariableDeclaration {
                var_id,
                value: Box::new(ASTNode::Number(0))
            })
        } else {
            self.croak(format!("Expected \"to\" or \"iz\" keywords but got \"{}\"", next_kw));
            unreachable!();
        }
    }

    fn read_value_atom (&mut self) -> ASTNode {
        let tk = self.source.read();

        match tk {
            Token::Number(n) => ASTNode::Number(n.clone()),
            Token::Variable(var_id) => ASTNode::Variable(var_id.clone()),
            _ => panic!("Expected a value, got {}", stringify_token(&tk))
        }
    }

    fn expect_keyword (&mut self, keyword: &str) {
        let tk = self.source.read();

        let matches = match tk {
            Token::Keyword(ref word) => *word == keyword.to_string(),
            _ => false
        };

        if !matches {
            let st = stringify_token(&tk);
            self.croak(format!("Expected keyword \"{}\", got {}", keyword, st))
        }
    }

    fn croak(&self, msg: String) {
        self.source.croak(msg)
    }

    pub fn read_block (&mut self) -> Vec<ASTNode> {
        let mut block = vec![];

        while !self.source.eof {
            if let Token::Keyword(kw) = self.source.peek() {
                if kw == "brb" {
                    self.source.read();
                    break;
                }
            }

            block.push(self.read_next());
        }

        block
    }

    pub fn new (source: String) -> Parser {
        let source = Tokeniser::new(source);
        Parser {
            source
        }
    }
}
