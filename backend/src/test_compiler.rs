use cruxlang::{
    lexer::Lexer,
    parser::Parser,
    bytecode_gen::BytecodeGenerator,
    vm::VM,
};

fn main() {
    let source_code = r#"
emit "Testing for loop:";
let arr = [1, 2, 3, 4, 5];
for item in arr {
    emit item;
}
emit "For loop done!";
"#;

    println!("=== Testing CruxLang Compiler (For Loop) ===");
    println!("Source code:");
    println!("{}", source_code);
    println!();

    // Lexing
    println!("=== Lexing ===");
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.tokenize();
    println!("Tokens: {:?}", tokens);
    println!();

    // Parsing
    println!("=== Parsing ===");
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => {
            println!("AST: {:?}", ast);
            ast
        },
        Err(e) => {
            println!("Parse error: {}", e);
            return;
        }
    };
    println!();

    // Bytecode Generation
    println!("=== Bytecode Generation ===");
    let mut generator = BytecodeGenerator::with_debug();
    for stmt in &ast {
        generator.compile_stmt(stmt);
    }
    let chunk = generator.into_chunk();
    println!("Instructions: {:?}", chunk.instructions);
    println!("Lines: {:?}", chunk.lines);
    println!();

    // Save bytecode to file
    println!("=== Saving Bytecode to test.crx ===");
    chunk.save_to_file("test.crx").expect("Failed to save bytecode");
    println!("Saved bytecode to test.crx");
    println!();

    // Load bytecode from file and run
    println!("=== Loading Bytecode from test.crx and Running ===");
    let loaded_chunk = cruxlang::bytecode::Chunk::load_from_file("test.crx").expect("Failed to load bytecode");
    let instructions = loaded_chunk.instructions.into_iter().zip(loaded_chunk.lines.into_iter()).collect();
    let mut vm = VM::new(instructions);
    match vm.run() {
        Ok(()) => {
            println!("Output: {}", vm.get_output());
        },
        Err(e) => {
            println!("Runtime error: {}", e);
        }
    }
} 