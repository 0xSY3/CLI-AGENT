#[derive(Debug)]
pub struct ParsedContract {
    pub functions: Vec<String>,
    pub structs: Vec<String>,
}

impl ParsedContract {
    pub fn new(content: String) -> Self {
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        // Basic parsing of Rust code
        for line in content.lines() {
            let trimmed = line.trim();

            // Function detection
            if trimmed.starts_with("pub fn") || trimmed.starts_with("fn") {
                functions.push(line.to_string());
            }

            // Struct detection
            if trimmed.starts_with("pub struct") || trimmed.starts_with("struct") {
                structs.push(line.to_string());
            }
        }

        Self {
            functions,
            structs,
        }
    }

    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    pub fn struct_count(&self) -> usize {
        self.structs.len()
    }
}