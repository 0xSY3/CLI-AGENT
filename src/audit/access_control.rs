use crate::audit::vulnerabilities::{Vulnerability, Severity};
use crate::audit::rules::AuditRule;
use std::error::Error;
use async_trait::async_trait;

pub struct AccessControlRule;

#[async_trait]
impl AuditRule for AccessControlRule {
    async fn check(&mut self, content: &str) -> Result<Vec<Vulnerability>, Box<dyn Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        // Check for public functions without access control
        if content.contains("pub fn") && !content.contains("#[access_control") {
            let has_role_check = content.contains("require!(msg.sender") || 
                                content.contains("ensure!(is_owner") ||
                                content.contains("only_owner");

            if !has_role_check {
                vulnerabilities.push(Vulnerability {
                    name: "Missing Access Control".to_string(),
                    severity: Severity::High,
                    risk_description: "Functions can be called by unauthorized users".to_string(),
                    recommendation: "Implement role-based access control using Stylus SDK".to_string(),
                });
            }
        }

        // Check for privileged operations
        if content.contains("admin") || content.contains("owner") {
            if !content.contains("initialize") || !content.contains("constructor") {
                vulnerabilities.push(Vulnerability {
                    name: "Uninitialized Admin Role".to_string(),
                    severity: Severity::Critical,
                    risk_description: "Contract may lack proper administrative controls".to_string(),
                    recommendation: "Initialize admin roles in constructor or initialization function".to_string(),
                });
            }
        }

        // Check for role management
        if content.contains("role") || content.contains("permission") {
            let has_role_management = content.contains("grant_role") || 
                                    content.contains("revoke_role") ||
                                    content.contains("renounce_role");

            if !has_role_management {
                vulnerabilities.push(Vulnerability {
                    name: "Incomplete Role Management".to_string(),
                    severity: Severity::Medium,
                    risk_description: "Unable to modify roles after deployment".to_string(),
                    recommendation: "Implement complete role management functionality".to_string(),
                });
            }
        }

        Ok(vulnerabilities)
    }

    fn name(&self) -> &'static str {
        "Access Control Pattern Analyzer"
    }
}