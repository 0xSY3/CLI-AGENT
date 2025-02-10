use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;

pub struct ComplexityAnalyzer;

#[async_trait::async_trait]
impl Analyzer for ComplexityAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
        let content = fs::read_to_string(file)?;
        println!("🔄 Analyzing function complexity...");
        println!("⏳ Please wait while we process your contract...\n");
        let analysis = ai::analyze_function_complexity(&content).await?;

        Ok(format!(
            "\n{}\n{}\n\n{}\n{}\n{}\n\n{}\n",
            "🔍 Function Complexity Analysis Report".bright_green().bold(),
            "══════════════════════════════════".bright_green(),
            "📊 Complexity Distribution:".yellow().bold(),
            format_overview(&analysis),
            format_metrics(&analysis),
            format_summary(&analysis)
        ))
    }
}

fn format_overview(metrics: &str) -> String {
    format!(
        "{}\n{}\n",
        "📈 Overview".yellow().bold(),
        "═════════".yellow()
    ) + &metrics
        .lines()
        .take_while(|line| !line.contains("Cyclomatic"))
        .map(format_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_metrics(metrics: &str) -> String {
    metrics
        .lines()
        .skip_while(|line| !line.contains("Cyclomatic"))
        .map(|line| {
            if line.contains("High") {
                format!("🚨 {}", line.red().bold())
            } else if line.contains("Medium") {
                format!("⚠️  {}", line.yellow())
            } else if line.contains("Low") {
                format!("✅ {}", line.green())
            } else if line.contains("Cyclomatic") || line.contains("Code Metrics") || line.contains("Parameter Analysis") {
                let separator = format!("{}", "─".repeat(50));
                format!("\n┌{}\n{}\n", separator, line.yellow().bold())
            } else if line.trim().ends_with(":") {
                let separator = format!("{}", "─".repeat(30));
                format!("\n└{}\n  {}", separator, line.cyan().bold())
            } else if line.contains("Function:") {
                format!("\n📝 {}", line.cyan().bold())
            } else if line.contains("Stats:") {
                format!("\n📈 {}", line.yellow())
            } else if line.contains("Recommendation:") {
                format!("\n💡 {}", line.green().italic())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_line(line: &str) -> String {
    if line.contains("Function:") {
        format!("\n📝 {}", line.cyan().bold())
    } else if line.contains("Severity:") {
        let severity = if line.contains("High") {
            "🚨 High".red().bold()
        } else if line.contains("Medium") {
            "⚠️  Medium".yellow()
        } else {
            "✅ Low".green()
        };
        format!("⚖️  Severity: {}", severity)
    } else if line.contains("Impact:") {
        format!("💥 {}", line.yellow())
    } else if line.contains("Optimization:") {
        format!("🔧 {}", line.green().italic())
    } else {
        format!("  • {}", line)
    }
}

fn format_summary(metrics: &str) -> String {
    let high_count = count_severity(metrics, "High");
    let medium_count = count_severity(metrics, "Medium");
    let low_count = count_severity(metrics, "Low");

    format!(
        "{}\n{}\n\n{}\n{}\n{}\n",
        "📊 Complexity Summary".bright_yellow().bold(),
        "══════════════════".bright_yellow(),
        format!("🚨 High Complexity: {} functions", high_count).red().bold(),
        format!("⚠️  Medium Complexity: {} functions", medium_count).yellow(),
        format!("✅ Low Complexity: {} functions", low_count).green()
    )
}

fn count_severity(text: &str, severity: &str) -> usize {
    text.lines()
        .filter(|line| line.contains(severity))
        .count()
}