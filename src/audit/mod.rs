use std::path::PathBuf;
use std::error::Error;
use std::sync::RwLock;
use crate::analyzer::Analyzer;

pub mod patterns;
pub mod rules;
pub mod report;
pub mod vulnerabilities;
pub mod ai_patterns;
pub mod memory_safety;
pub mod l2_patterns;
pub mod access_control;
pub mod test_patterns;

use vulnerabilities::{Vulnerability, Severity};
use rules::AuditRule;
use report::generate_full_report;

#[derive(Debug)]
pub struct AuditResult {
    pub critical_vulnerabilities: Vec<Vulnerability>,
    pub high_vulnerabilities: Vec<Vulnerability>,
    pub medium_vulnerabilities: Vec<Vulnerability>,
    pub low_vulnerabilities: Vec<Vulnerability>,
}

pub struct AuditAnalyzer {
    rules: RwLock<Vec<Box<dyn AuditRule>>>,
}

impl AuditAnalyzer {
    pub fn new() -> Self {
        Self {
            rules: RwLock::new(Vec::new()),
        }
    }

    pub fn add_rule(&self, rule: Box<dyn AuditRule>) {
        self.rules.write().unwrap().push(rule);
    }
}

#[async_trait::async_trait]
impl Analyzer for AuditAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
        let content = std::fs::read_to_string(file).map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read file: {}", e)
            )) as Box<dyn Error + Send + Sync>
        })?;

        let mut audit_result = AuditResult {
            critical_vulnerabilities: Vec::new(),
            high_vulnerabilities: Vec::new(),
            medium_vulnerabilities: Vec::new(),
            low_vulnerabilities: Vec::new(),
        };

        // Get all rules first
        let rules = {
            let guard = self.rules.read().map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to acquire read lock: {}", e)
                )) as Box<dyn Error + Send + Sync>
            })?;
            guard.iter().map(|rule| rule.name().to_string()).collect::<Vec<_>>()
        };

        // Process each rule individually with improved error handling
        for rule_name in rules {
            let mut rule = {
                let mut guard = self.rules.write().map_err(|e| {
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to acquire write lock: {}", e)
                    )) as Box<dyn Error + Send + Sync>
                })?;
                let idx = guard.iter().position(|r| r.name() == rule_name).ok_or_else(|| {
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Rule '{}' not found", rule_name)
                    )) as Box<dyn Error + Send + Sync>
                })?;
                guard.swap_remove(idx)
            };

            match rule.check(&content).await {
                Ok(vulnerabilities) => {
                    for vuln in vulnerabilities {
                        match vuln.severity {
                            Severity::Critical => audit_result.critical_vulnerabilities.push(vuln),
                            Severity::High => audit_result.high_vulnerabilities.push(vuln),
                            Severity::Medium => audit_result.medium_vulnerabilities.push(vuln),
                            Severity::Low => audit_result.low_vulnerabilities.push(vuln),
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error running rule {}: {}", rule_name, e);
                }
            }

            // Put the rule back
            self.rules.write().map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to acquire write lock: {}", e)
                )) as Box<dyn Error + Send + Sync>
            })?.push(rule);
        }

        Ok(generate_full_report(&audit_result))
    }
}