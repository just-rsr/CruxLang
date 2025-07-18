// token.rs
// CruxLang Token Definitions: Enumerates all possible token types and the Token struct.
// Used by the lexer and parser to represent source code elements.

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Identifier,
    Number,
    String,
    True,
    False,
    Boolean,

    // Keywords
    Let,
    Function,
    Func,  // for 'func' keyword
    Class, // for 'class' keyword
    Return,
    Emit,
    If,
    Else,
    While,
    Try,
    Catch,
    For,    // for 'for' loops
    In,     // for 'in' keyword

    // Operators
    Assign,         // =
    Plus,           // +
    Minus,          // -
    Mul,            // *
    Div,            // /
    Mod,            // %
    EqEq,           // ==
    NotEq,          // !=
    Greater,        // >
    GreaterEq,      // >=
    Less,           // <
    LessEq,         // <=
    And,
    Or,
    Not,

    // Punctuation
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    LBracket,       // [
    RBracket,       // ]
    Comma,
    Dot,
    Colon,
    Semicolon,
    Bang,           // !
    Unknown,        // For unrecognized characters
    EOF,            // End of file

    // New token types for OOP and module features
    Entity,      // for 'entity' keyword
    Def,         // for 'def' keyword
    SelfKw,      // for 'self' keyword
    DoubleColon, // ::
    ModuleImport, // for 'use' keyword
    
    // String interpolation tokens
    StringStart,    // Start of interpolated string
    StringEnd,      // End of interpolated string
    Interpolate,    // $ for interpolation
    StringPart,     // Part of interpolated string
}


#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>, // The actual string from the source code
    pub line: usize,           // Line number for error reporting
}
impl Token {
    /// Create a new token with type, lexeme, and line number.
    pub fn new(token_type: TokenType, lexeme: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}