// value.rs

// Only keep Display impl for Value

impl std::fmt::Display for crate::bytecode::Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::bytecode::Value::Null => write!(f, "null"),
            crate::bytecode::Value::Bool(b) => write!(f, "{}", b),
            crate::bytecode::Value::Number(n) => write!(f, "{}", n),
            crate::bytecode::Value::String(s) => write!(f, "\"{}\"", s),
            crate::bytecode::Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", items.join(", "))
            }
            crate::bytecode::Value::Iterator { elements, current_index } => {
                write!(f, "<iterator at {}/{}>", current_index, elements.len())
            }
            crate::bytecode::Value::Function { name, .. } => write!(f, "<function {}>", name),
            crate::bytecode::Value::NativeFunc(_) => write!(f, "<native function>"),
            crate::bytecode::Value::Class { name, .. } => write!(f, "<class {}>", name),
            crate::bytecode::Value::Object { class, .. } => write!(f, "<object of {}>", class),
            crate::bytecode::Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}
