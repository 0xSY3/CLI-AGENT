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

    fn format_audit_result(&self, result: &AuditResult) -> String {
        report::generate_report(result)
    }
}

#[async_trait::async_trait]
impl Analyzer for AuditAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error>> {
        let content = std::fs::read_to_string(file)?;

        let mut audit_result = AuditResult {
            critical_vulnerabilities: Vec::new(),
            high_vulnerabilities: Vec::new(),
            medium_vulnerabilities: Vec::new(),
            low_vulnerabilities: Vec::new(),
        };

        // Get all rules first
        let rules = {
            let guard = self.rules.read().unwrap();
            guard.iter().map(|rule| rule.name().to_string()).collect::<Vec<_>>()
        };

        // Process each rule individually
        for rule_name in rules {
            let mut rule = {
                let mut guard = self.rules.write().unwrap();
                let idx = guard.iter().position(|r| r.name() == rule_name).unwrap();
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
                Err(e) => eprintln!("Error running rule {}: {}", rule_name, e),
            }

            // Put the rule back
            self.rules.write().unwrap().push(rule);
        }

        Ok(self.format_audit_result(&audit_result))
    }

    fn format_output(&self, analysis: &str) -> String {
        format!("{}", analysis)
    }
}