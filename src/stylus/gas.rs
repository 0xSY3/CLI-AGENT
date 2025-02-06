use super::StylusError;
use super::GasOptimization;
use regex::Regex;

pub struct GasAnalyzer {
    content: String,
}

impl GasAnalyzer {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn analyze(&self) -> Result<Vec<GasOptimization>, StylusError> {
        let mut optimizations = Vec::new();

        // Check for common gas-intensive patterns
        self.check_storage_patterns(&mut optimizations)?;
        self.check_loop_patterns(&mut optimizations)?;
        self.check_memory_patterns(&mut optimizations)?;
        self.check_redundant_operations(&mut optimizations)?;

        Ok(optimizations)
    }

    fn check_storage_patterns(&self, optimizations: &mut Vec<GasOptimization>) -> Result<(), StylusError> {
        // Pattern for multiple storage reads of same variable
        let storage_read_pattern = Regex::new(r"self\.[a-zA-Z_][a-zA-Z0-9_]*\.get\(&[^)]+\)").map_err(|e| 
            StylusError::GasAnalysisError(format!("Regex error: {}", e)))?;

        for (line_num, line) in self.content.lines().enumerate() {
            let matches: Vec<_> = storage_read_pattern.find_iter(line).collect();
            if matches.len() > 1 {
                optimizations.push(GasOptimization {
                    line: line_num + 1,
                    description: "Multiple storage reads in single function".to_string(),
                    suggestion: "Cache storage values in memory when used multiple times".to_string(),
                    estimated_savings: 2100 * (matches.len() as u64 - 1), // ~2100 gas per SLOAD
                });
            }
        }
        Ok(())
    }

    fn check_loop_patterns(&self, optimizations: &mut Vec<GasOptimization>) -> Result<(), StylusError> {
        // Pattern for unchecked loops with storage operations
        let loop_pattern = Regex::new(r"for\s+[^{]+\{").map_err(|e| 
            StylusError::GasAnalysisError(format!("Regex error: {}", e)))?;

        let storage_in_loop = Regex::new(r"self\.[a-zA-Z_][a-zA-Z0-9_]*\.(get|insert)").map_err(|e|
            StylusError::GasAnalysisError(format!("Regex error: {}", e)))?;

        let mut in_loop = false;
        let mut loop_start_line = 0;
        let mut has_storage_op = false;

        for (line_num, line) in self.content.lines().enumerate() {
            if loop_pattern.is_match(line) {
                in_loop = true;
                loop_start_line = line_num + 1;
                has_storage_op = false;
            } else if in_loop {
                if line.contains("}") {
                    if has_storage_op {
                        optimizations.push(GasOptimization {
                            line: loop_start_line,
                            description: "Unbounded loop with storage operations".to_string(),
                            suggestion: "Add length check and consider caching storage values outside the loop".to_string(),
                            estimated_savings: 5000, // Conservative estimate for loop optimization
                        });
                    }
                    in_loop = false;
                } else if storage_in_loop.is_match(line) {
                    has_storage_op = true;
                }
            }
        }
        Ok(())
    }

    fn check_memory_patterns(&self, optimizations: &mut Vec<GasOptimization>) -> Result<(), StylusError> {
        // Pattern for inefficient memory operations
        let repeated_memory_op = Regex::new(r"let\s+[a-zA-Z_][a-zA-Z0-9_]*\s*=\s*self\.[a-zA-Z_][a-zA-Z0-9_]*\.get").map_err(|e| 
            StylusError::GasAnalysisError(format!("Regex error: {}", e)))?;

        for (line_num, line) in self.content.lines().enumerate() {
            if repeated_memory_op.is_match(line) {
                optimizations.push(GasOptimization {
                    line: line_num + 1,
                    description: "Inefficient memory usage with storage operations".to_string(),
                    suggestion: "Consider using stack variables for intermediate operations".to_string(),
                    estimated_savings: 200,
                });
            }
        }
        Ok(())
    }

    fn check_redundant_operations(&self, optimizations: &mut Vec<GasOptimization>) -> Result<(), StylusError> {
        // Pattern for redundant storage operations
        let storage_write_pattern = Regex::new(r"self\.[a-zA-Z_][a-zA-Z0-9_]*\.insert").map_err(|e| 
            StylusError::GasAnalysisError(format!("Regex error: {}", e)))?;

        let mut current_writes = Vec::new();

        for (line_num, line) in self.content.lines().enumerate() {
            if let Some(write) = storage_write_pattern.find(line) {
                current_writes.push((line_num + 1, write.as_str().to_string()));

                if current_writes.len() > 1 {
                    optimizations.push(GasOptimization {
                        line: line_num + 1,
                        description: "Multiple storage writes without batching".to_string(),
                        suggestion: "Consider batching storage updates or using memory for intermediate values".to_string(),
                        estimated_savings: 5000, // SSTORE cost
                    });
                }
            } else if line.trim().is_empty() {
                current_writes.clear();
            }
        }
        Ok(())
    }
}

pub fn analyze_gas_usage(content: String) -> Result<Vec<GasOptimization>, StylusError> {
    let analyzer = GasAnalyzer::new(content);
    analyzer.analyze()
}