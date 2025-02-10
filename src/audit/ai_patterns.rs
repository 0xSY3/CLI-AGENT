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
        // Core security weights - adjusted for critical vulnerabilities
        pattern_weights.insert("memory_safety".to_string(), 1.5);
        pattern_weights.insert("access_control".to_string(), 1.4);
        pattern_weights.insert("storage_pattern".to_string(), 1.3);
        pattern_weights.insert("cross_chain".to_string(), 1.5);

        // L2-specific weights - optimized
        pattern_weights.insert("l2_optimization".to_string(), 1.4);
        pattern_weights.insert("batch_operations".to_string(), 1.3);
        pattern_weights.insert("calldata_compression".to_string(), 1.4);
        pattern_weights.insert("state_packing".to_string(), 1.3);

        // Stylus-specific weights - refined for performance
        pattern_weights.insert("stylus_pattern".to_string(), 1.4);
        pattern_weights.insert("wasm_pattern".to_string(), 1.4);
        pattern_weights.insert("precompile".to_string(), 1.3);

        // New patterns for comprehensive detection
        pattern_weights.insert("arithmetic_safety".to_string(), 1.4);
        pattern_weights.insert("event_validation".to_string(), 1.2);
        pattern_weights.insert("upgrade_safety".to_string(), 1.3);
        pattern_weights.insert("dos_protection".to_string(), 1.4);
        pattern_weights.insert("input_validation".to_string(), 1.3);
        pattern_weights.insert("timestamp_dependence".to_string(), 1.3);

        Self {
            pattern_cache: HashMap::new(),
            pattern_weights,
            learning_threshold: 0.80, // Increased threshold for higher precision
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

        // Enhanced pattern detection
        self.detect_security_patterns(content, &mut patterns);
        self.detect_l2_optimization_patterns(content, &mut patterns);
        self.detect_stylus_specific_patterns(content, &mut patterns);
        self.detect_advanced_patterns(content, &mut patterns); // New method

        patterns = self.apply_pattern_weights(patterns);
        self.pattern_cache.insert(cache_key, patterns.clone());
        patterns
    }

    fn detect_security_patterns(&mut self, content: &str, patterns: &mut Vec<(String, f64)>) {
        // Enhanced access control detection
        if content.contains("pub fn") || content.contains("public") || content.contains("external") {
            let mut confidence = 0.85;
            if !content.contains("#[access_control") && !content.contains("require!(msg.sender") {
                confidence += 0.10;
            }
            if content.contains("owner") || content.contains("admin") || content.contains("role") {
                confidence += 0.05;
            }
            patterns.push(("Access Control Risk".to_string(), confidence));
        }

        // Advanced memory safety detection
        if content.contains("unsafe") || content.contains("*mut") || content.contains("*const") || content.contains("raw pointer") {
            let mut confidence = 0.90;
            if !content.contains("Box<") && !content.contains("Rc<") {
                confidence += 0.05;
            }
            if content.contains("transmute") || content.contains("offset") {
                confidence += 0.05;
            }
            patterns.push(("Memory Safety Risk".to_string(), confidence));
        }

        // Comprehensive reentrancy detection
        if content.contains("external_call") || content.contains("send") || content.contains("transfer") || content.contains("call") {
            let mut confidence = 0.90;
            if content.contains("self.") && !content.contains("mutex") && !content.contains("reentrancy_guard") {
                confidence += 0.05;
            }
            if content.contains("balance") || content.contains("withdraw") || content.contains("eth_transfer") {
                confidence += 0.05;
            }
            patterns.push(("Reentrancy Risk".to_string(), confidence));
        }

        // Advanced arithmetic safety detection
        if content.contains("u256") || content.contains("u128") || content.contains("arithmetic") || content.contains("math") {
            let mut confidence = 0.85;
            if !content.contains("checked_add") && !content.contains("checked_mul") {
                confidence += 0.10;
            }
            if content.contains("unchecked") || content.contains("unsafe_") || content.contains("overflow") {
                confidence += 0.05;
            }
            patterns.push(("Arithmetic Safety Risk".to_string(), confidence));
        }

        // DoS protection detection
        if content.contains("loop") || content.contains("for") || content.contains("while") || content.contains("array") {
            let mut confidence = 0.80;
            if !content.contains("limit") && !content.contains("max_") {
                confidence += 0.15;
            }
            if content.contains("push") || content.contains("extend") {
                confidence += 0.05;
            }
            patterns.push(("DoS Risk".to_string(), confidence));
        }

        // Input validation detection
        if content.contains("input") || content.contains("param") || content.contains("argument") {
            let mut confidence = 0.80;
            if !content.contains("validate") && !content.contains("require") && !content.contains("assert") {
                confidence += 0.15;
            }
            if content.contains("external") || content.contains("public") {
                confidence += 0.05;
            }
            patterns.push(("Input Validation Risk".to_string(), confidence));
        }
    }

    fn detect_l2_optimization_patterns(&mut self, content: &str, patterns: &mut Vec<(String, f64)>) {
        // Enhanced batch operations detection
        if content.contains("loop") || content.contains("for") || content.contains("while") {
            let mut confidence = 0.75;
            if !content.contains("batch") {
                confidence += 0.15;
            }
            if content.contains("array") || content.contains("vec") {
                confidence += 0.1;
            }
            patterns.push(("Batch Operations".to_string(), confidence));
        }

        // Improved calldata optimization
        if content.contains("calldata") || content.contains("input") {
            let mut confidence = 0.75;
            if !content.contains("compress") && !content.contains("packed") {
                confidence += 0.15;
            }
            if content.contains("encoding") || content.contains("decode") {
                confidence += 0.1;
            }
            patterns.push(("Calldata Optimization".to_string(), confidence));
        }

        // Enhanced state packing
        if content.contains("struct") || content.contains("StorageMap") {
            let mut confidence = 0.75;
            if !content.contains("#[repr(packed)]") {
                confidence += 0.15;
            }
            if content.contains("storage") || content.contains("state") {
                confidence += 0.1;
            }
            patterns.push(("State Packing".to_string(), confidence));
        }
    }

    fn detect_stylus_specific_patterns(&mut self, content: &str, patterns: &mut Vec<(String, f64)>) {
        // Improved SDK integration detection
        if content.contains("stylus_sdk") {
            let mut confidence = 0.75;
            if !content.contains("#[stylus_sdk::contract]") {
                confidence += 0.15;
            }
            if content.contains("precompile") || content.contains("native") {
                confidence += 0.1;
            }
            patterns.push(("Stylus SDK Usage".to_string(), confidence));
        }

        // Enhanced precompile usage detection
        if content.contains("precompile") {
            let mut confidence = 0.75;
            if !content.contains("verify") {
                confidence += 0.15;
            }
            if content.contains("unsafe") || content.contains("external") {
                confidence += 0.1;
            }
            patterns.push(("Precompile Usage".to_string(), confidence));
        }

        // Improved WASM optimization detection
        if content.contains("wasm") {
            let mut confidence = 0.75;
            if !content.contains("memory") {
                confidence += 0.15;
            }
            if content.contains("export") || content.contains("import") {
                confidence += 0.1;
            }
            patterns.push(("WASM Optimization".to_string(), confidence));
        }
    }

    fn detect_advanced_patterns(&mut self, content: &str, patterns: &mut Vec<(String, f64)>) {
        // Enhanced event validation patterns
        if content.contains("event") || content.contains("emit") || content.contains("#[event]") {
            let mut confidence = 0.80;
            if !content.contains("indexed") {
                confidence += 0.10;
            }
            if content.contains("anonymous") || !content.contains("topic") {
                confidence += 0.10;
            }
            patterns.push(("Event Validation".to_string(), confidence));
        }

        // Comprehensive upgrade safety patterns
        if content.contains("upgrade") || content.contains("proxy") || content.contains("implementation") {
            let mut confidence = 0.85;
            if !content.contains("initialize") || !content.contains("version") {
                confidence += 0.10;
            }
            if content.contains("storage") && content.contains("layout") {
                confidence += 0.05;
            }
            patterns.push(("Upgrade Safety".to_string(), confidence));
        }

        // Advanced cross-chain interaction patterns
        if content.contains("bridge") || content.contains("cross-chain") || content.contains("L1") || content.contains("L2") {
            let mut confidence = 0.90;
            if !content.contains("verify") || !content.contains("proof") {
                confidence += 0.05;
            }
            if content.contains("message") || content.contains("relay") || content.contains("gateway") {
                confidence += 0.05;
            }
            patterns.push(("Cross-chain Security".to_string(), confidence));
        }

        // Timestamp dependence patterns
        if content.contains("timestamp") || content.contains("block.timestamp") || content.contains("now") {
            let mut confidence = 0.85;
            if !content.contains("grace_period") || !content.contains("time_buffer") {
                confidence += 0.10;
            }
            if content.contains("require") && content.contains("time") {
                confidence += 0.05;
            }
            patterns.push(("Timestamp Dependence".to_string(), confidence));
        }
    }
}

#[async_trait::async_trait]
impl AuditRule for AIPatternDetector {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        self.detected_vulnerabilities.clear();
        let mut vulnerabilities = Vec::new();
        let patterns = self.analyze_semantic_patterns(content);

        for (pattern, confidence) in patterns {
            if confidence > self.learning_threshold {
                let vuln = match pattern.as_str() {
                    "Access Control Risk" => Vulnerability {
                        name: "Missing Access Control".to_string(),
                        severity: Severity::High,
                        risk_description: "Functions lack proper access control mechanisms".to_string(),
                        recommendation: "Implement role-based access control using Stylus SDK's security features".to_string(),
                    },
                    "Memory Safety Risk" => Vulnerability {
                        name: "Memory Safety Issue".to_string(),
                        severity: Severity::Critical,
                        risk_description: "Potential memory corruption from unsafe operations".to_string(),
                        recommendation: "Replace unsafe operations with safe alternatives and use Rust's ownership system".to_string(),
                    },
                    "Reentrancy Risk" => Vulnerability {
                        name: "Reentrancy Vulnerability".to_string(),
                        severity: Severity::Critical,
                        risk_description: "Contract state could be manipulated through external calls".to_string(),
                        recommendation: "Implement reentrancy guards and follow checks-effects-interactions pattern".to_string(),
                    },
                    "Arithmetic Safety Risk" => Vulnerability {
                        name: "Arithmetic Safety Risk".to_string(),
                        severity: Severity::High,
                        risk_description: "Potential integer overflow/underflow in calculations".to_string(),
                        recommendation: "Use checked arithmetic operations and consider using SafeMath equivalents".to_string(),
                    },
                    "Batch Operations" => Vulnerability {
                        name: "Unoptimized Batch Operations".to_string(),
                        severity: Severity::Medium,
                        risk_description: "Inefficient gas usage in loop operations".to_string(),
                        recommendation: "Implement batch processing and optimize loop conditions".to_string(),
                    },
                    "State Packing" => Vulnerability {
                        name: "Inefficient State Packing".to_string(),
                        severity: Severity::Low,
                        risk_description: "Suboptimal storage layout increases gas costs".to_string(),
                        recommendation: "Use packed structs and optimize storage slot usage".to_string(),
                    },
                    "Event Validation" => Vulnerability {
                        name: "Insufficient Event Validation".to_string(),
                        severity: Severity::Medium,
                        risk_description: "Events may lack proper validation or indexing".to_string(),
                        recommendation: "Add proper event parameter validation and optimize indexing".to_string(),
                    },
                    "Upgrade Safety" => Vulnerability {
                        name: "Upgrade Safety Concerns".to_string(),
                        severity: Severity::High,
                        risk_description: "Contract upgrades may introduce vulnerabilities".to_string(),
                        recommendation: "Implement proper upgrade patterns and storage layout checks".to_string(),
                    },
                    "Cross-chain Security" => Vulnerability {
                        name: "Cross-chain Interaction Risks".to_string(),
                        severity: Severity::Critical,
                        risk_description: "Unsafe cross-chain message handling".to_string(),
                        recommendation: "Implement proper message verification and handle edge cases".to_string(),
                    },
                    "DoS Risk" => Vulnerability {
                        name: "Denial of Service Risk".to_string(),
                        severity: Severity::High,
                        risk_description: "Potential for denial-of-service attacks due to unbounded loops or resource consumption.".to_string(),
                        recommendation: "Implement input validation and resource limits to prevent DoS attacks.".to_string(),
                    },
                    "Input Validation Risk" => Vulnerability {
                        name: "Insufficient Input Validation".to_string(),
                        severity: Severity::High,
                        risk_description: "Lack of input validation can lead to unexpected behavior or vulnerabilities.".to_string(),
                        recommendation: "Implement robust input validation to sanitize and check all inputs before processing.".to_string(),
                    },
                    "Timestamp Dependence" => Vulnerability {
                        name: "Timestamp Dependence Vulnerability".to_string(),
                        severity: Severity::Medium,
                        risk_description: "Contract logic relies on block timestamps, which can be manipulated by miners.".to_string(),
                        recommendation: "Avoid using block timestamps for critical logic; use timelocks or other mechanisms for predictable timing.".to_string(),
                    },
                    _ => continue,
                };
                vulnerabilities.push(vuln);
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "AI-Powered Security & Pattern Analyzer"
    }
}