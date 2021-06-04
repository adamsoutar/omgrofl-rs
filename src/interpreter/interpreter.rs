use std::thread;
use std::time::Duration;
use std::io::{stdout, Write};
use crate::parser::parser_utils::*;
use crate::parser::parser_debug::*;
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

    fn run_for_loop (&mut self, for_loop: &ASTForLoopDeclaration) -> BlockDecision {
        let counter = for_loop.var_id;

        let init_val = self.get_value(&for_loop.initial_value);
        let target_val = self.get_value(&for_loop.target_value);
        let increasing = target_val > init_val;

        self.vars.set(counter, init_val);

        while self.check_loop_var(counter, target_val, increasing) {
            let bd = self.run_block(&for_loop.body);

            if bd == BlockDecision::Break { break }
            if bd == BlockDecision::Exit { return bd }

            if increasing {
                self.vars.inc(counter);
            } else {
                self.vars.dec(counter);
            }
        }

        BlockDecision::None
    }

    fn check_loop_var (&self, counter: usize, target_val: u8, increasing: bool) -> bool {
        let val = self.vars.get(counter);
        if increasing {
            val <= target_val
        } else {
            val >= target_val
        }
    }

    fn run_infinite_loop (&mut self, body: &Vec<ASTNode>) -> BlockDecision {
        loop {
            let bd = self.run_block(body);
            // This ends here
            if bd == BlockDecision::Break { break }
            // This one is propagated
            if bd == BlockDecision::Exit { return bd }
        }

        BlockDecision::None
    }

    fn run_argless_statement (&mut self, stmt: &Statement) -> BlockDecision {
        match stmt {
            Statement::Tldr => BlockDecision::Break,
            Statement::Stfu => BlockDecision::Exit,
            _ => unimplemented!("Argless statement {}", get_statement_string(stmt))
        }
    }

    fn run_statement_with_arg (&mut self, stmt_with_arg: &ASTStatementWithArg) -> BlockDecision {
        let arg_val = self.get_value(&stmt_with_arg.arg);

        let mut arg_was_a_var = true;
        let arg_as_var_id = match *stmt_with_arg.arg {
            ASTNode::Variable(var_id) => var_id,
            _ => { arg_was_a_var = false; 0 }
        };

        let want_a_var = |for_stmt: &str| {
            if !arg_was_a_var {
                self.croak(format!("{} wants a variable as an arg", for_stmt))
            }
        };

        match stmt_with_arg.statement {
            Statement::Lmao => {
                want_a_var("lmao");
                self.vars.inc(arg_as_var_id);
                BlockDecision::None
            },
            Statement::Roflmao => {
                want_a_var("roflmao");
                self.vars.dec(arg_as_var_id);
                BlockDecision::None
            },
            Statement::N00b => {
                self.vars.staque_push(arg_val);
                BlockDecision::None
            },
            Statement::L33t => {
                want_a_var("l33t");
                self.vars.staque_pop_to_var(arg_as_var_id);
                BlockDecision::None
            },
            Statement::Haxor => {
                want_a_var("haxor");
                self.vars.staque_dequeue_to_var(arg_as_var_id);
                BlockDecision::None
            },
            Statement::Afk => {
                thread::sleep(Duration::from_millis(arg_val as u64));
                BlockDecision::None
            },
            Statement::Rofl => {
                stdout()
                    .write(&[arg_val])
                    .expect("Can't lock standard out");
                BlockDecision::None
            },
            _ => unimplemented!("Statement with arg {}", get_statement_string(&stmt_with_arg.statement))
        }
    }

    fn run_block (&mut self, body: &Vec<ASTNode>) -> BlockDecision {
        for node in body {
            let bd = match node {
                ASTNode::VariableDeclaration(var_dec) => self.run_variable_declaration(var_dec),
                ASTNode::IfDeclaration(if_stmt) => self.run_if_statement(if_stmt),
                ASTNode::ForLoopDeclaration(for_loop) => self.run_for_loop(for_loop),
                ASTNode::ArglessStatement(stmt) => self.run_argless_statement(stmt),
                ASTNode::StatementWithArg(stmt_with_arg) => self.run_statement_with_arg(stmt_with_arg),
                ASTNode::InfiniteLoopDeclaration(body) => self.run_infinite_loop(body),
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
