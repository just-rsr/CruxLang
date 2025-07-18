use axum::{routing::post, Router, Json, serve};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use cruxlang::{
    lexer::Lexer,
    parser::Parser,
    bytecode_gen::BytecodeGenerator,
    vm::VM,
};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[derive(Serialize)]
struct RunResponse {
    output: String,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/run", post(run_handler))
        .layer(cors);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("CruxLang server running at http://{}/run", addr);
    println!("Open index.html in your browser to use the web interface!");
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn run_handler(Json(payload): Json<RunRequest>) -> Json<RunResponse> {
    let (output, error) = run_cruxlang_code(&payload.code);
    Json(RunResponse { output, error })
}

fn run_cruxlang_code(source_code: &str) -> (String, Option<String>) {
    // Lexing
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.tokenize();
    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => return (String::new(), Some(format!("Parse error: {}", e))),
    };
    // Bytecode Generation
    let mut generator = BytecodeGenerator::new();
    for stmt in ast {
        generator.compile_stmt(&stmt);
    }
    let chunk = generator.into_chunk();
    let instructions = chunk.instructions.into_iter().zip(chunk.lines.into_iter()).collect();
    let mut vm = VM::new(instructions);
    // Run the VM and return output
    match vm.run() {
        Ok(()) => (vm.get_output(), None),
        Err(e) => (String::new(), Some(e)),
    }
}
