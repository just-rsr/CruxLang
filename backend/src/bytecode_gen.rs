// bytecode_gen.rs
// CruxLang Bytecode Generator: Converts AST nodes into bytecode instructions for the VM.
// Handles code generation for statements, expressions, functions, classes, and more.

use crate::{
    ast::{Expr, Stmt},
    bytecode::{Chunk, Instruction},
};

pub struct BytecodeGenerator {
    chunk: Chunk,
    debug: bool, // Enable debug tracing
}
impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            debug: false,
        }
    }

    pub fn with_debug() -> Self {
        Self {
            chunk: Chunk::new(),
            debug: true,
        }
    }

    pub fn into_chunk(self) -> Chunk {
        self.chunk
    }

    fn emit(&mut self, instr: Instruction, line: usize) {
        if self.debug {
            println!("Emit: {:?} @ line {}", instr, line);
        }
        self.chunk.write(instr, line);
    }

    fn emit_jump_placeholder(&mut self, is_conditional: bool, line: usize) -> usize {
        self.chunk.emit_placeholder_jump(is_conditional, line)
    }

    fn patch_jump(&mut self, position: usize, target_offset: usize) {
        self.chunk.patch_jump(position, target_offset);
    }

    fn block_ends_with_return(block: &[Stmt]) -> bool {
        block.last().map(|stmt| matches!(stmt, Stmt::ReturnStmt(_, _))).unwrap_or(false)
    }

    /// Compile a statement into bytecode instructions.
    pub fn compile_stmt(&mut self, stmt: &Stmt) {
        // === Statement Compilation Dispatch ===
        // This is where we turn high-level statements (like 'let', 'if', 'for') into low-level instructions
        // that the virtual machine can understand. 
        match stmt {
            Stmt::Let { name, expr, line } => {
                self.compile_expr(expr);
                self.emit(Instruction::StoreVar(name.clone()), *line);
            }
            Stmt::Emit { expr, line } => {
                self.compile_expr(expr);
                self.emit(Instruction::Emit, *line);
                self.emit(Instruction::Pop, *line); // discard result
            }
            Stmt::ReturnStmt(expr, line) => {
                self.compile_expr(expr);
                self.emit(Instruction::Return, *line);
            }
            Stmt::ExprStmt(expr, line) => {
                self.compile_expr(expr);
                self.emit(Instruction::Pop, *line);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                line,
            } => {
                self.compile_expr(condition);

                let jump_if_false = self.emit_jump_placeholder(true, *line);

                for stmt in then_branch {
                    self.compile_stmt(stmt);
                }

                let end_jump = if else_branch.is_some() {
                    let end_jump = self.emit_jump_placeholder(false, *line);
                    self.patch_jump(jump_if_false, self.chunk.len());
                    if let Some(else_branch) = else_branch {
                        for stmt in else_branch {
                            self.compile_stmt(stmt);
                        }
                    }
                    Some(end_jump)
                } else {
                    self.patch_jump(jump_if_false, self.chunk.len());
                    None
                };

                if let Some(end_jump) = end_jump {
                    self.patch_jump(end_jump, self.chunk.len());
                }
            }
            Stmt::While { condition, body, line } => {
                let loop_start = self.chunk.len();
                self.compile_expr(condition);

                let exit_jump = self.emit_jump_placeholder(true, *line);

                for stmt in body {
                    self.compile_stmt(stmt);
                }

                self.emit(Instruction::Jump(loop_start), *line);
                let body_returns = BytecodeGenerator::block_ends_with_return(body);
                self.patch_jump(exit_jump, self.chunk.len());
                if !body_returns {
                    self.emit(Instruction::Pop, *line);
                }
            }
            Stmt::Function { name, params, body, line } => {
                let mut func_gen = BytecodeGenerator::new();
                for stmt in body {
                    func_gen.compile_stmt(stmt);
                }

                func_gen.emit(Instruction::LoadNumber(0.0), *line);
                func_gen.emit(Instruction::Return, *line);

                let func_chunk = Box::new(func_gen.into_chunk());
                self.emit(
                    Instruction::MakeFunc {
                        name: name.clone(),
                        params: params.clone(),
                        chunk: func_chunk,
                    },
                    *line,
                );
                self.emit(Instruction::StoreVar(name.clone()), *line);
            }
            Stmt::ForLoop { var_name, iterable, body, line } => {
                // Compile the iterable expression
                self.compile_expr(iterable);
                
                // Get iterator from the iterable (assume it's an array)
                self.emit(Instruction::GetIterator, *line);
                
                // Store iterator in a temporary variable
                let iterator_var = format!("__iter_{}", var_name);
                self.emit(Instruction::StoreVar(iterator_var.clone()), *line);
                
                // Loop start
                let loop_start = self.chunk.len();
                
                // Check if iterator has next element
                self.emit(Instruction::LoadVar(iterator_var.clone()), *line);
                self.emit(Instruction::HasNext, *line);
                
                // If no more elements, jump to end
                let exit_jump = self.emit_jump_placeholder(true, *line);
                
                // Get next element and assign to loop variable
                self.emit(Instruction::LoadVar(iterator_var.clone()), *line);
                self.emit(Instruction::GetNext, *line);
                self.emit(Instruction::StoreVar(var_name.clone()), *line);
                
                // Execute loop body
                for stmt in body {
                    self.compile_stmt(stmt);
                }
                
                // Jump back to loop start
                self.emit(Instruction::Jump(loop_start), *line);
                
                // Patch the exit jump
                self.patch_jump(exit_jump, self.chunk.len());
                
                // Clean up: pop the iterator
                self.emit(Instruction::Pop, *line);
            }
            Stmt::TryCatch { try_block, catch_block: _, line: _ } => {
                // TODO: Implement try-catch
                for stmt in try_block {
                    self.compile_stmt(stmt);
                }
                // For now, just execute try block
            }
            Stmt::Entity { name, methods, line } => {
                // Compile methods and store as class
                let mut method_map = Vec::new();
                for method in methods {
                    if let Stmt::Method { name: mname, params, body, line: mline } = method {
                        let mut method_gen = BytecodeGenerator::new();
                        for stmt in body {
                            method_gen.compile_stmt(stmt);
                        }
                        method_gen.emit(Instruction::LoadNumber(0.0), *mline);
                        method_gen.emit(Instruction::Return, *mline);
                        let chunk = Box::new(method_gen.into_chunk());
                        method_map.push((mname.clone(), params.clone(), chunk));
                    }
                }
                self.emit(Instruction::MakeClass {
                    name: name.clone(),
                    methods: method_map,
                }, *line);
            }
            Stmt::Method { .. } => {
                // Methods are handled in Entity
            }
            Stmt::ModuleImport { .. } => {
                // No-op for now (modules handled at runtime)
            }
            // TODO: Stmt::For, Stmt::TryCatch, Stmt::Class support
            _ => {}
        }
    }

    pub fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n, line) => {
                self.emit(Instruction::LoadNumber(*n), *line);
            }
            Expr::String(s, line) => {
                self.emit(Instruction::LoadString(s.clone()), *line);
            }
            Expr::Boolean(b, line) => {
                self.emit(Instruction::LoadBool(*b), *line);
            }
            Expr::Variable(name, line) => {
                self.emit(Instruction::LoadVar(name.clone()), *line);
            }
            Expr::Array(elements, line) => {
                for e in elements {
                    self.compile_expr(e);
                }
                self.emit(Instruction::MakeArray(elements.len()), *line);
            }
            Expr::IndexAccess { array, index, line } => {
                self.compile_expr(array);
                self.compile_expr(index);
                self.emit(Instruction::IndexAccess, *line);
            }
            Expr::BinaryOp { left, op, right, line } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op.as_str() {
                    "and" => self.emit(Instruction::BinaryOp("and".to_string()), *line),
                    "or" => self.emit(Instruction::BinaryOp("or".to_string()), *line),
                    "not" => self.emit(Instruction::Not, *line),
                    _ => self.emit(Instruction::BinaryOp(op.clone()), *line),
                }
            }
            Expr::Call { callee, args, line } => {
                self.compile_expr(callee);
                for arg in args {
                    self.compile_expr(arg);
                }
                self.emit(Instruction::CallFunc("<anon>".to_string(), args.len()), *line);
            }
            Expr::Assign { target, value, line } => {
                // Only support variable assignment for now
                if let Expr::Variable(name, _) = &**target {
                    self.compile_expr(value);
                    self.emit(Instruction::StoreVar(name.clone()), *line);
                } else {
                    panic!("Assignment to non-variable is not supported yet");
                }
            }
            Expr::FieldAccess { object, field, line } => {
                self.compile_expr(object);
                self.emit(Instruction::GetField(field.clone()), *line);
            }
            Expr::ModuleCall { module, func, args, line } => {
                // For now, treat as a special CallFunc
                self.emit(Instruction::ModuleCall {
                    module: module.clone(),
                    func: func.clone(),
                    argc: args.len(),
                }, *line);
                for arg in args {
                    self.compile_expr(arg);
                }
            }
            Expr::SelfRef(line) => {
                self.emit(Instruction::LoadVar("self".to_string()), *line);
            }
            Expr::ObjectInstance { class, fields, line } => {
                for (name, expr) in fields {
                    self.compile_expr(expr);
                    self.emit(Instruction::LoadString(name.clone()), *line);
                }
                self.emit(Instruction::MakeObject {
                    class: class.clone(),
                    field_count: fields.len(),
                }, *line);
            }
            Expr::MethodCall { object, method, args, line } => {
                self.compile_expr(object);
                for arg in args {
                    self.compile_expr(arg);
                }
                self.emit(Instruction::CallMethod(method.clone(), args.len()), *line);
            }
            Expr::InterpolatedString { parts, line } => {
                // Compile all parts (strings and variables)
                for part in parts {
                    self.compile_expr(part);
                }
                // Concatenate all parts
                self.emit(Instruction::ConcatStrings(parts.len()), *line);
            }
            // TODO: Expr::Lambda, Expr::Object, Expr::FieldAccess, Expr::This
            _ => {}
        }
    }
}
