use super::{AuditResult, Vulnerability};
use colored::*;
use std::collections::HashSet;

pub fn generate_report(result: &AuditResult) -> String {
    let mut report = String::new();
    let risk_score = calculate_risk_score(result);
    let total = total_issues(result);

    // Header and Summary
    report.push_str(&format!("{}\n", 
        "Smart Contract Security Audit".bright_green().bold()
    ));
    report.push_str(&format!("{}\n\n", "═".repeat(40).bright_green()));
    report.push_str(&format!("Risk Score: {}/10 | Issues Found: {}\n\n",
        format!("{:.1}", risk_score).cyan().bold(),
        total.to_string().cyan()
    ));

    let mut seen = HashSet::new();

    // Critical & High Issues
    if !result.critical_vulnerabilities.is_empty() || !result.high_vulnerabilities.is_empty() {
        report.push_str(&format!("{}\n", "Critical & High Severity Issues".red().bold()));

        for vuln in &result.critical_vulnerabilities {
            let key = format!("{}:{}", vuln.name, vuln.recommendation);
            if !seen.contains(&key) {
                seen.insert(key);
                report.push_str(&format_critical_vulnerability(vuln));
            }
        }

        for vuln in &result.high_vulnerabilities {
            let key = format!("{}:{}", vuln.name, vuln.recommendation);
            if !seen.contains(&key) {
                seen.insert(key);
                report.push_str(&format_high_vulnerability(vuln));
            }
        }
    }

    // Medium Issues
    if !result.medium_vulnerabilities.is_empty() {
        report.push_str(&format!("\n{}\n", "Medium Severity Issues".yellow().bold()));
        for vuln in &result.medium_vulnerabilities {
            let key = format!("{}:{}", vuln.name, vuln.recommendation);
            if !seen.contains(&key) {
                seen.insert(key);
                report.push_str(&format_medium_vulnerability(vuln));
            }
        }
    }

    // Low Issues
    if !result.low_vulnerabilities.is_empty() {
        report.push_str(&format!("\n{}\n", "Low Severity Issues".blue().bold()));
        for vuln in &result.low_vulnerabilities {
            let key = format!("{}:{}", vuln.name, vuln.recommendation);
            if !seen.contains(&key) {
                seen.insert(key);
                report.push_str(&format_low_vulnerability(vuln));
            }
        }
    }

    // Recommendations
    if total > 0 {
        report.push_str(&format!("\n{}\n", "Recommended Actions".bright_cyan().bold()));
        report.push_str(&generate_action_items(result));
    }

    report
}

fn format_critical_vulnerability(vuln: &Vulnerability) -> String {
    format!("❗ {}\n  Risk: {}\n  Fix: {}\n\n",
        vuln.name.red().bold(),
        vuln.risk_description,
        vuln.recommendation.bright_green()
    )
}

fn format_high_vulnerability(vuln: &Vulnerability) -> String {
    format!("⚠️ {}\n  Risk: {}\n  Fix: {}\n\n",
        vuln.name.red(),
        vuln.risk_description,
        vuln.recommendation.bright_green()
    )
}

fn format_medium_vulnerability(vuln: &Vulnerability) -> String {
    format!("⚡ {}\n  Risk: {}\n  Fix: {}\n\n",
        vuln.name.yellow(),
        vuln.risk_description,
        vuln.recommendation.bright_green()
    )
}

fn format_low_vulnerability(vuln: &Vulnerability) -> String {
    format!("ℹ️ {}\n  Risk: {}\n  Fix: {}\n\n",
        vuln.name.blue(),
        vuln.risk_description,
        vuln.recommendation.bright_green()
    )
}

fn total_issues(result: &AuditResult) -> usize {
    result.critical_vulnerabilities.len() +
    result.high_vulnerabilities.len() +
    result.medium_vulnerabilities.len() +
    result.low_vulnerabilities.len()
}

fn calculate_risk_score(result: &AuditResult) -> f64 {
    let critical_weight = 10.0;
    let high_weight = 7.0;
    let medium_weight = 4.0;
    let low_weight = 1.0;

    let total_weight = result.critical_vulnerabilities.len() as f64 * critical_weight +
                      result.high_vulnerabilities.len() as f64 * high_weight +
                      result.medium_vulnerabilities.len() as f64 * medium_weight +
                      result.low_vulnerabilities.len() as f64 * low_weight;

    let max_score = 10.0;
    let normalized_score = total_weight / 3.0;

    normalized_score.min(max_score)
}

fn generate_action_items(result: &AuditResult) -> String {
    let mut items = String::new();
    let mut actions = Vec::new();

    // Critical priority actions
    for vuln in &result.critical_vulnerabilities {
        actions.push(format!("1. {}", vuln.recommendation));
    }

    // High priority actions
    for vuln in &result.high_vulnerabilities {
        actions.push(format!("2. {}", vuln.recommendation));
    }

    // Medium priority actions
    for vuln in &result.medium_vulnerabilities {
        actions.push(format!("3. {}", vuln.recommendation));
    }

    // Sort actions by priority (prefix number) and remove duplicates
    actions.sort();
    actions.dedup();

    // Remove priority numbers and format
    for action in actions {
        items.push_str(&format!("• {}\n", &action[3..]));
    }

    items
}