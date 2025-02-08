use crate::audit::vulnerabilities::{Vulnerability, Severity};
use crate::audit::rules::AuditRule;
use std::error::Error;

pub struct MemorySafetyRule;

#[async_trait::async_trait]
impl AuditRule for MemorySafetyRule {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error>> {
        let mut vulnerabilities = Vec::new();

        // Check raw pointer usage
        if content.contains("*mut") || content.contains("*const") {
            vulnerabilities.push(Vulnerability {
                name: "Raw Pointer Usage".to_string(),
                severity: Severity::High,
                risk_description: "Raw pointers can lead to memory corruption and undefined behavior".to_string(),
                recommendation: "Use safe alternatives like references or smart pointers".to_string(),
            });
        }

        // Check unsafe block usage
        if content.contains("unsafe") && !content.contains("unsafe trait") {
            vulnerabilities.push(Vulnerability {
                name: "Unsafe Block Usage".to_string(),
                severity: Severity::Critical,
                risk_description: "Unsafe blocks can bypass Rust's memory safety guarantees".to_string(),
                recommendation: "Remove unsafe blocks or provide strong safety invariants".to_string(),
            });
        }

        // Check for potential memory leaks
        if content.contains("Box::into_raw") || content.contains("ManuallyDrop") {
            vulnerabilities.push(Vulnerability {
                name: "Potential Memory Leak".to_string(),
                severity: Severity::High,
                risk_description: "Memory leaks can cause resource exhaustion and contract failure".to_string(),
                recommendation: "Ensure proper cleanup of resources and avoid manual memory management".to_string(),
            });
        }

        // Check for uninitialized memory usage
        if content.contains("MaybeUninit") || content.contains("std::mem::uninitialized") {
            vulnerabilities.push(Vulnerability {
                name: "Uninitialized Memory Usage".to_string(),
                severity: Severity::Critical,
                risk_description: "Using uninitialized memory leads to undefined behavior".to_string(),
                recommendation: "Initialize all memory before use and avoid MaybeUninit when possible".to_string(),
            });
        }

        // Check for proper lifetime annotations
        if content.contains("'static") && content.contains("&mut") {
            vulnerabilities.push(Vulnerability {
                name: "Suspicious Lifetime Usage".to_string(),
                severity: Severity::Medium,
                risk_description: "Improper lifetime usage can lead to memory safety issues".to_string(),
                recommendation: "Review lifetime annotations and ensure they are necessary".to_string(),
            });
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Memory Safety Analyzer"
    }
}