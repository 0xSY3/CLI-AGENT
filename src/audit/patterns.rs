use crate::audit::vulnerabilities::{Vulnerability, Severity};
use crate::audit::rules::AuditRule;
use crate::audit::memory_safety::MemorySafetyRule;
use crate::audit::l2_patterns::L2OptimizationRule;
use crate::audit::access_control::AccessControlRule;
use crate::audit::test_patterns::TestPatternRule;
use crate::audit::ai_patterns::AIPatternDetector;
use std::error::Error;

pub struct ReentrancyPattern;
pub struct L2SpecificPattern;
pub struct StorageSecurityPattern;
pub struct StateTransitionPattern;
pub struct CrossChainVulnerabilityPattern;

#[async_trait::async_trait]
impl AuditRule for ReentrancyPattern {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        if content.contains("external") && content.contains("call") {
            vulnerabilities.push(Vulnerability {
                name: "Potential Reentrancy".to_string(),
                severity: Severity::High,
                risk_description: "External call detected before state changes".to_string(),
                recommendation: "Implement checks-effects-interactions pattern".to_string(),
            });
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Reentrancy Pattern Checker"
    }
}

#[async_trait::async_trait]
impl AuditRule for L2SpecificPattern {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        if content.contains("block.number") || content.contains("block.timestamp") {
            vulnerabilities.push(Vulnerability {
                name: "L2 Timing Assumptions".to_string(),
                severity: Severity::Medium,
                risk_description: "Usage of block.number or block.timestamp in L2 context".to_string(),
                recommendation: "Use L2-specific timing mechanisms or account for L2 block timing".to_string(),
            });
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "L2-Specific Pattern Checker"
    }
}

#[async_trait::async_trait]
impl AuditRule for StorageSecurityPattern {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        if content.contains("StorageMap") || content.contains("StorageVec") {
            let has_bounds_check = content.contains(".get_or_default()") || content.contains("if let Some");
            let has_access_control = content.contains("#[authorize") || content.contains("require!(");

            if !has_bounds_check {
                vulnerabilities.push(Vulnerability {
                    name: "Unsafe Storage Access".to_string(),
                    severity: Severity::High,
                    risk_description: "Storage access without bounds checking".to_string(),
                    recommendation: "Implement bounds checking with get_or_default() or Option handling".to_string(),
                });
            }

            if !has_access_control {
                vulnerabilities.push(Vulnerability {
                    name: "Missing Storage Access Control".to_string(),
                    severity: Severity::High,
                    risk_description: "Storage modification without access control".to_string(),
                    recommendation: "Add access control checks using authorize attribute or require macro".to_string(),
                });
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Storage Security Pattern Analyzer"
    }
}

#[async_trait::async_trait]
impl AuditRule for StateTransitionPattern {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        if content.contains("pub fn") && (content.contains("mut self") || content.contains("&mut self")) {
            let has_state_validation = content.contains("ensure!(") || content.contains("require!(");
            let has_event_emission = content.contains("emit!(") || content.contains("log!(");

            if !has_state_validation {
                vulnerabilities.push(Vulnerability {
                    name: "Missing State Validation".to_string(),
                    severity: Severity::Medium,
                    risk_description: "State transition without proper validation".to_string(),
                    recommendation: "Add state validation using ensure! or require! macros".to_string(),
                });
            }

            if !has_event_emission {
                vulnerabilities.push(Vulnerability {
                    name: "Missing Event Emission".to_string(),
                    severity: Severity::Low,
                    risk_description: "State change without event emission".to_string(),
                    recommendation: "Emit events for all important state transitions".to_string(),
                });
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "State Transition Pattern Analyzer"
    }
}

#[async_trait::async_trait]
impl AuditRule for CrossChainVulnerabilityPattern {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        if content.contains("cross_chain") || content.contains("bridge") || content.contains("L1_to_L2") {
            let has_delay = content.contains("delay") || content.contains("timelock");
            let has_verification = content.contains("verify_proof") || content.contains("verify_message");

            if !has_delay {
                vulnerabilities.push(Vulnerability {
                    name: "Missing Cross-Chain Delay".to_string(),
                    severity: Severity::High,
                    risk_description: "Cross-chain operation without delay mechanism".to_string(),
                    recommendation: "Implement timelock or delay mechanism for cross-chain operations".to_string(),
                });
            }

            if !has_verification {
                vulnerabilities.push(Vulnerability {
                    name: "Insufficient Cross-Chain Verification".to_string(),
                    severity: Severity::Critical,
                    risk_description: "Cross-chain message without proper verification".to_string(),
                    recommendation: "Add proper verification for all cross-chain messages".to_string(),
                });
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Cross-Chain Vulnerability Analyzer"
    }
}

pub fn create_default_rules() -> Vec<Box<dyn AuditRule>> {
    vec![
        Box::new(ReentrancyPattern),
        Box::new(L2SpecificPattern),
        Box::new(StorageSecurityPattern), 
        Box::new(StateTransitionPattern),
        Box::new(CrossChainVulnerabilityPattern),
        Box::new(MemorySafetyRule),
        Box::new(L2OptimizationRule),
        Box::new(AccessControlRule),
        Box::new(TestPatternRule),
        Box::new(AIPatternDetector::new()),
    ]
}