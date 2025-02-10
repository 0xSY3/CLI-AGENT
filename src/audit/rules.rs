use super::{Vulnerability, Severity};
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait AuditRule: Send + Sync {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>>;
    fn name(&self) -> &'static str;
}

pub struct UnusedStorageRule;
pub struct UnsafeCallRule;
pub struct StoragePatternRule;

#[async_trait]
impl AuditRule for UnusedStorageRule {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        if content.contains("StorageU64") || content.contains("StorageU256") {
            if !content.contains(".get()") || !content.contains(".set(") {
                vulnerabilities.push(Vulnerability {
                    name: "Unused Storage Variable".to_string(),
                    severity: Severity::Low,
                    risk_description: "Storage variable declared but never accessed".to_string(),
                    recommendation: "Remove unused storage variables or implement their usage".to_string(),
                });
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Unused Storage Detector"
    }
}

#[async_trait]
impl AuditRule for UnsafeCallRule {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        if content.contains("unsafe") {
            vulnerabilities.push(Vulnerability {
                name: "Unsafe Block Usage".to_string(),
                severity: Severity::High,
                risk_description: "Contract contains unsafe blocks that may lead to memory corruption".to_string(),
                recommendation: "Review and remove unsafe blocks if possible".to_string(),
            });
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Unsafe Code Detector"
    }
}

#[async_trait]
impl AuditRule for StoragePatternRule {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        if content.contains("get") && content.contains("set") {
            if content.contains("&mut self") && !content.contains("#[stylus_sdk::storage]") {
                vulnerabilities.push(Vulnerability {
                    name: "Incorrect Storage Pattern".to_string(),
                    severity: Severity::Medium,
                    risk_description: "Storage pattern may not be optimal for L2 operations".to_string(),
                    recommendation: "Use Stylus SDK storage attributes and patterns".to_string(),
                });
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Storage Pattern Analyzer"
    }
}