use regex::Regex;

pub struct StylusAnalyzer {
    content: String,
}

impl StylusAnalyzer {
    pub fn from_string(content: String) -> Self {
        Self { content }
    }

    pub fn extract_functions(&self) -> Vec<String> {
        let mut functions = Vec::new();
        let function_pattern = Regex::new(r"pub\s+fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*\)").unwrap();

        for cap in function_pattern.captures_iter(&self.content) {
            functions.push(cap[1].to_string());
        }

        functions
    }

    pub fn extract_state_variables(&self) -> Vec<String> {
        let mut variables = Vec::new();
        let storage_pattern = Regex::new(r"pub\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([a-zA-Z_][a-zA-Z0-9_<>]*)")
            .unwrap();

        for cap in storage_pattern.captures_iter(&self.content) {
            variables.push(format!("{}: {}", &cap[1], &cap[2]));
        }

        variables
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
        let code = String::from("pub balance: uint256");
        let analyzer = StylusAnalyzer::from_string(code);
        let vars = analyzer.extract_state_variables();
        assert!(!vars.is_empty());
        assert_eq!(vars[0], "balance: uint256");
    }
}