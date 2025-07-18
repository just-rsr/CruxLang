# CruxLang - A Modern Programming Language

## 🚀 Overview

CruxLang is a modern, interpreted programming language built from scratch in Rust. It features a clean syntax, powerful features, and a complete development environment with a web-based playground.

## ✨ Features

### Core Language Features
- **Variables & Assignment**: `let x = 10;` or `x = 10;` (implicit declaration)
- **Data Types**: Numbers, Strings, Booleans, Arrays, Objects
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `and`, `or`, `not`
- **Control Flow**: `if/else`, `while`, `for` loops
- **Functions**: First-class functions with parameters and return values
- **Arrays**: Dynamic arrays with indexing
- **Output**: `emit` statement for printing

### Advanced Features
- **String Interpolation**: `"Hello, $" $ name`
- **For Loops**: `for i in range(0, 10) { emit i; }`
- **Classes & Objects**: Object-oriented programming support
- **Methods**: Instance and static methods
- **Modules**: Basic module system with `math::sqrt()`
- **Try-Catch**: Exception handling (syntax support)
- **Comments**: Single-line comments with `//`

### Development Environment
- **Web Playground**: React-based online editor
- **HTTP API**: RESTful backend for compilation and execution
- **Real-time Execution**: Instant feedback in the browser
- **Error Handling**: Detailed error messages with line numbers
- **CORS Support**: Ready for production deployment

## 🏗️ Architecture

### Compiler Pipeline
```
Source Code → Lexer → Parser → AST → Bytecode Generator → VM → Output
```

### Components
- **Lexer** (`lexer.rs`): Tokenizes source code
- **Parser** (`parser.rs`): Builds Abstract Syntax Tree
- **AST** (`ast.rs`): Defines language constructs
- **Bytecode Generator** (`bytecode_gen.rs`): Compiles AST to bytecode
- **VM** (`vm.rs`): Executes bytecode instructions
- **HTTP Server** (`main.rs`): Provides API endpoints

## 🚀 Quick Start

### Running Locally

1. **Start the Backend**:
   ```bash
   cd backend
   cargo run
   ```
   Server runs on `http://localhost:3000`

2. **Start the Frontend**:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```
   Playground opens at `http://localhost:5173`

### Testing the API

```powershell
# Test basic arithmetic
Invoke-RestMethod -Uri "http://localhost:3000/run" -Method POST -ContentType "application/json" -Body '{"code": "emit 5 + 3;"}'

# Test variables
Invoke-RestMethod -Uri "http://localhost:3000/run" -Method POST -ContentType "application/json" -Body '{"code": "let x = 10; emit x;"}'

# Test string interpolation
Invoke-RestMethod -Uri "http://localhost:3000/run" -Method POST -ContentType "application/json" -Body '{"code": "let name = \"World\"; emit \"Hello, $\" $ name;"}'

# Test for loops
Invoke-RestMethod -Uri "http://localhost:3000/run" -Method POST -ContentType "application/json" -Body '{"code": "for i in range(0, 3) { emit i; }"}'
```

## 📚 Language Examples

### Basic Syntax
```cruxlang
// Variables
let name = "CruxLang";
let age = 25;
let isActive = true;

// Output
emit "Hello, World!";
emit name;
emit 42;

// String interpolation
emit "Hello, $" $ name;
```

### Control Flow
```cruxlang
// If statements
let x = 10;
if (x > 5) {
    emit "x is greater than 5";
} else {
    emit "x is 5 or less";
}

// While loops
let i = 0;
while (i < 5) {
    emit i;
    i = i + 1;
}

// For loops
for i in range(0, 5) {
    emit i;
}
```

### Functions
```cruxlang
// Function definition
function add(a, b) {
    return a + b;
}

// Function call
let result = add(5, 3);
emit result;

// Recursive function
function factorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

### Arrays
```cruxlang
// Array creation
let numbers = [1, 2, 3, 4, 5];

// Array access
emit numbers[0];  // 1
emit numbers[2];  // 3

// Array iteration
for i in range(0, len(numbers)) {
    emit numbers[i];
}
```

### Classes and Objects
```cruxlang
// Class definition
class Person {
    function greet() {
        emit "Hello!";
    }
    
    function introduce(name) {
        emit "I am " $ name;
    }
}

// Object creation
let person = Person();
person.greet();
person.introduce("Alice");
```

### Modules
```cruxlang
// Using math module
let result = math::sqrt(16);
emit result;  // 4.0

// Range function
for i in range(0, 10) {
    emit i;
}
```

## 🔧 Development

### Project Structure
```
cruxlang/
├── backend/           # Rust compiler and VM
│   ├── src/
│   │   ├── main.rs    # HTTP server
│   │   ├── lexer.rs   # Tokenizer
│   │   ├── parser.rs  # Parser
│   │   ├── ast.rs     # Abstract Syntax Tree
│   │   ├── vm.rs      # Virtual Machine
│   │   └── ...
│   └── Cargo.toml
├── frontend/          # React playground
│   ├── src/
│   │   ├── App.tsx    # Main app
│   │   ├── Playground.tsx
│   │   └── ...
│   └── package.json
├── examples/          # Example programs
├── tests/             # Test files
└── explanation/       # Detailed documentation
```

### Building from Source
```bash
# Backend
cd backend
cargo build --release

# Frontend
cd frontend
npm install
npm run build
```

## 🚀 Deployment

### Railway (Recommended)
1. Connect your GitHub repository to Railway
2. Railway auto-detects Rust and deploys
3. Set start command: `cargo run --release`
4. Get production URL

### Other Platforms
- **Render**: Web service deployment
- **Heroku**: Container deployment
- **VPS**: Manual deployment with Nginx

## 📖 Documentation

See the `explanation/` folder for detailed documentation:
- [Language Specification](explanation/LANGUAGE_SPECIFICATION.md)
- [Implementation Details](explanation/IMPLEMENTATION_DETAILS.md)
- [API Reference](explanation/API_REFERENCE.md)
- [Recent Changes](explanation/RECENT_CHANGES.md)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is open source and available under the MIT License.

## 🎯 Roadmap

### Phase 1 (Completed)
- ✅ Basic language features
- ✅ HTTP API server
- ✅ Web playground
- ✅ String interpolation
- ✅ Range function
- ✅ Class constructors

### Phase 2 (In Progress)
- 🔄 Standard library functions
- 🔄 Try-catch implementation
- 🔄 Module system improvements
- 🔄 Error handling enhancements

### Phase 3 (Planned)
- 📋 Advanced OOP features
- 📋 Type system
- 📋 Performance optimizations
- 📋 Development tools

---

**CruxLang** - Building the future of programming languages, one feature at a time! 🚀
