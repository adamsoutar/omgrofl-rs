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

    fn run_variable_declaration (&mut self, var_dec: &ASTVariableDeclaration) -> BlockDecision {
        let set_val = self.get_value(&var_dec.value);
        self.vars.set(var_dec.var_id, set_val);
        BlockDecision::None
    }

    fn run_if_statement (&mut self, if_stmt: &ASTIfDeclaration) -> BlockDecision {
        let left_val = self.get_value(&if_stmt.left);
        let right_val = self.get_value(&if_stmt.right);
        let should_run_body = test_values(left_val, &if_stmt.operator, right_val);

        if should_run_body {
            self.run_block(&if_stmt.body)
        } else {
            BlockDecision::None
        }
    }

    // TODO: Loop should run once if init and target are the same
    // TODO: Check if it's inclusive or exclusive
    fn run_for_loop (&mut self, for_loop: &ASTForLoopDeclaration) -> BlockDecision {
        let counter = for_loop.var_id;

        let init_val = self.get_value(&for_loop.initial_value);
        let target_val = self.get_value(&for_loop.target_value);
        let increasing = target_val > init_val;

        self.vars.set(counter, init_val);

        while self.vars.get(counter) != target_val {
            let bd = self.run_block(&for_loop.body);

            if bd == BlockDecision::Break {
                break
            }

            if increasing {
                self.vars.inc(counter);
            } else {
                self.vars.dec(counter);
            }
        }

        BlockDecision::None
    }

    fn run_argless_statement (&mut self, stmt: &Statement) -> BlockDecision {
        match stmt {
            Statement::Tldr => BlockDecision::Break,
            _ => unimplemented!("Argless statement")
        }
    }

    fn run_block (&mut self, body: &Vec<ASTNode>) -> BlockDecision {
        for node in body {
            let bd = match node {
                ASTNode::VariableDeclaration(var_dec) => self.run_variable_declaration(var_dec),
                ASTNode::IfDeclaration(if_stmt) => self.run_if_statement(if_stmt),
                ASTNode::ForLoopDeclaration(for_loop) => self.run_for_loop(for_loop),
                ASTNode::ArglessStatement(stmt) => self.run_argless_statement(stmt),
                _ => unimplemented!("AST node")
            };

            // For example, so we can Break from within an if statement
            if bd != BlockDecision::None {
                return bd;
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
        int.run_block(&ast);
        int
    }
}
