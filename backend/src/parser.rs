// parser.rs
// CruxLang Parser: Converts a stream of tokens into an Abstract Syntax Tree (AST).
// Handles statements, expressions, control flow, functions, classes, and more.

use crate::ast::{Expr, Stmt};
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Create a new parser from a vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parse the entire token stream into a list of statements (AST root).
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        // === Main Parse Loop ===
        // Think of this as reading a script line by line:
        // - We keep asking for the next statement until we reach the end.
        // - Each statement could be a variable, a function, a loop, etc.
        // - We build a list of all the statements, which becomes our program's "recipe" (the AST).
        while !self.is_at_end() {
            statements.push(self.parse_stmt()?); // Parse the next statement and add it to the list
        }
        Ok(statements) // Return the full list of statements (the program)
    }

    /// Parse a single statement from the token stream.
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        let token = self.peek();
        println!("[DEBUG] parse_stmt: token_type = {:?}, lexeme = {:?}", token.token_type, token.lexeme);
        // === Statement Dispatch ===
        // This is like a traffic controller for the parser:
        // - We look at the next token to decide what kind of statement is coming up.
        // - If it's 'let', we know a variable is being declared.
        // - If it's 'if', we know a conditional is coming.
        // - If it's something else, we treat it as an expression.
        match &token.token_type {
            TokenType::Entity => self.parse_entity_stmt(), // Parse an entity (OOP feature)
            TokenType::Def => self.parse_method_stmt(),     // Parse a method definition
            TokenType::Let => self.parse_let_stmt(),       // Parse a variable declaration
            TokenType::Emit => self.parse_emit_stmt(),     // Parse an output statement
            TokenType::Return => self.parse_return_stmt(), // Parse a return statement
            TokenType::While => self.parse_while_stmt(),   // Parse a while loop
            TokenType::If => self.parse_if_stmt(),         // Parse an if/else block
            TokenType::For => self.parse_for_stmt(),       // Parse a for loop
            TokenType::Try => self.parse_try_stmt(),       // Parse a try/catch block
            TokenType::Function  | TokenType::Func => self.parse_function_stmt(), // Parse a function
            TokenType::Class => self.parse_class_stmt(),   // Parse a class definition
            TokenType::ModuleImport => self.parse_module_import_stmt(), // Parse a module import
            _ => self.parse_expr_stmt(),                   // Otherwise, parse as an expression statement
        }
    }

    /// Parse a let statement (variable declaration).
    fn parse_let_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::Let, "Expected 'let'")?;
        let name = self.consume_identifier("Expected variable name")?;
        self.consume(TokenType::Assign, "Expected '=' after variable name")?;
        let expr = self.parse_expr()?;
        self.consume(TokenType::Semicolon, "Expected ';' after let statement")?;
        Ok(Stmt::Let { name, expr, line })
    }

    fn parse_emit_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::Emit, "Expected 'emit'")?;
        let expr = self.parse_expr()?;
        self.consume(TokenType::Semicolon, "Expected ';' after emit statement")?;
        Ok(Stmt::Emit { expr, line })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::Return, "Expected 'return'")?;
        let expr = self.parse_expr()?;
        println!("[DEBUG] AST for return: {:?}", expr);
        println!("[DEBUG] Next token before consuming semicolon in return: {:?}", self.peek());
        self.consume(TokenType::Semicolon, "Expected ';' after return statement")?;
        Ok(Stmt::ReturnStmt(expr, line))
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::While, "Expected 'while'")?;
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body, line })
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::If, "Expected 'if'")?;
        let condition = self.parse_expr()?;
        let then_branch = self.parse_block()?;
        let else_branch = if self.matches(TokenType::Else) {
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Stmt::If { condition, then_branch, else_branch, line })
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::For, "Expected 'for'")?;
        let name = self.consume_identifier("Expected loop variable name")?;
        self.consume(TokenType::In, "Expected 'in'")?;
        let iterable = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::ForLoop { var_name: name, iterable, body, line })
    }

    fn parse_try_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::Try, "Expected 'try'")?;
        let try_block = self.parse_block()?;
        self.consume(TokenType::Catch, "Expected 'catch'")?;
        let _catch_block = self.parse_block()?;
        Ok(Stmt::TryCatch { try_block, catch_block: _catch_block, line })
    }

    fn parse_function_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        if self.check(TokenType::Function) {
            self.consume(TokenType::Function, "Expected 'function'")?;
        } else {
            self.consume(TokenType::Func, "Expected 'func'")?;
        }
        let name = self.consume_identifier("Expected function name")?;
        self.consume(TokenType::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                params.push(self.consume_identifier("Expected parameter name")?);
                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RParen, "Expected ')' after parameters")?;
        println!("Next token before block: {:?}", self.peek());
        let body = self.parse_block()?;
        Ok(Stmt::Function { name, params, body, line })
    }

    fn parse_class_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::Class, "Expected 'class'")?;
        let name = self.consume_identifier("Expected class name")?;
        let mut methods = Vec::new();
        self.consume(TokenType::LBrace, "Expected '{' before class body")?;
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            methods.push(self.parse_function_stmt()?);
        }
        self.consume(TokenType::RBrace, "Expected '}' after class body")?;
        Ok(Stmt::Class { name, methods, line })
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr()?;
        let line = self.peek().line;
        self.consume(TokenType::Semicolon, "Expected ';' after expression statement")?;
        Ok(Stmt::ExprStmt(expr, line))
    }

    fn parse_entity_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::Entity, "Expected 'entity'")?;
        let name = self.consume_identifier("Expected entity name")?;
        let mut methods = Vec::new();
        self.consume(TokenType::LBrace, "Expected '{' before entity body")?;
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            methods.push(self.parse_method_stmt()?);
        }
        self.consume(TokenType::RBrace, "Expected '}' after entity body")?;
        Ok(Stmt::Entity { name, methods, line })
    }

    fn parse_method_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::Def, "Expected 'def'")?;
        let name = self.consume_identifier("Expected method name")?;
        self.consume(TokenType::LParen, "Expected '(' after method name")?;
        let mut params = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                params.push(self.consume_identifier("Expected parameter name")?);
                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RParen, "Expected ')' after parameters")?;
        let body = self.parse_block()?;
        Ok(Stmt::Method { name, params, body, line })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        if self.check(TokenType::DoubleColon) {
            self.advance();
            let module = self.consume_identifier("Expected module name after '::'")?;
            self.consume(TokenType::Dot, "Expected '.' after module name")?;
            let func = self.consume_identifier("Expected function name after '.'")?;
            self.consume(TokenType::LParen, "Expected '(' after function name")?;
            let mut args = Vec::new();
            if !self.check(TokenType::RParen) {
                loop {
                    args.push(self.parse_expr()?);
                    if !self.matches(TokenType::Comma) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RParen, "Expected ')' after arguments")?;
            let line = self.peek().line;
            return Ok(Expr::ModuleCall { module, func, args, line });
        }
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let left = self.parse_binary_expr(0)?;
        if self.check(TokenType::Assign) {
            let assign_token = self.advance();
            let value = self.parse_assignment()?; // right-associative
            let line = assign_token.line;
            // Only allow assignment to variables or index access
            match left {
                Expr::Variable(_, _) | Expr::IndexAccess { .. } => {
                    Ok(Expr::Assign {
                        target: Box::new(left),
                        value: Box::new(value),
                        line,
                    })
                }
                _ => Err(format!("Invalid assignment target at line {}", line)),
            }
        } else {
            Ok(left)
        }
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        loop {
            // Method call: expr.method(...)
            if self.check(TokenType::Dot) {
                let save_pos = self.current;
                self.advance(); // consume '.'
                if self.check(TokenType::Identifier) {
                    let method_token = self.advance();
                    if self.check(TokenType::LParen) {
                        self.advance(); // consume '('
                        let mut args = Vec::new();
                        if !self.check(TokenType::RParen) {
                            loop {
                                args.push(self.parse_expr()?);
                                if !self.matches(TokenType::Comma) {
                                    break;
                                }
                            }
                        }
                        self.consume(TokenType::RParen, "Expected ')' after arguments")?;
                        let line = method_token.line;
                        left = Expr::MethodCall {
                            object: Box::new(left),
                            method: method_token.lexeme.clone().unwrap(),
                            args,
                            line,
                        };
                        continue;
                    } else {
                        // Not a method call, revert
                        self.current = save_pos;
                        break;
                    }
                } else {
                    // Not a method call, revert
                    self.current = save_pos;
                    break;
                }
            }
            // Function call: expr(...)
            if self.check(TokenType::LParen) {
                self.advance(); // consume '('
                let mut args = Vec::new();
                if !self.check(TokenType::RParen) {
                    loop {
                        args.push(self.parse_expr()?);
                        if !self.matches(TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RParen, "Expected ')' after arguments")?;
                let line = self.peek().line;
                left = Expr::Call {
                    callee: Box::new(left),
                    args,
                    line,
                };
                continue;
            }
            // Array indexing: expr[expr]
            if self.check(TokenType::LBracket) {
                self.advance(); // consume '['
                let index = self.parse_expr()?;
                self.consume(TokenType::RBracket, "Expected ']' after index")?;
                let line = self.peek().line;
                left = Expr::IndexAccess {
                    array: Box::new(left),
                    index: Box::new(index),
                    line,
                };
                continue;
            }
            // Binary operators
            let token = self.peek();
            let prec = get_precedence(&token.token_type);
            if prec < min_prec || prec == 0 {
                break;
            }
            let op_token = self.advance();
            let op_str = op_token.lexeme.clone().unwrap_or_else(|| format!("{:?}", op_token.token_type));
            let right = self.parse_binary_expr(prec + 1)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op_str,
                right: Box::new(right),
                line: op_token.line,
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let token = self.advance();
        match &token.token_type {
            TokenType::Identifier => {
                // Object instantiation: Identifier { ... }
                if self.check(TokenType::LBrace) {
                    // Look ahead: is this really an object instantiation?
                    let save_pos = self.current;
                    self.advance(); // consume '{'
                    // If next token is identifier and then colon, it's object instantiation
                    if self.check(TokenType::Identifier) {
                        let save_pos2 = self.current;
                        self.advance();
                        if self.check(TokenType::Colon) {
                            // It's object instantiation, rewind and parse fields
                            self.current = save_pos + 1; // after {
                            let mut fields = Vec::new();
                            if !self.check(TokenType::RBrace) {
                                loop {
                                    let field_name = self.consume_identifier("Expected field name in object instantiation")?;
                                    self.consume(TokenType::Colon, "Expected ':' after field name")?;
                                    let expr = self.parse_expr()?;
                                    fields.push((field_name, expr));
                                    if !self.matches(TokenType::Comma) {
                                        break;
                                    }
                                }
                            }
                            self.consume(TokenType::RBrace, "Expected '}' after object fields")?;
                            let line = token.line;
                            Ok(Expr::ObjectInstance {
                                class: token.lexeme.clone().unwrap(),
                                fields,
                                line,
                            })
                        } else {
                            // Not object instantiation, revert
                            self.current = save_pos;
                            Ok(Expr::Variable(token.lexeme.clone().unwrap(), token.line))
                        }
                    } else {
                        // Not object instantiation, revert
                        self.current = save_pos;
                        Ok(Expr::Variable(token.lexeme.clone().unwrap(), token.line))
                    }
                } else if self.check(TokenType::Dot) {
                    self.advance();
                    let field = self.consume_identifier("Expected field name after '.'")?;
                    let line = token.line;
                    Ok(Expr::FieldAccess {
                        object: Box::new(Expr::Variable(token.lexeme.clone().unwrap(), token.line)),
                        field,
                        line,
                    })
                } else {
                    Ok(Expr::Variable(token.lexeme.clone().unwrap(), token.line))
                }
            }
            TokenType::Not => {
                let expr = self.parse_primary()?;
                Ok(Expr::BinaryOp {
                    left: Box::new(Expr::Boolean(true, token.line)),
                    op: "not".to_string(),
                    right: Box::new(expr),
                    line: token.line,
                })
            }
            TokenType::Number => Ok(Expr::Number(token.lexeme.as_ref().unwrap().parse().unwrap(), token.line)),
            TokenType::String => Ok(Expr::String(token.lexeme.clone().unwrap(), token.line)),
            TokenType::StringStart => {
                // Parse interpolated string: "Hello, $" $ name
                let mut parts = Vec::new();
                let mut current_string = String::new();
                let mut in_interpolation = false;
                
                // Process the string content
                let content = token.lexeme.as_ref().unwrap();
                let mut chars = content.chars().peekable();
                
                while let Some(ch) = chars.next() {
                    if ch == '$' {
                        // End current string part if not empty
                        if !current_string.is_empty() {
                            parts.push(Expr::String(current_string.clone(), token.line));
                            current_string.clear();
                        }
                        in_interpolation = true;
                        break;
                    } else {
                        current_string.push(ch);
                    }
                }
                
                // Add remaining string part if any
                if !current_string.is_empty() {
                    parts.push(Expr::String(current_string, token.line));
                }
                
                // For now, just parse the interpolated expression
                if in_interpolation {
                    let expr = self.parse_expr()?;
                    parts.push(expr);
                }
                
                Ok(Expr::InterpolatedString { parts, line: token.line })
            },
            TokenType::True => Ok(Expr::Boolean(true, token.line)),
            TokenType::False => Ok(Expr::Boolean(false, token.line)),
            TokenType::LParen => {
                let expr = self.parse_expr()?;
                self.consume(TokenType::RParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            TokenType::LBracket => {
                let mut elements = Vec::new();
                if !self.check(TokenType::RBracket) {
                    loop {
                        elements.push(self.parse_expr()?);
                        if !self.matches(TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RBracket, "Expected ']' after array elements")?;
                Ok(Expr::Array(elements, token.line))
            }
            TokenType::SelfKw => Ok(Expr::SelfRef(token.line)),
            _ => Err(format!("Unexpected token {:?} at line {}", token.token_type, token.line))
        }
    }

    // ------- Helper Methods -------
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }

    fn check(&self, kind: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == kind
    }

    fn matches(&mut self, kind: TokenType) -> bool {
        if self.check(kind.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn consume(&mut self, kind: TokenType, msg: &str) -> Result<Token, String> {
        if self.check(kind.clone()) {
            Ok(self.advance())
        } else {
            Err(format!("{} at line {}", msg, self.peek().line))
        }
    }

    fn consume_identifier(&mut self, msg: &str) -> Result<String, String> {
        let token = self.peek().clone();
        if token.token_type == TokenType::Identifier {
            self.advance();
            Ok(token.lexeme.clone().unwrap())
        } else {
            Err(format!("{} at line {}", msg, token.line))
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let token = self.peek();
        println!("[DEBUG] parse_block: token_type = {:?}", token.token_type);
        if token.token_type != TokenType::LBrace {
            panic!("Expected '{{' but got {:?}", token.token_type);
        }
        self.advance();
        let mut statements = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            statements.push(self.parse_stmt()?);
        }
        self.consume(TokenType::RBrace, "Expected '}'")?;
        Ok(statements)
    }

    fn parse_module_import_stmt(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        self.consume(TokenType::ModuleImport, "Expected 'use'")?;
        self.consume(TokenType::DoubleColon, "Expected '::' after 'use'")?;
        let module = self.consume_identifier("Expected module name after '::'")?;
        Ok(Stmt::ModuleImport { module, line })
    }
}

fn get_precedence(token_type: &TokenType) -> u8 {
    match token_type {
        TokenType::Or => 1,
        TokenType::And => 2,
        TokenType::EqEq | TokenType::NotEq => 3,
        TokenType::Less | TokenType::LessEq | TokenType::Greater | TokenType::GreaterEq => 4,
        TokenType::Plus | TokenType::Minus => 5,
        TokenType::Mul | TokenType::Div | TokenType::Mod => 6,
        TokenType::Not => 7, // Highest precedence for unary not
        _ => 0,
    }
}
