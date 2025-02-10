use crate::audit::vulnerabilities::{Vulnerability, Severity};
use crate::audit::rules::AuditRule;
use std::error::Error;

pub struct MemorySafetyRule;

#[async_trait::async_trait]
impl AuditRule for MemorySafetyRule {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
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

        // Stylus-specific memory checks
        if content.contains("stylus_sdk") {
            // Check for large allocations
            if content.contains("Vec::with_capacity") && content.contains(">1024") {
                vulnerabilities.push(Vulnerability {
                    name: "Large Memory Allocation".to_string(),
                    severity: Severity::High,
                    risk_description: "Large memory allocations can cause contract execution failures".to_string(),
                    recommendation: "Use smaller, fixed-size allocations or paginate data".to_string(),
                });
            }

            // Check for proper storage usage
            if content.contains("storage::") && !content.contains("try_") {
                vulnerabilities.push(Vulnerability {
                    name: "Unchecked Storage Access".to_string(),
                    severity: Severity::Medium,
                    risk_description: "Storage operations without error handling may fail silently".to_string(),
                    recommendation: "Use try_ variants for storage operations and handle errors explicitly".to_string(),
                });
            }

            // Check for proper error handling in external calls
            if content.contains("external::") && !content.contains("Result<") {
                vulnerabilities.push(Vulnerability {
                    name: "Unchecked External Calls".to_string(),
                    severity: Severity::High,
                    risk_description: "External calls without proper error handling can lead to undefined state".to_string(),
                    recommendation: "Always use Result for external calls and handle all error cases".to_string(),
                });
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Memory Safety Analyzer"
    }
}