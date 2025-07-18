// semantic.rs
use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
    pub line: usize,
}

impl SemanticError {
    pub fn new(msg: impl Into<String>, line: usize) -> Self {
        Self {
            message: msg.into(),
            line,
        }
    }
}

pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, bool>>, // Variable name -> isDeclared
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    fn declare(&mut self, name: &str, line: usize) -> Result<(), SemanticError> {
        let current = self.scopes.last_mut().unwrap();
        if current.contains_key(name) {
            return Err(SemanticError::new(format!("Variable '{}' already declared", name), line));
        }
        current.insert(name.to_string(), true);
        Ok(())
    }

    fn resolve(&self, name: &str, line: usize) -> Result<(), SemanticError> {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return Ok(());
            }
        }
        Err(SemanticError::new(format!("Undefined variable '{}'", name), line))
    }

    pub fn analyze_stmt(&mut self, stmt: &Stmt, _line: usize) -> Result<(), SemanticError> {
        match stmt {
            Stmt::Program { statements, .. } => {
                for s in statements {
                    self.analyze_stmt(s, _line)?;
                }
            }
            Stmt::Let { name, expr, .. } => {
                self.analyze_expr(expr)?;
                self.declare(name, _line)?;
            }
            Stmt::Emit { expr, .. } => {
                self.analyze_expr(expr)?;
            }
            Stmt::ExprStmt(expr, ..) => {
                self.analyze_expr(expr)?;
            }
            Stmt::ReturnStmt(expr, ..) => {
                self.analyze_expr(expr)?;
            }
            Stmt::If { condition, then_branch, else_branch, .. } => {
                self.analyze_expr(condition)?;
                self.begin_scope();
                for stmt in then_branch {
                    self.analyze_stmt(stmt, _line)?;
                }
                self.end_scope();

                if let Some(else_branch) = else_branch {
                    self.begin_scope();
                    for stmt in else_branch {
                        self.analyze_stmt(stmt, _line)?;
                    }
                    self.end_scope();
                }
            }
            Stmt::While { condition, body, .. } => {
                self.analyze_expr(condition)?;
                self.begin_scope();
                for stmt in body {
                    self.analyze_stmt(stmt, _line)?;
                }
                self.end_scope();
            }
            Stmt::ForLoop { var_name, iterable, body, .. } => {
                self.analyze_expr(iterable)?;
                self.begin_scope();
                self.declare(var_name, _line)?;
                for stmt in body {
                    self.analyze_stmt(stmt, _line)?;
                }
                self.end_scope();
            }
            Stmt::TryCatch { try_block, catch_block: _, .. } => {
                for stmt in try_block {
                    self.analyze_stmt(stmt, 0)?;
                }
                // TODO: Implement catch block analysis
            }
            Stmt::Function { .. } | Stmt::Class { .. } => {}
            _ => {}
        }
        Ok(())
    }

    pub fn analyze_expr(&mut self, expr: &Expr) -> Result<(), SemanticError> {
        match expr {
            Expr::Number(..) | Expr::String(..) | Expr::Boolean(..) => Ok(()),
            Expr::Variable(name, ..) => {
                self.resolve(name, 0)
            }
            Expr::Array(elements, ..) => {
                for el in elements {
                    self.analyze_expr(el)?;
                }
                Ok(())
            }
            Expr::IndexAccess { array, index, .. } => {
                self.analyze_expr(array)?;
                self.analyze_expr(index)
            }
            Expr::BinaryOp { left, right, .. } => {
                self.analyze_expr(left)?;
                self.analyze_expr(right)?;
                Ok(())
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.analyze_expr(arg)?;
                }
                Ok(())
            }
            Expr::Function { name, params, body, .. } => {
                self.declare(name, 0)?;
                self.begin_scope();
                for param in params {
                    self.declare(param, 0)?;
                }
                for stmt in body {
                    self.analyze_stmt(stmt, 0)?;
                }
                self.end_scope();
                Ok(())
            }
            Expr::Return(inner, ..) => self.analyze_expr(inner),
            Expr::Assign { target, value, .. } => {
                self.analyze_expr(target)?;
                self.analyze_expr(value)
            }
            _ => Ok(()),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}
