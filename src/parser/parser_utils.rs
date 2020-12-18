pub enum ASTNode {
    Variable(usize),
    Number(u8),
    VariableDeclaration(ASTVariableDeclaration),
    IfDeclaration(ASTIfDeclaration),
    WhileLoopDeclaration(ASTWhileLoopDeclaration),
    ForLoopDeclaration(ASTForLoopDeclaration),
    ArglessStatement(Statement),
    StatementWithArg(ASTStatementWithArg)
}

#[derive(PartialEq)]
pub enum Operator {
    Uber,
    NopeUber,
    Liek,
    NopeLiek
}

pub enum Statement {
    Brb,
    Rofl,
    Lmao,
    Rtfm,
    Tldr,
    Roflmao
}

pub struct ASTVariableDeclaration {
    pub var_id: usize,
    pub value: Box<ASTNode>
}

pub struct ASTIfDeclaration {
    pub left: Box<ASTNode>,
    pub operator: Operator,
    pub right: Box<ASTNode>
}

pub struct ASTWhileLoopDeclaration {
    pub body: Vec<ASTNode>
}

pub struct ASTForLoopDeclaration {
    pub var_id: usize,
    pub initial_value: Box<ASTNode>,
    pub target_value: Box<ASTNode>
}

pub struct ASTStatementWithArg {
    pub statement: Statement,
    pub arg: Box<ASTNode>
}
