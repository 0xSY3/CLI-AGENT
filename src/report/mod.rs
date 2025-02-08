use std::path::PathBuf;
use std::error::Error;
use colored::*;
use crate::analyzer::{
    gas::GasAnalyzer,
    size::SizeAnalyzer, 
    security::SecurityAnalyzer,
    complexity::ComplexityAnalyzer,
    interactions::InteractionsAnalyzer,
    quality::QualityAnalyzer
};
use crate::analyzer::Analyzer;

pub async fn generate_full_report(file: &PathBuf) -> Result<String, Box<dyn Error>> {
    let gas_analyzer = GasAnalyzer;
    let size_analyzer = SizeAnalyzer;
    let security_analyzer = SecurityAnalyzer;
    let complexity_analyzer = ComplexityAnalyzer;
    let interactions_analyzer = InteractionsAnalyzer;
    let quality_analyzer = QualityAnalyzer;

    let gas_content = gas_analyzer.analyze(file).await?;
    let size_content = size_analyzer.analyze(file).await?;
    let security_content = security_analyzer.analyze(file).await?;
    let complexity_content = complexity_analyzer.analyze(file).await?;
    let interactions_content = interactions_analyzer.analyze(file).await?;
    let quality_content = quality_analyzer.analyze(file).await?;

    let report = format!(
        "{}\n{}\n\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n",
        "===========================================".bright_green(),
        "Arbitrum Stylus Smart Contract Analysis Report".bright_green().bold(),
        format_summary(&gas_content, &size_content, &security_content, &quality_content),
        format_separator("Gas Analysis"),
        gas_analyzer.format_output(&gas_content),
        format_separator("Contract Size Analysis"),
        size_analyzer.format_output(&size_content),
        format_separator("Security Analysis"),
        security_analyzer.format_output(&security_content),
        format_separator("Complexity Analysis"),
        complexity_analyzer.format_output(&complexity_content),
        format_separator("Cross-Contract Interactions"),
        interactions_analyzer.format_output(&interactions_content),
        format_separator("Quality Analysis"),
        quality_analyzer.format_output(&quality_content)
    );

    Ok(report)
}

fn format_separator(title: &str) -> String {
    format!(
        "\n{}\n{}\n{}\n",
        "─".repeat(50).bright_blue(),
        format!("[ {} ]", title).bright_yellow().bold(),
        "─".repeat(50).bright_blue()
    )
}

fn format_summary(gas: &str, size: &str, security: &str, quality: &str) -> String {
    format!(
        "{}\n{}\n\n{}\n{}\n{}\n{}\n",
        "Executive Summary".bright_yellow().bold(),
        "----------------".bright_yellow(),
        "🔍 Analysis Overview:".bold(),
        format_key_findings(gas, size, security, quality),
        "⚠️  High Priority Items:".bold(),
        extract_critical_items(gas, size, security, quality)
    )
}

fn format_key_findings(gas: &str, size: &str, security: &str, quality: &str) -> String {
    format!(
        "• Gas Optimization: {}\n• Contract Size: {}\n• Security Status: {}\n• Code Quality: {}",
        get_severity_indicator(gas),
        get_severity_indicator(size),
        get_severity_indicator(security),
        get_quality_indicator(quality)
    )
}

fn get_severity_indicator(content: &str) -> String {
    if content.contains("Critical") {
        "🔴 Critical Issues Found".red().bold().to_string()
    } else if content.contains("High") || content.contains("Major") {
        "🟠 High Severity Issues".yellow().bold().to_string()
    } else if content.contains("Medium") {
        "🟡 Medium Severity Issues".yellow().to_string()
    } else {
        "🟢 Low/No Issues".green().to_string()
    }
}

fn get_quality_indicator(content: &str) -> String {
    if content.contains("Excellent") {
        "🌟 Excellent".green().bold().to_string()
    } else if content.contains("Good") {
        "✨ Good".green().to_string()
    } else if content.contains("Fair") {
        "📝 Fair".yellow().to_string()
    } else {
        "⚠️  Needs Improvement".red().to_string()
    }
}

fn extract_critical_items(gas: &str, size: &str, security: &str, quality: &str) -> String {
    let mut critical_items = Vec::new();

    for (section, content) in [
        ("Gas", gas),
        ("Size", size),
        ("Security", security),
        ("Quality", quality),
    ] {
        for line in content.lines() {
            if line.contains("Critical") || line.contains("High") || line.contains("Major") {
                critical_items.push(format!("• [{}] {}", section, line.trim()));
            }
        }
    }

    if critical_items.is_empty() {
        "✅ No high-priority issues detected".green().to_string()
    } else {
        critical_items.join("\n")
    }
}