use regex::Regex;
use syn::parse_file;
use quote::ToTokens;
use super::StylusError;

pub struct SolidityComparison {
    pub memory_differences: String,
    pub optimization_suggestions: String,
}

pub struct StylusAnalyzer {
    content: String,
    ast: Option<syn::File>,
}

impl StylusAnalyzer {
    pub fn from_string(content: String) -> Self {
        let ast = parse_file(&content).ok();
        Self { content, ast }
    }

    pub fn extract_functions(&self) -> Vec<String> {
        if let Some(ref ast) = self.ast {
            ast.items
                .iter()
                .filter_map(|item| {
                    if let syn::Item::Fn(func) = item {
                        Some(func.sig.ident.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            let function_pattern = Regex::new(r"pub\s+fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*\)").unwrap();
            function_pattern.captures_iter(&self.content)
                .map(|cap| cap[1].to_string())
                .collect()
        }
    }

    pub fn extract_state_variables(&self) -> Vec<String> {
        if let Some(ref ast) = self.ast {
            ast.items
                .iter()
                .filter_map(|item| {
                    if let syn::Item::Struct(struct_item) = item {
                        Some(struct_item.fields.iter()
                            .filter_map(|field| {
                                Some(field.to_token_stream().to_string())
                            })
                            .collect::<Vec<_>>())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect()
        } else {
            let struct_pattern = Regex::new(r"pub struct ([a-zA-Z_][a-zA-Z0-9_]*)\s*\{([^}]*)\}").unwrap();
            let field_pattern = Regex::new(r"(?m)^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([^,\n]+)").unwrap();

            let mut variables = Vec::new();
            for struct_capture in struct_pattern.captures_iter(&self.content) {
                if let Some(fields) = struct_capture.get(2) {
                    let fields_str = fields.as_str();
                    for field_capture in field_pattern.captures_iter(fields_str) {
                        if let (Some(name), Some(type_)) = (field_capture.get(1), field_capture.get(2)) {
                            variables.push(format!("{}: {}", name.as_str(), type_.as_str().trim()));
                        }
                    }
                }
            }
            variables
        }
    }

    pub fn compare_with_solidity(&self) -> Result<SolidityComparison, StylusError> {
        let memory_differences = format!(
            "• Memory Model: Rust (Stylus) uses a more sophisticated memory model\n\
             • Stack vs Heap: Explicit control in Stylus vs implicit in Solidity\n\
             • Ownership: Rust's ownership rules provide better memory safety\n\
             • References: Explicit borrowing in Stylus vs implicit in Solidity\n\
             • Storage: More granular control over storage access in Stylus\n\
             • Size Limits: 24kb contract size limit in Stylus vs Solidity's larger limit"
        );

        let optimization_suggestions = format!(
            "• Use &str instead of String where possible\n\
             • Pre-allocate vectors with known capacity\n\
             • Implement Copy trait for small types\n\
             • Use references instead of cloning\n\
             • Cache storage reads in memory\n\
             • Batch storage writes\n\
             • Keep contract size under 24kb limit"
        );

        Ok(SolidityComparison {
            memory_differences,
            optimization_suggestions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_functions() {
        let code = String::from("pub fn transfer() {}");
        let analyzer = StylusAnalyzer::from_string(code);
        let functions = analyzer.extract_functions();
        assert!(!functions.is_empty());
        assert_eq!(functions[0], "transfer");
    }

    #[test]
    fn test_analyze_state_variables() {
        let code = String::from("pub struct Token { pub balance: u256 }");
        let analyzer = StylusAnalyzer::from_string(code);
        let vars = analyzer.extract_state_variables();
        assert!(!vars.is_empty());
        assert_eq!(vars[0], "balance: u256");
    }
}