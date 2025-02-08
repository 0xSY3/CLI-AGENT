use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;
use crate::parser::ParsedContract;

#[derive(Debug)]
pub struct SizeAnalysis {
    pub issues: String,
    pub suggestions: String,
}

pub struct SizeAnalyzer;

#[async_trait::async_trait]
impl Analyzer for SizeAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(file)?;
        let parsed = ParsedContract::new(content.clone());

        println!("📏 Analyzing contract with {} functions and {} structs...", 
                parsed.function_count(), parsed.struct_count());
        println!("⏳ Please wait while we process your contract...\n");

        let analysis = ai::analyze_contract_size(&content).await?;
        Ok(format!("{}\n{}", analysis.issues, analysis.suggestions))
    }

    fn format_output(&self, analysis: &str) -> String {
        format!(
            "\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n",
            "📊 Contract Size Analysis Report".bright_green().bold(),
            "════════════════════════════".bright_green(),
            "🔍 Size Issues:".yellow().bold(),
            format_issues(analysis),
            "💡 Optimization Suggestions:".yellow().bold(),
            format_suggestions(analysis),
            format_summary(analysis)
        )
    }
}

fn format_issues(issues: &str) -> String {
    issues
        .lines()
        .map(|line| {
            if line.contains("Critical") {
                format!("🚨 {}", line.red().bold())
            } else if line.contains("Major") {
                format!("⚠️  {}", line.yellow().bold())
            } else if line.contains("Medium") {
                format!("📝 {}", line.yellow())
            } else if line.contains("Minor") {
                format!("✅ {}", line.green())
            } else if line.contains("Analysis:") || line.contains("Size Contributors:") {
                let separator = format!("{}", "─".repeat(50));
                format!("\n┌{}\n{}\n", separator, line.cyan().bold())
            } else if line.trim().ends_with(":") {
                let separator = format!("{}", "─".repeat(30));
                format!("\n└{}\n  {}", separator, line.yellow().bold())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_suggestions(suggestions: &str) -> String {
    suggestions
        .lines()
        .map(|line| {
            if line.contains("Estimated reduction:") {
                format!("📉 {}", line.green().bold())
            } else if line.contains("Example:") {
                format!("\n📝 {}", line.cyan().italic())
            } else if line.trim().ends_with(":") {
                format!("\n{}\n", line.yellow().bold())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_summary(issues: &str) -> String {
    let critical_count = count_severity(issues, "Critical");
    let major_count = count_severity(issues, "Major");
    let medium_count = count_severity(issues, "Medium");
    let minor_count = count_severity(issues, "Minor");

    format!(
        "{}\n{}\n\n{}\n{}\n{}\n{}\n",
        "📈 Size Analysis Summary".bright_yellow().bold(),
        "═══════════════════".bright_yellow(),
        format!("🚨 Critical Size Issues: {}", critical_count).red().bold(),
        format!("⚠️  Major Size Issues: {}", major_count).yellow().bold(),
        format!("📝 Medium Size Issues: {}", medium_count).yellow(),
        format!("✅ Minor Size Issues: {}", minor_count).green()
    )
}

fn count_severity(text: &str, severity: &str) -> usize {
    text.lines()
        .filter(|line| line.contains(severity))
        .count()
}