// ast.rs
// CruxLang Abstract Syntax Tree (AST): Defines the structure of parsed programs.
// Contains enums and structs for expressions and statements.

// === Expression Enum ===
// Expressions are the "building blocks" of code: numbers, variables, calculations, function calls, etc.
// Anything that produces a value is an expression.
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64, usize), // value, line
    String(String, usize),
    Boolean(bool, usize),
    Variable(String, usize),
    Array(Vec<Expr>, usize),
    IndexAccess {
        array: Box<Expr>,
        index: Box<Expr>,
        line: usize,
    },
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
        line: usize,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        line: usize,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        line: usize,
    },
    Return(Box<Expr>, usize),
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
        line: usize,
    },
    FieldAccess {
        object: Box<Expr>,
        field: String,
        line: usize,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
        line: usize,
    },
    ModuleCall {
        module: String,
        func: String,
        args: Vec<Expr>,
        line: usize,
    },
    SelfRef(usize),
    ObjectInstance {
        class: String,
        fields: Vec<(String, Expr)>,
        line: usize,
    },
    InterpolatedString {
        parts: Vec<Expr>,
        line: usize,
    },
}

// === Statement Enum ===
// Statements are the "actions" of code: variable declarations, loops, ifs, returns, etc.
// They control the flow of the program and may contain expressions inside them.
#[derive(Debug, Clone)]
pub enum Stmt {
    Program {
        statements: Vec<Stmt>,
        line: usize,
    },
    Let {
        name: String,
        expr: Expr,
        line: usize,
    },
    Emit {
        expr: Expr,
        line: usize,
    },
    ForLoop {
        var_name: String,
        iterable: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        line: usize,
    },
    TryCatch {
        try_block: Vec<Stmt>,
        catch_block: Vec<Stmt>,
        line: usize,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        line: usize,
    },
    Class {
        name: String,
        methods: Vec<Stmt>,
        line: usize,
    },
    ExprStmt(Expr, usize),
    ReturnStmt(Expr, usize),
    Entity {
        name: String,
        methods: Vec<Stmt>,
        line: usize,
    },
    Method {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        line: usize,
    },
    ModuleImport {
        module: String,
        line: usize,
    },
}
