// lexer.rs
// CruxLang Lexer: Converts source code into a stream of tokens for parsing.
// Handles identifiers, numbers, strings, operators, and keywords.

use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub file: Option<String>, // optional filename support
}

#[derive(Debug)]
pub struct Lexer<'a> {
    _source: &'a str,
    chars: Peekable<Chars<'a>>,
    current_index: usize,
    current_line: usize,
    current_col: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            _source: source,
            chars: source.chars().peekable(),
            current_index: 0,
            current_line: 1,
            current_col: 1,
        }
    }

    /// Advance to the next character in the input.
    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.current_index += ch.len_utf8();
        // If we see a newline, move to the next line and reset column
        if ch == '\n' {
            self.current_line += 1;
            self.current_col = 1;
        } else {
            self.current_col += 1; // Otherwise, just move to the next column
        }
        Some(ch)
    }

    /// Peek at the next character without consuming it.
    fn peek(&mut self) -> Option<char> {
        // Look ahead at the next character, but don't move forward
        self.chars.peek().copied()
    }

    fn _make_span(&self, _start: usize, _end: usize) -> Span {
        Span {
            start: 0,
            end: 0,
            line: self.current_line,
            column: self.current_col,
            file: None,
        }
    }

    /// Skip whitespace characters.
    fn skip_whitespace(&mut self) {
        // Keep advancing as long as the next character is a space, tab, or newline
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break; // Stop when we hit a non-whitespace character
            }
        }
    }

    /// Read an identifier or keyword token.
    fn read_identifier(&mut self, _start: usize) -> Token {
        let mut value = String::new();
        // Read all letters, digits, or underscores to form the identifier
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                value.push(self.advance().unwrap());
            } else {
                break; // Stop at the first non-identifier character
            }
        }
        // Check if the identifier is a reserved keyword (like 'let', 'if', etc.)
        let token_type = match value.as_str() {
            "let" => TokenType::Let,
            "func" => TokenType::Func,
            "function" => TokenType::Function,
            "class" => TokenType::Class,
            "return" => TokenType::Return,
            "emit" => TokenType::Emit,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            "entity" => TokenType::Entity,
            "def" => TokenType::Def,
            "self" => TokenType::SelfKw,
            "use" => TokenType::ModuleImport,
            _ => TokenType::Identifier, // If not a keyword, it's just a variable name
        };
        Token {
            token_type,
            lexeme: Some(value),
            line: self.current_line,
        }
    }

    /// Read a numeric literal token.
    fn read_number(&mut self, _start: usize) -> Token {
        let mut value = String::new();
        // Read all digits (and possibly a dot for decimals)
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                value.push(self.advance().unwrap());
            } else {
                break; // Stop at the first non-digit
            }
        }
        Token {
            token_type: TokenType::Number,
            lexeme: Some(value),
            line: self.current_line,
        }
    }

    /// Read a string literal or interpolated string token.
    fn read_string(&mut self, _start: usize) -> Token {
        let mut value = String::new();
        let mut terminated = false;
        let mut has_interpolation = false;

        // Read characters until we find the closing quote or reach the end
        while let Some(ch) = self.advance() {
            if ch == '"' {
                terminated = true; // Found the end of the string
                break;
            } else if ch == '$' {
                has_interpolation = true; // Found a $ for interpolation
                value.push(ch);
            } else {
                value.push(ch); // Add character to the string
            }
        }

        // Decide what kind of string token this is
        let token_type = if terminated {
            if has_interpolation {
                TokenType::StringStart // Start of an interpolated string
            } else {
                TokenType::String // Regular string
            }
        } else {
            TokenType::Unknown // Unterminated string (error)
        };
        Token {
            token_type,
            lexeme: Some(value),
            line: self.current_line,
        }
    }

    /// Tokenize the entire source code into a vector of tokens.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        // === Main Tokenization Loop ===
        // This is the heart of the lexer. Imagine reading a book letter by letter:
        // - We skip over spaces (whitespace)
        // - We look at each character and decide what kind of thing it starts (word, number, symbol)
        // - We call a helper to read the whole thing (like reading a whole word, not just a letter)
        // - We turn each thing into a Token, which is like a label for what we found
        while let Some(_ch) = self.peek() {
            self.skip_whitespace(); // Ignore spaces, tabs, and newlines
            let _start = self.current_index;

            let token = match self.peek() {
                // === Identifier or Keyword ===
                // If the next character is a letter or '_', we might be starting a variable name or a keyword (like 'let').
                // We read the whole word and then check if it's a special word (keyword) or just a name (identifier).
                Some(c) if c.is_alphabetic() || c == '_' => self.read_identifier(_start),
                // === Number Literal ===
                // If the next character is a digit, we read all the digits (and maybe a dot) to get the full number.
                Some(c) if c.is_ascii_digit() => self.read_number(_start),
                // === String Literal ===
                // If we see a double quote, we know a string is starting. We read until the closing quote.
                // If we see a $ inside, we know it's an interpolated string (like "Hello, $name").
                Some('"') => {
                    self.advance(); // consume opening "
                    self.read_string(_start)
                }
                // === Multi-character Operators ===
                // Some operators are two characters (like <=, >=, ==, !=). We check for those first.
                Some('<') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token {
                            token_type: TokenType::LessEq,
                            lexeme: Some("<=".to_string()),
                            line: self.current_line,
                        }
                    } else {
                        Token {
                            token_type: TokenType::Less,
                            lexeme: Some("<".to_string()),
                            line: self.current_line,
                        }
                    }
                }
                Some('>') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token {
                            token_type: TokenType::GreaterEq,
                            lexeme: Some(">=".to_string()),
                            line: self.current_line,
                        }
                    } else {
                        Token {
                            token_type: TokenType::Greater,
                            lexeme: Some(">".to_string()),
                            line: self.current_line,
                        }
                    }
                }
                Some('=') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token {
                            token_type: TokenType::EqEq,
                            lexeme: Some("==".to_string()),
                            line: self.current_line,
                        }
                    } else {
                        Token {
                            token_type: TokenType::Assign,
                            lexeme: Some("=".to_string()),
                            line: self.current_line,
                        }
                    }
                }
                Some('!') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token {
                            token_type: TokenType::NotEq,
                            lexeme: Some("!=".to_string()),
                            line: self.current_line,
                        }
                    } else {
                        Token {
                            token_type: TokenType::Bang,
                            lexeme: Some("!".to_string()),
                            line: self.current_line,
                        }
                    }
                }
                Some('/') => {
                    self.advance();
                    if self.peek() == Some('/') {
                        // Single-line comment: skip until end of line
                        self.advance(); // consume second '/'
                        while let Some(ch) = self.peek() {
                            if ch == '\n' {
                                break; // Stop at end of line
                            }
                            self.advance();
                        }
                        // Don't add a token for comments, continue to next iteration
                        continue;
                    } else {
                        Token {
                            token_type: TokenType::Div,
                            lexeme: Some("/".to_string()),
                            line: self.current_line,
                        }
                    }
                }
                // === Single-character Tokens and Unknowns ===
                // For everything else (punctuation, math symbols, or unknowns), we match the character directly.
                // If it's not something we recognize, we mark it as Unknown so the parser can handle errors.
                Some(c) => {
                    self.advance();
                    let token_type = match c {
                        '(' => TokenType::LParen,
                        ')' => TokenType::RParen,
                        '{' => TokenType::LBrace,
                        '}' => TokenType::RBrace,
                        '[' => TokenType::LBracket,
                        ']' => TokenType::RBracket,
                        '+' => TokenType::Plus,
                        '-' => TokenType::Minus,
                        '*' => TokenType::Mul,
                        '%' => TokenType::Mod,
                        ',' => TokenType::Comma,
                        '.' => TokenType::Dot,
                        ':' => {
                            // Check for double colon '::' (used for module access)
                            if self.peek() == Some(':') {
                                self.advance();
                                TokenType::DoubleColon
                            } else {
                                TokenType::Colon
                            }
                        },
                        ';' => TokenType::Semicolon,
                        _ => TokenType::Unknown, // Anything else is unknown
                    };
                    Token {
                        token_type,
                        lexeme: Some(c.to_string()),
                        line: self.current_line,
                    }
                }
                None => break,
            };

            tokens.push(token); // Add the new token to our list
        }

        tokens // Return the full list of tokens
    }
}
