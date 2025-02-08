use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;

pub struct InteractionsAnalyzer;

#[async_trait::async_trait]
impl Analyzer for InteractionsAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(file)?;
        println!("🔄 Analyzing cross-contract interactions...");
        println!("⏳ Please wait while we process your contract...\n");
        let analysis = ai::analyze_contract_interactions(&content).await?;
        Ok(analysis)
    }

    fn format_output(&self, analysis: &str) -> String {
        format!(
            "\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n",
            "🔗 Cross-Contract Interaction Analysis".bright_green().bold(),
            "═══════════════════════════════════".bright_green(),
            "📊 Interaction Patterns:".yellow().bold(),
            format_overview(analysis),
            "🛡️  Security Analysis:".yellow().bold(),
            format_interactions(analysis),
            "💡 Optimization Recommendations:".yellow().bold(),
            format_recommendations(analysis),
            format_summary(analysis)
        )
    }
}

fn format_overview(interactions: &str) -> String {
    format!(
        "{}\n{}\n",
        "📈 Overview".yellow().bold(),
        "═════════".yellow()
    ) + &interactions
        .lines()
        .take_while(|line| !line.contains("Risk Assessment"))
        .map(format_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_interactions(interactions: &str) -> String {
    interactions
        .lines()
        .skip_while(|line| !line.contains("Risk Assessment"))
        .take_while(|line| !line.contains("Trust Assumptions"))
        .map(|line| {
            if line.contains("Critical Risk") {
                format!("💥 {}", line.red().bold())
            } else if line.contains("High Risk") {
                format!("🚨 {}", line.red())
            } else if line.contains("Medium Risk") {
                format!("⚠️  {}", line.yellow())
            } else if line.contains("Low Risk") {
                format!("🟢 {}", line.green())
            } else if line.contains("Risk Assessment") || line.contains("Security Analysis") {
                let separator = format!("{}", "─".repeat(50));
                format!("\n┌{}\n{}\n", separator, line.cyan().bold())
            } else if line.trim().ends_with(":") {
                let separator = format!("{}", "─".repeat(30));
                format!("\n└{}\n  {}", separator, line.yellow().bold())
            } else if line.contains("Pattern:") {
                format!("\n📝 {}", line.cyan().bold())
            } else if line.contains("Impact:") {
                format!("💥 {}", line.yellow())
            } else if line.contains("Mitigation:") {
                format!("🛡️  {}", line.green().italic())
            } else if line.contains("Example:") {
                format!("📝 {}", line.cyan().italic())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_recommendations(recommendations: &str) -> String {
    recommendations
        .lines()
        .skip_while(|line| !line.contains("Trust Assumptions"))
        .map(|line| {
            if line.contains("Priority: Critical") {
                format!("💥 {}", line.red().bold())
            } else if line.contains("Priority: High") {
                format!("🚨 {}", line.red())
            } else if line.contains("Priority: Medium") {
                format!("⚠️  {}", line.yellow())
            } else if line.contains("Priority: Low") {
                format!("ℹ️  {}", line.green())
            } else if line.contains("Trust Assumptions") || line.contains("Security Framework") {
                let separator = format!("{}", "─".repeat(50));
                format!("\n┌{}\n{}\n", separator, line.cyan().bold())
            } else if line.trim().ends_with(":") {
                let separator = format!("{}", "─".repeat(30));
                format!("\n└{}\n  {}", separator, line.yellow().bold())
            } else if line.contains("Implementation:") {
                format!("\n🔧 {}", line.yellow())
            } else if line.contains("Impact:") {
                format!("💫 {}", line.yellow())
            } else if line.contains("Attack Vector:") {
                format!("🎯 {}", line.red().italic())
            } else if line.contains("Mitigation:") {
                format!("🛡️  {}", line.green())
            } else if line.contains("Example:") {
                format!("📝 {}", line.cyan().italic())
            } else if line.contains("Testing:") {
                format!("🧪 {}", line.green().italic())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_line(line: &str) -> String {
    if line.contains("Pattern:") {
        format!("\n📝 {}", line.cyan().bold())
    } else if line.contains("Risk Level:") {
        let risk = if line.contains("Critical") {
            "💥 Critical".red().bold()
        } else if line.contains("High") {
            "🚨 High".red()
        } else if line.contains("Medium") {
            "⚠️  Medium".yellow()
        } else {
            "🟢 Low".green()
        };
        format!("⚖️  Risk Level: {}", risk)
    } else if line.contains("Impact:") {
        format!("💫 {}", line.yellow())
    } else if line.contains("Mitigation:") {
        format!("🛡️  {}", line.green().italic())
    } else if line.contains("Example:") {
        format!("📝 {}", line.cyan().italic())
    } else {
        format!("  • {}", line)
    }
}

fn format_summary(interactions: &str) -> String {
    let critical_count = count_severity(interactions, "Critical Risk");
    let high_count = count_severity(interactions, "High Risk");
    let medium_count = count_severity(interactions, "Medium Risk");
    let low_count = count_severity(interactions, "Low Risk");

    format!(
        "{}\n{}\n\n{}\n{}\n{}\n{}\n\n{}\n{}\n",
        "📊 Interaction Risk Summary".bright_yellow().bold(),
        "════════════════════════".bright_yellow(),
        format!("💥 Critical Risk Patterns: {} found", critical_count).red().bold(),
        format!("🚨 High Risk Patterns: {} found", high_count).red(),
        format!("⚠️  Medium Risk Patterns: {} found", medium_count).yellow(),
        format!("🟢 Low Risk Patterns: {} found", low_count).green(),
        "🎯 Next Steps:".bright_yellow().bold(),
        format_next_steps(critical_count, high_count, medium_count)
    )
}

fn format_next_steps(critical: usize, high: usize, medium: usize) -> String {
    let mut steps = Vec::new();

    if critical > 0 {
        steps.push("💥 CRITICAL: Immediate action required - these patterns pose significant security risks");
    }
    if high > 0 {
        steps.push("🚨 HIGH: Prioritize fixing these interaction patterns in the next development cycle");
    }
    if medium > 0 {
        steps.push("⚠️  MEDIUM: Address these patterns after resolving critical and high-risk issues");
    }
    if critical == 0 && high == 0 && medium == 0 {
        steps.push("✅ No significant interaction risks found - maintain regular security reviews");
    }

    steps.join("\n")
}

fn count_severity(text: &str, severity: &str) -> usize {
    text.lines()
        .filter(|line| line.contains(severity))
        .count()
}