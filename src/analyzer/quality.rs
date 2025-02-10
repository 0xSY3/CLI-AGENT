use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;
use crate::parser::ParsedContract;

pub struct QualityAnalyzer;

#[async_trait::async_trait]
impl Analyzer for QualityAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
        let content = fs::read_to_string(file)?;
        let _parsed = ParsedContract::new(content.clone())?;

        println!("📊 Analyzing code quality metrics...");
        println!("⏳ Please wait while we process your contract...\n");

        let analysis = ai::analyze_code_quality(&content).await?;

        Ok(format!(
            "\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n",
            "🎯 Code Quality Analysis Report".bright_green().bold(),
            "═══════════════════════════".bright_green(),
            "📊 Quality Metrics Overview:".yellow().bold(),
            format_metrics(&analysis),
            "💡 Best Practices Analysis:".yellow().bold(),
            format_practices(&analysis),
            "⚠️  Areas for Improvement:".yellow().bold(),
            format_improvements(&analysis),
            format_summary(&analysis)
        ))
    }
}

fn format_metrics(analysis: &str) -> String {
    analysis
        .lines()
        .take_while(|line| !line.contains("Best Practices"))
        .map(|line| {
            if line.contains("Excellent") {
                format!("🌟 {}", line.green().bold())
            } else if line.contains("Good") {
                format!("✨ {}", line.green())
            } else if line.contains("Fair") {
                format!("📝 {}", line.yellow())
            } else if line.contains("Poor") {
                format!("⚠️  {}", line.red())
            } else if line.contains("Analysis:") || line.contains("Metrics:") {
                let separator = format!("{}", "─".repeat(50));
                format!("\n┌{}\n{}\n", separator, line.cyan().bold())
            } else if line.trim().ends_with(":") {
                let separator = format!("{}", "─".repeat(30));
                format!("\n└{}\n  {}", separator, line.yellow().bold())
            } else if line.contains("Score:") {
                format!("📈 {}", line.cyan())
            } else if line.contains("Impact:") {
                format!("💥 {}", line.yellow())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_practices(practices: &str) -> String {
    practices
        .lines()
        .skip_while(|line| !line.contains("Best Practices"))
        .take_while(|line| !line.contains("Areas for Improvement"))
        .map(|line| {
            if line.contains("✓") {
                format!("✅ {}", line.green())
            } else if line.contains("!") {
                format!("⚠️  {}", line.yellow())
            } else if line.contains("×") {
                format!("❌ {}", line.red())
            } else if line.contains("Best Practices") || line.contains("Category:") {
                format!("\n{}\n{}\n", line.yellow().bold(), "─".repeat(line.len()).yellow())
            } else if line.trim().ends_with(":") {
                format!("\n{}", line.cyan().bold())
            } else if line.contains("Example:") {
                format!("📝 {}", line.cyan().italic())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_improvements(improvements: &str) -> String {
    improvements
        .lines()
        .skip_while(|line| !line.contains("Areas for Improvement"))
        .map(|line| {
            if line.contains("Critical") {
                format!("🚨 {}", line.red().bold())
            } else if line.contains("Important") {
                format!("⚠️  {}", line.yellow().bold())
            } else if line.contains("Minor") {
                format!("📝 {}", line.green())
            } else if line.contains("Areas for Improvement") {
                format!("\n{}\n{}\n", line.yellow().bold(), "─".repeat(line.len()).yellow())
            } else if line.contains("Solution:") {
                format!("💡 {}", line.green().italic())
            } else if line.contains("Impact:") {
                format!("💥 {}", line.yellow())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_summary(analysis: &str) -> String {
    let excellent_count = count_quality(analysis, "Excellent");
    let good_count = count_quality(analysis, "Good");
    let fair_count = count_quality(analysis, "Fair");
    let poor_count = count_quality(analysis, "Poor");

    format!(
        "{}\n{}\n\n{}\n{}\n{}\n{}\n",
        "📈 Quality Summary".bright_yellow().bold(),
        "════════════════".bright_yellow(),
        format!("🌟 Excellent Metrics: {} found", excellent_count).green().bold(),
        format!("✨ Good Metrics: {} found", good_count).green(),
        format!("📝 Fair Metrics: {} found", fair_count).yellow(),
        format!("⚠️  Poor Metrics: {} found", poor_count).red()
    )
}

fn count_quality(text: &str, quality: &str) -> usize {
    text.lines()
        .filter(|line| line.contains(quality))
        .count()
}