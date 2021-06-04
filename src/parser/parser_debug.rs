use crate::parser::parser_utils::*;

pub fn print_block (ast: &Vec<ASTNode>, indent: usize) {
    for node in ast {
        print_ast_node(node, indent);
    }
}

fn print_ast_node (node: &ASTNode, indent: usize) {
    match node {
        ASTNode::Variable(var_id) => print_at_depth(format!("Variable: #{}", var_id), indent),
        ASTNode::Number(n) => print_at_depth(format!("Number: {}", n), indent),
        ASTNode::VariableDeclaration(var_dec) => {
            print_at_depth(format!("Variable #{} declared as:", var_dec.var_id), indent);
            print_ast_node(&var_dec.value, indent + 1);
        },
        ASTNode::ArglessStatement(stmt) => print_at_depth(format!("Argless statement: {}", get_statement_string(stmt)), indent),
        ASTNode::StatementWithArg(arg_stmt) => {
            print_at_depth(format!("Statement with arg: {}", get_statement_string(&arg_stmt.statement)), indent);
            print_ast_node(&arg_stmt.arg, indent + 1);
        },
        ASTNode::IfDeclaration(if_stmt) => {
            print_at_depth("If declaration:".to_string(), indent);
            print_at_depth("Left:".to_string(), indent + 1);
            print_ast_node(&if_stmt.left, indent + 2);
            print_at_depth(format!("Operator: {}", get_operator_string(&if_stmt.operator)), indent + 1);
            print_at_depth("Right:".to_string(), indent + 1);
            print_ast_node(&if_stmt.right, indent + 2);
            print_at_depth("Body:".to_string(), indent + 1);
            print_block(&if_stmt.body, indent + 2);
        },
        ASTNode::ForLoopDeclaration(for_loop) => {
            print_at_depth(format!("For loop, counter: Var #{}", for_loop.var_id), indent);
            print_at_depth("From:".to_string(), indent + 1);
            print_ast_node(&for_loop.initial_value, indent + 2);
            print_at_depth("To:".to_string(), indent + 1);
            print_ast_node(&for_loop.target_value, indent + 2);
            print_at_depth("Body:".to_string(), indent + 1);
            print_block(&for_loop.body, indent + 2);
        },
        ASTNode::InfiniteLoopDeclaration(body) => {
            print_at_depth("Infinite loop:".to_string(), indent);
            print_block(body, indent + 1)
        }
    }
}

fn get_operator_string(op: &Operator) -> &str {
    match op {
        Operator::Uber => "uber",
        Operator::NopeUber => "nope uber",
        Operator::Liek => "liek",
        Operator::NopeLiek => "nope liek"
    }
}

pub fn get_statement_string(stmt: &Statement) -> &str {
    match stmt {
        Statement::Rofl => "rofl",
        Statement::Lmao => "lmao",
        Statement::Tldr => "tldr",
        Statement::Roflmao => "roflmao",
        Statement::N00b => "n00b",
        Statement::L33t => "l33t",
        Statement::Haxor => "haxor",
        Statement::Stfu => "stfu",
        Statement::Afk => "afk"
     }
}

fn print_at_depth (s: String, indent: usize) {
    let mut str = String::from("");
    for _ in 0..indent * 4 {
        str += &String::from(" ");
    }
    str += &s;
    println!("{}", str);
}
