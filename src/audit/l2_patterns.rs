use crate::audit::vulnerabilities::{Vulnerability, Severity};
use crate::audit::rules::AuditRule;
use std::error::Error;
use async_trait::async_trait;

pub struct L2OptimizationRule;

#[async_trait]
impl AuditRule for L2OptimizationRule {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        // Check for batch operation patterns
        if content.contains("loop") && !content.contains("batch") {
            vulnerabilities.push(Vulnerability {
                name: "Missing Batch Operations".to_string(),
                severity: Severity::Medium,
                risk_description: "Non-batched operations may lead to higher gas costs on L2".to_string(),
                recommendation: "Implement batching for loop operations to optimize gas costs".to_string(),
            });
        }

        // Check for calldata optimization
        if content.contains("&[u8]") || content.contains("Vec<u8>") {
            if !content.contains("compression") && !content.contains("compact") {
                vulnerabilities.push(Vulnerability {
                    name: "Unoptimized Calldata".to_string(),
                    severity: Severity::Medium,
                    risk_description: "Uncompressed calldata increases L1 posting costs".to_string(),
                    recommendation: "Implement calldata compression for large data structures".to_string(),
                });
            }
        }

        // Check for storage slot packing
        if content.contains("StorageMap") || content.contains("StorageVec") {
            if !content.contains("packed") && !content.contains("#[repr(packed)]") {
                vulnerabilities.push(Vulnerability {
                    name: "Unpacked Storage".to_string(),
                    severity: Severity::Low,
                    risk_description: "Inefficient storage slot usage increases gas costs".to_string(),
                    recommendation: "Pack storage slots efficiently using appropriate data layouts".to_string(),
                });
            }
        }

        // Check for L2-specific event optimization
        if content.contains("emit!") || content.contains("log!") {
            if !content.contains("indexed") {
                vulnerabilities.push(Vulnerability {
                    name: "Unoptimized Event Indexing".to_string(),
                    severity: Severity::Low,
                    risk_description: "Non-indexed events may increase gas costs and reduce searchability".to_string(),
                    recommendation: "Use indexed parameters for searchable event data".to_string(),
                });
            }
        }

        // Stylus-specific patterns
        if content.contains("stylus_sdk") {
            // Check for proper memory management
            if !content.contains("prealloc") && (content.contains("Vec::new") || content.contains("String::new")) {
                vulnerabilities.push(Vulnerability {
                    name: "Non-preallocated Collections".to_string(),
                    severity: Severity::Medium,
                    risk_description: "Dynamic allocation in Stylus contracts can be expensive".to_string(),
                    recommendation: "Use preallocation for collections when size is known".to_string(),
                });
            }

            // Check for cross-contract call optimization
            if content.contains("call!") && !content.contains("multicall") {
                vulnerabilities.push(Vulnerability {
                    name: "Unoptimized Cross-Contract Calls".to_string(),
                    severity: Severity::Medium,
                    risk_description: "Multiple separate calls increase L2 operation costs".to_string(),
                    recommendation: "Use multicall pattern for batching cross-contract interactions".to_string(),
                });
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "L2 Optimization Analyzer"
    }
}