use super::StylusError;
use regex::Regex;

#[derive(Debug)]
pub struct GasOptimization {
    pub line: usize,
    pub description: String,
    pub suggestion: String,
    pub estimated_savings: u64,
    pub category: OptimizationCategory,
}

#[derive(Debug, Clone)]
pub enum OptimizationCategory {
    Storage,
    Memory,
}

#[derive(Debug)]
pub struct MemoryAnalysis {
    pub line: usize,
    pub description: String,
    pub suggestion: String,
    pub pattern_type: MemoryPatternType,
}

#[derive(Debug, Clone)]
pub enum MemoryPatternType {
    AllocationIntensive,
    NestedIterations,
    TemporaryAllocations,
    UnoptimizedCaching,
}

pub struct GasAnalyzer {
    content: String,
}

impl GasAnalyzer {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn analyze(&self) -> Result<Vec<GasOptimization>, StylusError> {
        let mut optimizations = Vec::new();
        self.check_storage_patterns(&mut optimizations)?;
        self.check_memory_patterns(&mut optimizations)?;
        Ok(optimizations)
    }

    fn check_storage_patterns(&self, optimizations: &mut Vec<GasOptimization>) -> Result<(), StylusError> {
        let patterns = [
            (r"self\.balances\.get\([^)]*\).*self\.balances\.get\([^)]*\)", 
             "Multiple storage reads of balances map",
             "Cache balance values in local variables when accessed multiple times",
             OptimizationCategory::Storage),

            (r"self\.holders\.get\([^)]*\).*self\.holders\.get\([^)]*\)", 
             "Multiple reads from holders vector",
             "Cache holder addresses in memory when used multiple times",
             OptimizationCategory::Storage),

            (r"for.*in.*0.*\.\..*self\.(holders|balances)\.get\([^)]*\)", 
             "Storage reads in loop",
             "Batch storage reads before loop iteration to reduce gas costs",
             OptimizationCategory::Storage),
        ];

        for (line_num, line) in self.content.lines().enumerate() {
            for &(pattern, desc, suggestion, ref category) in &patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(line) {
                        optimizations.push(GasOptimization {
                            line: line_num + 1,
                            description: desc.to_string(),
                            suggestion: suggestion.to_string(),
                            estimated_savings: 5000,
                            category: category.clone(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn check_memory_patterns(&self, optimizations: &mut Vec<GasOptimization>) -> Result<(), StylusError> {
        let patterns = [
            (r"let\s+mut\s+balances\s*=\s*Vec::new\(\)", 
             "Unallocated vector initialization for balances",
             "Pre-allocate vector with estimated holder count: Vec::with_capacity(holders.len())",
             OptimizationCategory::Memory),

            (r"\.clone\(\)|\.cloned\(\)", 
             "Unnecessary cloning of data",
             "Use references instead of cloning where possible",
             OptimizationCategory::Memory),

            (r"push\([^)]*\)", 
             "Vector pushing without pre-allocation",
             "Pre-allocate vector capacity to avoid reallocation costs",
             OptimizationCategory::Memory),
        ];

        for (line_num, line) in self.content.lines().enumerate() {
            for &(pattern, desc, suggestion, ref category) in &patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(line) {
                        optimizations.push(GasOptimization {
                            line: line_num + 1,
                            description: desc.to_string(),
                            suggestion: suggestion.to_string(),
                            estimated_savings: 2000,
                            category: category.clone(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    pub fn analyze_memory_usage(&self, detailed: bool) -> Result<Vec<MemoryAnalysis>, StylusError> {
        let mut analyses = Vec::new();

        let memory_patterns = [
            (r"for\s+.*\s+in\s+0\s*\.\.\s*self\.holders\.len\(\)", 
             "Linear iteration over holders",
             "Consider using a more efficient data structure or index for holder lookup",
             MemoryPatternType::NestedIterations),

            (r"Vec::new\(\).*push\(", 
             "Dynamic vector growth in get_holder_balances",
             "Pre-allocate vector: Vec::with_capacity(holders.len())",
             MemoryPatternType::AllocationIntensive),

            (r"self\.balances\.get\(&[^)]*\).*self\.balances\.get\(&[^)]*\)", 
             "Multiple map accesses without caching",
             "Cache map values in local variables to reduce memory operations",
             MemoryPatternType::UnoptimizedCaching),

            (r"\.cloned\(\)|\.clone\(\)", 
             "Cloning in storage operations",
             "Use references instead of cloning data where possible",
             MemoryPatternType::TemporaryAllocations),
        ];

        for (line_num, line) in self.content.lines().enumerate() {
            for &(pattern, desc, suggestion, ref pattern_type) in &memory_patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(line) {
                        analyses.push(MemoryAnalysis {
                            line: line_num + 1,
                            description: desc.to_string(),
                            suggestion: suggestion.to_string(),
                            pattern_type: pattern_type.clone(),
                        });
                    }
                }
            }
        }

        if detailed {
            let detailed_patterns = [
                (r"Option<.*>\.cloned\(\)", 
                 "Option value cloning",
                 "Use as_ref() before cloning Option contents",
                 MemoryPatternType::TemporaryAllocations),

                (r"self\.holders\.get\(.*\).*\.clone\(\)", 
                 "Storage vector element cloning",
                 "Cache holder addresses before processing",
                 MemoryPatternType::AllocationIntensive),
            ];

            for (line_num, line) in self.content.lines().enumerate() {
                for &(pattern, desc, suggestion, ref pattern_type) in &detailed_patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        if regex.is_match(line) {
                            analyses.push(MemoryAnalysis {
                                line: line_num + 1,
                                description: desc.to_string(),
                                suggestion: suggestion.to_string(),
                                pattern_type: pattern_type.clone(),
                            });
                        }
                    }
                }
            }
        }

        Ok(analyses)
    }
}

pub fn analyze_gas_usage(content: String) -> Result<Vec<GasOptimization>, StylusError> {
    let analyzer = GasAnalyzer::new(content);
    analyzer.analyze()
}

pub fn analyze_memory_usage(content: String, detailed: bool) -> Result<Vec<MemoryAnalysis>, StylusError> {
    let analyzer = GasAnalyzer::new(content);
    analyzer.analyze_memory_usage(detailed)
}