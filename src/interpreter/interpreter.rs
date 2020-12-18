use crate::parser::parser_utils::*;
use crate::interpreter::variables::Variables;
use crate::interpreter::interpreter_utils::*;

pub struct Interpreter {
    pub vars: Variables
}

impl Interpreter {
    fn get_value (&self, node: &ASTNode) -> u8 {
        match node {
            ASTNode::Number(n) => *n,
            ASTNode::Variable(var_id) => self.vars.get(*var_id),
            _ => {
                self.croak("Tried to get value on a non-number/var node".to_string());
                unreachable!();
            }
        }
    }

    fn run_variable_declaration (&mut self, var_dec: ASTVariableDeclaration) {
        let set_val = self.get_value(&var_dec.value);
        self.vars.set(var_dec.var_id, set_val);
    }

    fn run_block (&mut self, body: Vec<ASTNode>) -> BlockDecision {
        for node in body {
            match node {
                ASTNode::VariableDeclaration(var_dec) => self.run_variable_declaration(var_dec),
                _ => panic!("Unimplemented node")
            }
        }

        BlockDecision::None
    }

    fn croak (&self, msg: String) {
        // TODO: Show what node we panicked on
        panic!("{}", msg);
    }

    pub fn create_and_run (ast: Vec<ASTNode>) -> Interpreter {
        let mut int = Interpreter {
            vars: Variables::new()
        };
        int.run_block(ast);
        int
    }
}
