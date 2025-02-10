use crate::audit::vulnerabilities::{Vulnerability, Severity};
use crate::audit::rules::AuditRule;
use std::error::Error;
use async_trait::async_trait;

pub struct TestPatternRule;

#[async_trait]
impl AuditRule for TestPatternRule {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        // Check for test module presence
        if !content.contains("#[cfg(test)]") {
            vulnerabilities.push(Vulnerability {
                name: "Missing Test Module".to_string(),
                severity: Severity::Medium,
                risk_description: "Untested code may contain bugs or vulnerabilities".to_string(),
                recommendation: "Add comprehensive test module with unit tests".to_string(),
            });
        }

        // Check for missing test assertions
        if content.contains("#[test]") && !content.contains("assert") {
            vulnerabilities.push(Vulnerability {
                name: "Missing Test Assertions".to_string(),
                severity: Severity::Medium,
                risk_description: "Tests without assertions may not verify functionality".to_string(),
                recommendation: "Add assertions to verify test outcomes".to_string(),
            });
        }

        // Check for integration tests
        if !content.contains("#[test]") || !content.contains("integration") {
            vulnerabilities.push(Vulnerability {
                name: "Missing Integration Tests".to_string(),
                severity: Severity::Low,
                risk_description: "Contract interactions may not be fully tested".to_string(),
                recommendation: "Add integration tests for contract interactions".to_string(),
            });
        }

        // Check for fuzz testing
        if !content.contains("quickcheck") && !content.contains("proptest") {
            vulnerabilities.push(Vulnerability {
                name: "Missing Fuzz Testing".to_string(),
                severity: Severity::Low,
                risk_description: "Edge cases may not be discovered through regular testing".to_string(),
                recommendation: "Implement property-based testing using quickcheck or proptest".to_string(),
            });
        }

        // Check for error case testing
        if content.contains("#[test]") && !content.contains("should_panic") {
            vulnerabilities.push(Vulnerability {
                name: "Missing Error Case Tests".to_string(),
                severity: Severity::Medium,
                risk_description: "Error handling may not be properly tested".to_string(),
                recommendation: "Add tests for error cases using #[should_panic]".to_string(),
            });
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Testing Pattern Analyzer"
    }
}