use crate::audit::vulnerabilities::{Vulnerability, Severity};
use crate::audit::rules::AuditRule;
use std::error::Error;
use std::collections::{HashMap, HashSet};

pub struct AIPatternDetector {
    pattern_cache: HashMap<String, Vec<(String, f64)>>,
    pattern_weights: HashMap<String, f64>,
    learning_threshold: f64,
    detected_vulnerabilities: HashSet<String>,
}

impl AIPatternDetector {
    pub fn new() -> Self {
        let mut pattern_weights = HashMap::new();
        // Core security weights
        pattern_weights.insert("memory_safety".to_string(), 1.3);
        pattern_weights.insert("access_control".to_string(), 1.2);
        pattern_weights.insert("storage_pattern".to_string(), 1.1);
        pattern_weights.insert("cross_chain".to_string(), 1.3);

        // L2-specific weights
        pattern_weights.insert("l2_optimization".to_string(), 1.2);
        pattern_weights.insert("batch_operations".to_string(), 1.1);
        pattern_weights.insert("calldata_compression".to_string(), 1.2);
        pattern_weights.insert("state_packing".to_string(), 1.1);

        // Stylus-specific weights
        pattern_weights.insert("stylus_pattern".to_string(), 1.2);
        pattern_weights.insert("wasm_pattern".to_string(), 1.2);
        pattern_weights.insert("precompile".to_string(), 1.1);

        Self {
            pattern_cache: HashMap::new(),
            pattern_weights,
            learning_threshold: 0.7,
            detected_vulnerabilities: HashSet::new(),
        }
    }

    fn apply_pattern_weights(&self, patterns: Vec<(String, f64)>) -> Vec<(String, f64)> {
        patterns.into_iter()
            .map(|(pattern, confidence)| {
                let weight = self.pattern_weights
                    .get(&pattern.to_lowercase())
                    .copied()
                    .unwrap_or(1.0);
                (pattern, (confidence * weight).min(1.0))
            })
            .collect()
    }

    fn analyze_semantic_patterns(&mut self, content: &str) -> Vec<(String, f64)> {
        let cache_key = content.get(0..100).unwrap_or(content).to_string();
        if let Some(cached_patterns) = self.pattern_cache.get(&cache_key) {
            return cached_patterns.clone();
        }

        let mut patterns = Vec::new();

        // Core security patterns
        self.detect_security_patterns(content, &mut patterns);
        // L2 optimization patterns
        self.detect_l2_optimization_patterns(content, &mut patterns);
        // Stylus-specific patterns
        self.detect_stylus_specific_patterns(content, &mut patterns);

        patterns = self.apply_pattern_weights(patterns);
        self.pattern_cache.insert(cache_key, patterns.clone());
        patterns
    }

    fn detect_security_patterns(&mut self, content: &str, patterns: &mut Vec<(String, f64)>) {
        // Access control patterns
        if content.contains("pub fn") {
            let mut confidence = 0.7;
            if !content.contains("#[access_control") && !content.contains("require!(msg.sender") {
                confidence += 0.2;
            }
            patterns.push(("Access Control Risk".to_string(), confidence));
        }

        // Memory safety patterns
        if content.contains("unsafe") || content.contains("*mut") || content.contains("*const") {
            let mut confidence = 0.8;
            if !content.contains("Box<") && !content.contains("Rc<") {
                confidence += 0.1;
            }
            patterns.push(("Memory Safety Risk".to_string(), confidence));
        }

        // Reentrancy detection
        if content.contains("external_call") || content.contains("send") {
            let mut confidence = 0.8;
            if content.contains("self.") && !content.contains("mutex") {
                confidence += 0.1;
            }
            patterns.push(("Reentrancy Risk".to_string(), confidence));
        }

        // Integer overflow detection
        if content.contains("u256") || content.contains("u128") {
            let mut confidence = 0.6;
            if !content.contains("checked_add") && !content.contains("checked_mul") {
                confidence += 0.2;
            }
            if content.contains("unchecked") {
                confidence += 0.2;
            }
            patterns.push(("Integer Overflow Risk".to_string(), confidence));
        }
    }

    fn detect_l2_optimization_patterns(&mut self, content: &str, patterns: &mut Vec<(String, f64)>) {
        // Batch operations
        if content.contains("loop") || content.contains("for") {
            let mut confidence = 0.7;
            if !content.contains("batch") {
                confidence += 0.2;
            }
            patterns.push(("Batch Operations".to_string(), confidence));
        }

        // Calldata optimization
        if content.contains("calldata") || content.contains("input") {
            let mut confidence = 0.7;
            if !content.contains("compress") && !content.contains("packed") {
                confidence += 0.2;
            }
            patterns.push(("Calldata Optimization".to_string(), confidence));
        }

        // State packing
        if content.contains("struct") || content.contains("StorageMap") {
            let mut confidence = 0.7;
            if !content.contains("#[repr(packed)]") {
                confidence += 0.2;
            }
            patterns.push(("State Packing".to_string(), confidence));
        }
    }

    fn detect_stylus_specific_patterns(&mut self, content: &str, patterns: &mut Vec<(String, f64)>) {
        // SDK integration
        if content.contains("stylus_sdk") {
            let mut confidence = 0.7;
            if !content.contains("#[stylus_sdk::contract]") {
                confidence += 0.2;
            }
            patterns.push(("Stylus SDK Usage".to_string(), confidence));
        }

        // Precompile usage
        if content.contains("precompile") {
            let mut confidence = 0.7;
            if !content.contains("verify") {
                confidence += 0.2;
            }
            patterns.push(("Precompile Usage".to_string(), confidence));
        }

        // WASM optimization
        if content.contains("wasm") {
            let mut confidence = 0.7;
            if !content.contains("memory") {
                confidence += 0.2;
            }
            patterns.push(("WASM Optimization".to_string(), confidence));
        }
    }
}

#[async_trait::async_trait]
impl AuditRule for AIPatternDetector {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error>> {
        self.detected_vulnerabilities.clear();
        let mut vulnerabilities = Vec::new();
        let patterns = self.analyze_semantic_patterns(content);

        for (pattern, confidence) in patterns {
            if confidence > self.learning_threshold {
                match pattern.as_str() {
                    "Access Control Risk" => {
                        vulnerabilities.push(Vulnerability {
                            name: "Missing Access Control".to_string(),
                            severity: Severity::High,
                            risk_description: "Functions can be called by unauthorized users".to_string(),
                            recommendation: "Implement role-based access control using Stylus SDK".to_string(),
                        });
                    },
                    "Memory Safety Risk" => {
                        vulnerabilities.push(Vulnerability {
                            name: "Memory Safety Issue".to_string(),
                            severity: Severity::Critical,
                            risk_description: "Memory corruption risk from unsafe operations".to_string(),
                            recommendation: "Replace unsafe operations with safe alternatives".to_string(),
                        });
                    },
                    "Reentrancy Risk" => {
                        vulnerabilities.push(Vulnerability {
                            name: "Reentrancy Vulnerability".to_string(),
                            severity: Severity::Critical,
                            risk_description: "Contract state manipulation risk in external calls".to_string(),
                            recommendation: "Implement reentrancy guards for external calls".to_string(),
                        });
                    },
                    "Integer Overflow Risk" => {
                        vulnerabilities.push(Vulnerability {
                            name: "Integer Overflow Risk".to_string(),
                            severity: Severity::High,
                            risk_description: "Arithmetic operations lack overflow protection".to_string(),
                            recommendation: "Use checked arithmetic operations for all calculations".to_string(),
                        });
                    },
                    "Batch Operations" => {
                        vulnerabilities.push(Vulnerability {
                            name: "Unoptimized Batch Operations".to_string(),
                            severity: Severity::Medium,
                            risk_description: "Higher gas costs from unoptimized loops".to_string(),
                            recommendation: "Implement batch processing for loop operations".to_string(),
                        });
                    },
                    "State Packing" => {
                        vulnerabilities.push(Vulnerability {
                            name: "Inefficient State Packing".to_string(),
                            severity: Severity::Low,
                            risk_description: "Increased storage costs from unpacked data".to_string(),
                            recommendation: "Implement storage packing strategies for contract state".to_string(),
                        });
                    },
                    _ => {}
                }
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "AI-Powered Security & Pattern Analyzer"
    }
}