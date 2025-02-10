use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;

pub struct SecurityAnalyzer;

#[async_trait::async_trait]
impl Analyzer for SecurityAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
        let content = fs::read_to_string(file)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        println!("🔍 Analyzing security patterns...");
        println!("⏳ Please wait while we process your contract...\n");
        let analysis = ai::analyze_security_issues(&content).await?;

        let output = format!(
            "\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n",
            "🔒 Security Analysis Report".bright_yellow().bold(),
            "═".repeat(40).bright_yellow(),
            "🔍 Security Findings:".yellow().bold(),
            format_security_findings(&analysis),
            "💡 Recommendations:".yellow().bold(),
            format_recommendations(&analysis),
            format_summary(&analysis)
        );
        Ok(output)
    }
}

fn format_security_findings(analysis: &str) -> String {
    let mut findings = String::new();

    // Format findings by severity
    for line in analysis.lines() {
        if line.starts_with("### ") || line.starts_with("**") {
            // Skip markdown formatting
            continue;
        }

        if line.contains("Critical") {
            findings.push_str(&format!("🚨 {}\n", line.trim_start_matches("- ").red().bold()));
        } else if line.contains("High") {
            findings.push_str(&format!("⚠️  {}\n", line.trim_start_matches("- ").yellow().bold()));
        } else if line.contains("Medium") {
            findings.push_str(&format!("ℹ️  {}\n", line.trim_start_matches("- ").blue()));
        } else if line.contains("Low") {
            findings.push_str(&format!("✅ {}\n", line.trim_start_matches("- ").green()));
        } else if line.contains("Recommendation") || line.contains("Mitigation") {
            findings.push_str(&format!("💡 {}\n", line.trim_start_matches("- ")));
        } else if !line.trim().is_empty() {
            let cleaned_line = line
                .trim_start_matches("- ")
                .trim_start_matches("* ")
                .trim_start_matches("> ");
            findings.push_str(&format!("  • {}\n", cleaned_line));
        }
    }

    findings
}

fn format_recommendations(analysis: &str) -> String {
    let mut recommendations = String::new();
    recommendations.push_str("\n");

    // Extract and format recommendations
    for line in analysis.lines() {
        if line.contains("Recommendation") || line.contains("Mitigation") {
            recommendations.push_str(&format!("• {}\n", line.trim_start_matches("- ").green()));
        }
    }

    // Add general security best practices
    recommendations.push_str("\nGeneral Security Best Practices:\n");
    recommendations.push_str("• Implement comprehensive access control\n");
    recommendations.push_str("• Use safe arithmetic operations\n");
    recommendations.push_str("• Follow checks-effects-interactions pattern\n");
    recommendations.push_str("• Add proper event emission and logging\n");
    recommendations.push_str("• Conduct thorough testing and auditing\n");

    recommendations
}

fn format_summary(analysis: &str) -> String {
    // Count issues by severity
    let critical = analysis.lines().filter(|line| line.contains("Critical")).count();
    let high = analysis.lines().filter(|line| line.contains("High")).count();
    let medium = analysis.lines().filter(|line| line.contains("Medium")).count();

    format!(
        "\n📊 Security Summary\n{}\n\n{}\n{}\n{}\n\n{}\n{}\n",
        "═".repeat(40).bright_yellow(),
        format!("🚨 Critical Issues: {}", critical).red().bold(),
        format!("⚠️  High Risk Issues: {}", high).yellow().bold(),
        format!("ℹ️  Medium Risk Issues: {}", medium).blue(),
        "Next Steps:".bright_yellow().bold(),
        if critical > 0 || high > 0 {
            "• Immediate attention required for critical/high issues\n• Review and fix security vulnerabilities\n• Consider external security audit"
        } else if medium > 0 {
            "• Address medium risk issues\n• Implement suggested improvements\n• Enhance security monitoring"
        } else {
            "• Continue monitoring security aspects\n• Keep dependencies updated\n• Maintain security best practices"
        }
    )
}