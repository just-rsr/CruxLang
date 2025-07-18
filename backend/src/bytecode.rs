// bytecode.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    // === Literal Values ===
    LoadNumber(f64),
    LoadString(String),
    LoadBool(bool),

    // === Variables ===
    LoadVar(String),
    StoreVar(String),

    // === Expressions ===
    BinaryOp(String),    // e.g., "+", "-", "*", "/", "%", "==", "!=" etc.
    Not,                 // Logical NOT

    // === Arrays ===
    MakeArray(usize),    // Create array with N elements
    IndexAccess,         // arr[index]

    // === Control Flow ===
    JumpIfFalse(usize),  // Jump if condition is false
    Jump(usize),         // Unconditional jump
    Break(usize),        // Break out of loop (jump to end)

    // === Iteration ===
    GetIterator,         // Get iterator from array
    HasNext,             // Check if iterator has next element
    GetNext,             // Get next element from iterator

    // === Functions ===
    CallFunc(String, usize), // Call function by name with N args
    MakeFunc {
        name: String,
        params: Vec<String>,
        chunk: Box<Chunk>,
    },
    LoadNil,

    // === Stack Management ===
    Pop,                 // Discard top of stack
    Return,              // Return from a function

    // === Built-in Output ===
    Emit,                // Print top of stack
    ConcatStrings(usize), // Concatenate N strings from stack

    // === OOP and Modules ===
    MakeClass {
        name: String,
        methods: Vec<(String, Vec<String>, Box<Chunk>)>,
    },
    MakeObject {
        class: String,
        field_count: usize,
    },
    GetField(String),
    CallMethod(String, usize),
    ModuleCall {
        module: String,
        func: String,
        argc: usize,
    },
    ModuleImport {
        module: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    #[serde(skip)]
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<(Instruction, usize)>,
    },
    #[serde(skip)]
    NativeFunc(fn(Vec<Value>) -> Result<Value, String>),
    #[serde(skip)]
    Iterator {
        elements: Vec<Value>,
        current_index: usize,
    },
    Class {
        name: String,
        methods: std::collections::HashMap<String, (Vec<String>, Vec<(Instruction, usize)>)>,
    },
    Object {
        class: String,
        fields: std::collections::HashMap<String, Value>,
    },
    Null,
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub lines: Vec<usize>, // Line number for each instruction
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// Add instruction with its source code line
    pub fn write(&mut self, instruction: Instruction, line: usize) {
        self.instructions.push(instruction);
        self.lines.push(line);
    }

    /// Shorthand to emit and return index
    pub fn emit_with_line(&mut self, instruction: Instruction, line: usize) -> usize {
        let pos = self.instructions.len();
        self.write(instruction, line);
        pos
    }

    /// Emit placeholder for future jump (offset will be patched)
    pub fn emit_placeholder_jump(&mut self, is_conditional: bool, line: usize) -> usize {
        let pos = self.instructions.len();
        let placeholder = if is_conditional {
            Instruction::JumpIfFalse(0)
        } else {
            Instruction::Jump(0)
        };
        self.write(placeholder, line);
        pos
    }

    /// Patch a previously emitted jump to new target offset
    pub fn patch_jump(&mut self, jump_pos: usize, target_offset: usize) {
        if let Some(instr) = self.instructions.get_mut(jump_pos) {
            match instr {
                Instruction::Jump(ref mut offset) | Instruction::JumpIfFalse(ref mut offset) => {
                    *offset = target_offset;
                }
                _ => panic!("Expected jump instruction at patch position"),
            }
        } else {
            panic!("Invalid jump patch position: {}", jump_pos);
        }
    }

    /// Read-only access
    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    pub fn line(&self, index: usize) -> Option<usize> {
        self.lines.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let bytes = bincode::serialize(self).unwrap();
        std::fs::write(path, bytes)
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        let chunk: Chunk = bincode::deserialize(&bytes).unwrap();
        Ok(chunk)
    }
}
