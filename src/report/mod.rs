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
use crate::parser::ParsedContract;

pub async fn generate_full_report(file: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
    println!("\n🤖 Starting AI-Powered Smart Contract Analysis...");
    println!("📝 Loading analyzers and preparing context...\n");

    let contract = ParsedContract::new(std::fs::read_to_string(file)?)?;
    let patterns = contract.analyze_patterns();
    let gas_patterns = contract.analyze_gas_patterns();

    println!("🔍 Running deep analysis with multiple AI agents...\n");

    let analyzers: Vec<(&str, Box<dyn Analyzer>)> = vec![
        ("Gas Optimization", Box::new(GasAnalyzer)),
        ("Contract Size", Box::new(SizeAnalyzer)),
        ("Security", Box::new(SecurityAnalyzer)),
        ("Complexity", Box::new(ComplexityAnalyzer)),
        ("Cross-Contract Interactions", Box::new(InteractionsAnalyzer)),
        ("Code Quality", Box::new(QualityAnalyzer)),
    ];

    let mut reports = Vec::new();
    for (name, analyzer) in analyzers {
        println!("🧠 AI Agent analyzing {name}...");
        let content = analyzer.analyze(file).await?;
        reports.push((name, content));
    }

    println!("\n✨ Analysis complete! Generating comprehensive report...\n");

    let report = format!(
        "{}\n{}\n\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}",
        "===========================================".bright_green(),
        "🤖 AI-Powered Smart Contract Analysis Report".bright_green().bold(),
        format_executive_summary(&reports),
        "🔍 Smart Contract Patterns".bright_yellow().bold(),
        format_patterns(&patterns),
        "⚡ Gas Usage Patterns".bright_yellow().bold(),
        format_gas_patterns(&gas_patterns),
        format_detailed_analysis(&reports)
    );

    Ok(report)
}

fn format_executive_summary(reports: &[(&str, String)]) -> String {
    let mut summary = String::new();
    summary.push_str(&format!("{}\n{}\n\n", 
        "Executive Summary".bright_yellow().bold(),
        "----------------".bright_yellow()));

    // Risk Score calculation based on findings
    let risk_score = calculate_risk_score(reports);
    summary.push_str(&format!("🎯 Overall Risk Score: {}/10\n", 
        if risk_score > 7.0 { risk_score.to_string().red() }
        else if risk_score > 4.0 { risk_score.to_string().yellow() }
        else { risk_score.to_string().green() }));

    // Key findings summary
    summary.push_str("\n🔑 Key Findings:\n");
    for (category, content) in reports {
        let severity = get_highest_severity(content);
        summary.push_str(&format!("• {}: {}\n", 
            category,
            format_severity(&severity)));
    }

    summary.push_str("\n💡 AI Recommendations:\n");
    let recommendations = extract_recommendations(reports);
    for rec in recommendations.iter().take(3) {
        summary.push_str(&format!("• {}\n", rec));
    }

    summary
}

fn calculate_risk_score(reports: &[(&str, String)]) -> f32 {
    let mut score: f32 = 10.0;
    for (_, content) in reports {
        if content.contains("Critical") { score -= 2.0; }
        else if content.contains("High") { score -= 1.0; }
        else if content.contains("Medium") { score -= 0.5; }
    }
    score.max(0.0)
}

fn format_severity(severity: &str) -> colored::ColoredString {
    match severity {
        "Critical" => "Critical Issues Found".red().bold(),
        "High" => "High Risk Areas".yellow().bold(),
        "Medium" => "Medium Concerns".yellow(),
        _ => "Low/No Issues".green(),
    }
}

fn get_highest_severity(content: &str) -> String {
    if content.contains("Critical") {
        "Critical".to_string()
    } else if content.contains("High") {
        "High".to_string()
    } else if content.contains("Medium") {
        "Medium".to_string()
    } else {
        "Low".to_string()
    }
}

fn format_patterns(patterns: &[String]) -> String {
    if patterns.is_empty() {
        "No significant patterns detected.".dimmed().to_string()
    } else {
        patterns.iter()
            .map(|p| format!("• {}", p))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn format_gas_patterns(patterns: &[String]) -> String {
    if patterns.is_empty() {
        "No gas-specific patterns detected.".dimmed().to_string()
    } else {
        patterns.iter()
            .map(|p| format!("• {}", p))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn format_detailed_analysis(reports: &[(&str, String)]) -> String {
    let mut analysis = String::new();

    for (category, content) in reports {
        analysis.push_str(&format!("\n{}\n{}\n{}\n",
            "─".repeat(50).bright_blue(),
            format!("[ 🤖 AI Analysis: {} ]", category).bright_yellow().bold(),
            "─".repeat(50).bright_blue()));

        // Format the content with improved readability
        for line in content.lines() {
            if line.contains("Critical") {
                analysis.push_str(&format!("🚨 {}\n", line.red().bold()));
            } else if line.contains("High") {
                analysis.push_str(&format!("⚠️  {}\n", line.yellow().bold()));
            } else if line.contains("Medium") {
                analysis.push_str(&format!("ℹ️  {}\n", line.blue()));
            } else if line.contains("Recommendation") {
                analysis.push_str(&format!("💡 {}\n", line.green()));
            } else if !line.trim().is_empty() {
                analysis.push_str(&format!("  {}\n", line));
            }
        }

        // Add AI insights section
        analysis.push_str("\n🤖 AI Agent Insights:\n");
        let insights = extract_ai_insights(content);
        for insight in insights {
            analysis.push_str(&format!("  • {}\n", insight.cyan()));
        }
    }

    analysis
}

fn extract_recommendations(reports: &[(&str, String)]) -> Vec<String> {
    let mut recommendations = Vec::new();
    for (_, content) in reports {
        for line in content.lines() {
            if line.contains("Recommendation") || line.contains("suggestion") {
                recommendations.push(line.trim().to_string());
            }
        }
    }
    recommendations
}

fn extract_ai_insights(content: &str) -> Vec<String> {
    let mut insights = Vec::new();
    let mut in_insights = false;

    for line in content.lines() {
        if line.contains("AI Insights:") {
            in_insights = true;
            continue;
        }
        if in_insights && line.starts_with('-') {
            insights.push(line[1..].trim().to_string());
        }
    }

    // If no explicit insights section, generate some based on the content
    if insights.is_empty() {
        if content.contains("storage") {
            insights.push("Contract uses storage operations - consider optimization potential".to_string());
        }
        if content.contains("event") {
            insights.push("Event emissions detected - good for off-chain monitoring".to_string());
        }
        if content.contains("modifier") {
            insights.push("Access control modifiers present - security best practice".to_string());
        }
    }

    insights
}