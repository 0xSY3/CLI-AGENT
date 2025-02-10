use super::{AuditResult, Vulnerability};
use colored::*;

pub fn generate_full_report(result: &AuditResult) -> String {
    let mut report = String::new();

    // Header
    report.push_str(&format!("{}\n", 
        "Smart Contract Security Audit Report".bright_green().bold()
    ));
    report.push_str(&format!("{}\n\n", "═".repeat(50).bright_green()));

    // Vulnerability Summary
    report.push_str(&format!("{}\n", "Summary".bold()));
    report.push_str(&format!("Critical Issues: {}\n", result.critical_vulnerabilities.len().to_string().red()));
    report.push_str(&format!("High Issues: {}\n", result.high_vulnerabilities.len().to_string().yellow()));
    report.push_str(&format!("Medium Issues: {}\n", result.medium_vulnerabilities.len().to_string().blue()));
    report.push_str(&format!("Low Issues: {}\n\n", result.low_vulnerabilities.len().to_string().green()));

    // Detailed Findings
    if !result.critical_vulnerabilities.is_empty() {
        report.push_str(&format!("\n{}\n", "Critical Findings".red().bold()));
        for vuln in &result.critical_vulnerabilities {
            report.push_str(&format_vulnerability(vuln, "❗"));
        }
    }

    if !result.high_vulnerabilities.is_empty() {
        report.push_str(&format!("\n{}\n", "High Risk Findings".yellow().bold()));
        for vuln in &result.high_vulnerabilities {
            report.push_str(&format_vulnerability(vuln, "⚠️"));
        }
    }

    if !result.medium_vulnerabilities.is_empty() {
        report.push_str(&format!("\n{}\n", "Medium Risk Findings".blue().bold()));
        for vuln in &result.medium_vulnerabilities {
            report.push_str(&format_vulnerability(vuln, "ℹ️"));
        }
    }

    if !result.low_vulnerabilities.is_empty() {
        report.push_str(&format!("\n{}\n", "Low Risk Findings".green().bold()));
        for vuln in &result.low_vulnerabilities {
            report.push_str(&format_vulnerability(vuln, "📝"));
        }
    }

    // Mitigation Summary
    if result.critical_vulnerabilities.is_empty() && 
       result.high_vulnerabilities.is_empty() && 
       result.medium_vulnerabilities.is_empty() && 
       result.low_vulnerabilities.is_empty() {
        report.push_str(&format!("\n{}\n", "✅ No vulnerabilities found!".green()));
    } else {
        report.push_str(&format!("\n{}\n", "Recommended Actions".cyan().bold()));
        report.push_str("• Review all identified vulnerabilities\n");
        report.push_str("• Prioritize fixes based on severity level\n");
        report.push_str("• Implement suggested mitigations\n");
        report.push_str("• Conduct thorough testing after fixes\n");
        report.push_str("• Consider additional security review\n");
    }

    report
}

fn format_vulnerability(vuln: &Vulnerability, icon: &str) -> String {
    format!("{} {}\n  Risk: {}\n  Mitigation: {}\n\n",
        icon,
        vuln.name,
        vuln.risk_description,
        vuln.recommendation.bright_green()
    )
}