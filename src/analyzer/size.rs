use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;
use crate::parser::ParsedContract;

pub struct SizeAnalyzer;

#[async_trait::async_trait]
impl Analyzer for SizeAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
        let content = fs::read_to_string(file)?;
        let parsed = ParsedContract::new(content.clone())?;

        println!("📏 Analyzing contract with {} functions and {} structs...", 
                parsed.function_count(), parsed.struct_count());
        println!("⏳ Please wait while we process your contract...\n");

        let analysis = ai::analyze_contract_size(&content).await?;

        // Enhanced L2-specific size analysis
        let mut total_size = 0;
        let mut component_sizes = Vec::new();

        // Analyze component sizes
        if let Ok(func_size) = parsed.get_function_size() {
            total_size += func_size;
            component_sizes.push(("Functions", func_size));
        }
        if let Ok(storage_size) = parsed.get_storage_size() {
            total_size += storage_size;
            component_sizes.push(("Storage", storage_size));
        }
        if let Ok(event_size) = parsed.get_event_size() {
            total_size += event_size;
            component_sizes.push(("Events", event_size));
        }

        Ok(format!(
            "\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n",
            "📊 Contract Size Analysis Report".bright_green().bold(),
            "════════════════════════════".bright_green(),
            "🔍 Size Metrics:".yellow().bold(),
            format_metrics(&component_sizes, total_size),
            "🔍 Size Issues:".yellow().bold(),
            format_issues(&analysis),
            "💡 Optimization Suggestions:".yellow().bold(),
            format_suggestions(&analysis),
            format_summary(&analysis, total_size)
        ))
    }
}

fn format_metrics(components: &[(&str, usize)], total: usize) -> String {
    let mut output = String::new();

    output.push_str(&format!("📦 Total Contract Size: {} bytes\n", total));
    output.push_str("════════════════════════\n\n");

    // Format individual components
    for (name, size) in components {
        let percentage = (*size as f64 / total as f64 * 100.0) as u32;
        let bar_length = (percentage as f32 / 2.0) as usize;
        let bar = "█".repeat(bar_length);

        output.push_str(&format!("{}: {} bytes ({}%)\n", name, size, percentage));
        output.push_str(&format!("[{}{}]\n\n", 
            bar.green().to_string(), 
            " ".repeat(50 - bar_length)
        ));
    }

    // Add L2-specific size analysis
    if total > 24576 { // Arbitrum's recommended max size
        output.push_str(&"⚠️ ".yellow().to_string());
        output.push_str("Contract exceeds recommended L2 size limit\n");
        output.push_str("Consider splitting functionality into multiple contracts\n");
    } else {
        output.push_str(&"✅ ".green().to_string());
        output.push_str("Contract size is within L2 recommended limits\n");
    }

    output
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
                format!("\n{}\n", line.cyan().bold())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_suggestions(content: &str) -> String {
    let mut suggestions = content
        .lines()
        .filter(|line| line.contains("suggestion") || line.contains("Recommendation"))
        .map(|line| format!("  • {}", line))
        .collect::<Vec<_>>();

    // Add L2-specific optimization suggestions
    suggestions.push("  • Consider using calldata instead of memory for read-only function parameters".to_string());
    suggestions.push("  • Use events strategically to reduce storage usage".to_string());
    suggestions.push("  • Implement proxy patterns for upgradeable contracts".to_string());
    suggestions.push("  • Optimize struct packing to reduce storage slots".to_string());

    suggestions.join("\n")
}

fn format_summary(content: &str, total_size: usize) -> String {
    let critical_count = count_severity(content, "Critical");
    let major_count = count_severity(content, "Major");
    let medium_count = count_severity(content, "Medium");
    let minor_count = count_severity(content, "Minor");

    // Calculate size-related metrics
    let size_severity = if total_size > 24576 {
        "Critical"
    } else if total_size > 16384 {
        "Major"
    } else if total_size > 8192 {
        "Medium"
    } else {
        "Minor"
    };

    let l2_recommendations = match size_severity {
        "Critical" => "• Immediate action required: Contract exceeds L2 size limits\n  • Split contract into multiple smaller contracts\n  • Consider implementing proxy patterns",
        "Major" => "• Review contract size: Approaching L2 limits\n  • Optimize large functions\n  • Consider removing unused features",
        "Medium" => "• Monitor contract size during development\n  • Look for optimization opportunities\n  • Plan for future growth",
        _ => "• Contract size is well within L2 limits\n  • Continue monitoring size during updates\n  • Consider gas optimization techniques",
    };

    format!(
        "{}\n{}\n\n{}\n{}\n{}\n{}\n\n{}\n{}\n",
        "📈 Size Analysis Summary".bright_yellow().bold(),
        "═══════════════════".bright_yellow(),
        format!("🚨 Critical Size Issues: {}", critical_count).red().bold(),
        format!("⚠️  Major Size Issues: {}", major_count).yellow().bold(),
        format!("📝 Medium Size Issues: {}", medium_count).yellow(),
        format!("✅ Minor Size Issues: {}", minor_count).green(),
        "🎯 L2 Optimization Strategy:".bright_yellow().bold(),
        l2_recommendations
    )
}

fn count_severity(text: &str, severity: &str) -> usize {
    text.lines()
        .filter(|line| line.contains(severity))
        .count()
}