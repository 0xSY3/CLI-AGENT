use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;
use crate::parser::ParsedContract;

#[derive(Debug)]
pub struct GasAnalysis {
    pub operations: String,
    pub recommendations: String,
}

pub struct GasAnalyzer;

#[async_trait::async_trait]
impl Analyzer for GasAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(file)?;
        let parsed = ParsedContract::new(content.clone());
        let analysis = ai::analyze_gas_usage(&content).await?;
        Ok(format!("{}\n{}", analysis.operations, analysis.recommendations))
    }

    fn format_output(&self, analysis: &str) -> String {
        format!(
            "{}\n{}\n\n{}\n",
            analysis.lines()
                .filter(|line| !line.contains("Analyzing") && !line.contains("Please wait"))
                .collect::<Vec<_>>()
                .join("\n"),
            "Gas Optimization Summary".bright_yellow().bold(),
            format_gas_findings(analysis)
        )
    }
}

fn format_gas_findings(analysis: &str) -> String {
    let critical = analysis.lines()
        .filter(|line| line.contains("Critical"))
        .count();
    let high = analysis.lines()
        .filter(|line| line.contains("High"))
        .count();

    let mut findings = String::new();
    if critical > 0 || high > 0 {
        findings.push_str("High Priority:\n");
        findings.push_str("• L1 Data Posting: Optimize\n");
        findings.push_str("• Cross-Chain Messaging: Review\n");
        findings.push_str("• State Management: Improve\n");
    }
    findings
}

fn format_l2_metrics(operations: &str) -> String {
    operations
        .lines()
        .take_while(|line| !line.contains("Memory Management"))
        .map(|line| {
            if line.contains("Critical") {
                format!("💥 {}", line.red().bold())
            } else if line.contains("High") {
                format!("🚨 {}", line.red())
            } else if line.contains("Medium") {
                format!("⚠️  {}", line.yellow())
            } else if line.contains("Low") {
                format!("✅ {}", line.green())
            } else if line.contains("L2-Specific") || line.contains("Arbitrum") || line.contains("Transaction") {
                let separator = format!("{}", "─".repeat(50));
                format!("\n┌{}\n{}\n", separator, line.cyan().bold())
            } else if line.trim().ends_with(":") {
                let separator = format!("{}", "─".repeat(30));
                format!("\n└{}\n  {}", separator, line.yellow().bold())
            } else if line.contains("Gas savings:") {
                format!("💰 {}", line.green().bold())
            } else if line.contains("Impact:") {
                format!("💫 {}", line.yellow())
            } else if line.contains("Example:") {
                format!("📝 {}", line.cyan().italic())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_memory_operations(operations: &str) -> String {
    operations
        .lines()
        .skip_while(|line| !line.contains("Memory Management"))
        .take_while(|line| !line.contains("Storage Optimization"))
        .map(|line| {
            if line.contains("Critical") {
                format!("💥 {}", line.red().bold())
            } else if line.contains("High") {
                format!("🚨 {}", line.red())
            } else if line.contains("Medium") {
                format!("⚠️  {}", line.yellow())
            } else if line.contains("Low") {
                format!("✅ {}", line.green())
            } else if line.contains("Analysis:") || line.contains("Operations:") || line.contains("Pattern") {
                let separator = format!("{}", "─".repeat(50));
                format!("\n┌{}\n{}\n", separator, line.cyan().bold())
            } else if line.trim().ends_with(":") {
                let separator = format!("{}", "─".repeat(30));
                format!("\n└{}\n  {}", separator, line.yellow().bold())
            } else if line.contains("Current cost:") {
                format!("💸 {}", line.yellow())
            } else if line.contains("Optimized cost:") {
                format!("💰 {}", line.green())
            } else if line.contains("Implementation:") {
                format!("🔧 {}", line.cyan())
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
        .skip_while(|line| !line.contains("Storage Optimization"))
        .take_while(|line| !line.contains("Environmental Impact"))
        .map(|line| {
            if line.contains("Estimated savings:") || line.contains("gas savings") {
                format!("💰 {}", line.green().bold())
            } else if line.contains("Implementation:") {
                format!("🔧 {}", line.cyan())
            } else if line.contains("Example:") {
                format!("\n📝 {}", line.cyan().italic())
            } else if line.contains("Current:") {
                format!("📊 {}", line.yellow())
            } else if line.contains("Optimized:") {
                format!("✨ {}", line.green())
            } else if line.contains("Priority:") {
                format!("\n🎯 {}", line.yellow().bold())
            } else if line.trim().ends_with(":") {
                format!("\n{}\n", line.yellow().bold())
            } else {
                format!("  • {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_total_gas(analysis: &str) -> u64 {
    let mut total_gas = 0;

    // Base gas costs
    let base_cost = 21000; // Base transaction cost
    total_gas += base_cost;

    // Extract L2-specific costs
    if analysis.contains("L2 block space") {
        total_gas += 100000; // Estimated L2 block space cost
    }
    if analysis.contains("Cross-chain messaging") {
        total_gas += 50000; // Cross-chain messaging overhead
    }
    if analysis.contains("L1 data posting") {
        total_gas += 80000; // L1 calldata posting cost
    }

    // Add costs based on complexity indicators
    analysis.lines()
        .filter(|line| line.contains("High"))
        .for_each(|_| total_gas += 20000);

    analysis.lines()
        .filter(|line| line.contains("Medium"))
        .for_each(|_| total_gas += 10000);

    total_gas
}

fn format_environmental_recommendations(co2: f64, energy: f64) -> String {
    let mut recommendations = Vec::new();

    let impact = if co2 > 1.0 {
        "🔴 High environmental impact - immediate optimization recommended".to_string()
    } else if co2 > 0.5 {
        "🟡 Medium environmental impact - optimization recommended".to_string()
    } else {
        "🟢 Low environmental impact - continue monitoring".to_string()
    };
    recommendations.push(impact);

    // Add L1 vs L2 comparison
    recommendations.push("\n📊 Network Comparison:".to_string());
    recommendations.push(format!("  • Ethereum L1: {:.2}x more CO2 emissions", (co2 * 100.0).max(1.0)));
    recommendations.push(format!("  • Average L2: {:.2}x typical emissions", (co2 * 2.0).max(0.5)));

    // Add energy efficiency metrics
    recommendations.push("\n⚡ Energy Efficiency:".to_string());
    recommendations.push(format!("  • Power usage: {:.4} kWh per transaction", energy));
    recommendations.push(format!("  • Yearly projection: {:.2} MWh", energy * 525600.0)); // Assuming 1 tx per minute

    // Add optimization suggestions based on impact
    recommendations.push("\n💡 Optimization Potential:".to_string());
    if co2 > 0.5 {
        recommendations.push("  • Implement batching to reduce L1 calldata".to_string());
        recommendations.push("  • Optimize storage patterns to minimize state updates".to_string());
        recommendations.push("  • Use more efficient data structures".to_string());
        recommendations.push("  • Consider implementing lazy evaluation".to_string());
        recommendations.push("  • Reduce cross-chain message frequency".to_string());
    } else {
        recommendations.push("  • Continue monitoring gas usage".to_string());
        recommendations.push("  • Regular efficiency audits recommended".to_string());
    }

    recommendations.join("\n")
}

fn format_environmental_impact(analysis: &str) -> String {
    let co2_per_gas = 0.0000002; // CO2 equivalent in kg per gas unit
    let total_gas = extract_total_gas(analysis);
    let total_co2 = total_gas as f64 * co2_per_gas;
    let energy_kwh = total_gas as f64 * 0.000001; // kWh per gas unit

    format!(
        "\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n",
        "🌍 Carbon Footprint Analysis".cyan().bold(),
        "═".repeat(30),
        "🔍 Gas Usage Analysis:".yellow(),
        format!("  • Total Gas Used: {} units", total_gas.to_string().green()),
        format!("🌱 Environmental Metrics:").yellow(),
        format!("  • CO2 Equivalent: {:.4} kg CO2e\n  • Energy Usage: {:.4} kWh", 
            total_co2, energy_kwh
        ).green(),
        format_environmental_recommendations(total_co2, energy_kwh)
    )
}

fn format_summary(analysis: &str) -> String {
    let critical_count = count_severity(analysis, "Critical");
    let high_count = count_severity(analysis, "High");
    let medium_count = count_severity(analysis, "Medium");
    let low_count = count_severity(analysis, "Low");

    let savings_estimates = extract_savings_estimates(analysis);
    let total_savings = calculate_total_savings(&savings_estimates);

    format!(
        "{}\n{}\n\n{}\n{}\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n",
        "📈 Gas Optimization Summary".bright_yellow().bold(),
        "════════════════════════".bright_yellow(),
        format!("💥 Critical Impact: {} issues", critical_count).red().bold(),
        format!("🚨 High Impact: {} issues", high_count).red(),
        format!("⚠️  Medium Impact: {} issues", medium_count).yellow(),
        format!("✅ Low Impact: {} issues", low_count).green(),
        "💰 Potential Gas Savings:".bright_yellow().bold(),
        format_savings_estimates(&savings_estimates, total_savings),
        "🎯 Next Steps:".bright_yellow().bold(),
        format_next_steps(critical_count, high_count, medium_count)
    )
}

fn extract_savings_estimates(text: &str) -> Vec<(String, u64)> {
    text.lines()
        .filter(|line| line.contains("Estimated savings:") || line.contains("gas savings"))
        .filter_map(|line| {
            if let Some(start) = line.find(char::is_numeric) {
                if let Some(end) = line[start..].find(|c: char| !c.is_numeric()) {
                    if let Ok(savings) = line[start..start + end].parse::<u64>() {
                        return Some((format!("{} ({})", line, calculate_cost_impact(savings)), savings));
                    }
                }
            }
            None
        })
        .collect()
}

fn calculate_total_savings(savings: &[(String, u64)]) -> u64 {
    savings.iter().map(|(_, amount)| amount).sum()
}

fn calculate_cost_impact(gas_amount: u64) -> String {
    let cost_per_gas = 0.000000001; // Example cost in ETH
    let total_eth = (gas_amount as f64) * cost_per_gas;
    format!("{:.8} ETH", total_eth)
}

fn format_savings_estimates(savings: &[(String, u64)], total: u64) -> String {
    let mut result = Vec::new();

    for (description, _amount) in savings {
        result.push(format!("  • {}", description.green()));
    }

    if total > 0 {
        let total_cost = calculate_cost_impact(total);
        result.push(format!("\n💫 Total Potential Savings: {} gas ({})", 
            total.to_string().green().bold(),
            total_cost.cyan()
        ));
    } else {
        result.push("\n💫 Total Potential Savings: Not enough data to calculate".yellow().to_string());
    }

    result.join("\n")
}

fn format_next_steps(critical: usize, high: usize, medium: usize) -> String {
    let mut steps = Vec::new();

    if critical > 0 {
        steps.push("❗ Address critical gas optimizations immediately - these issues significantly impact costs");
    }
    if high > 0 {
        steps.push("⚠️  Plan for high-impact optimizations in the next sprint");
    }
    if medium > 0 {
        steps.push("📝 Schedule medium-impact improvements for future iterations");
    }
    if critical == 0 && high == 0 && medium == 0 {
        steps.push("✅ Contract is well-optimized - continue monitoring gas usage in future updates");
    }

    steps.join("\n")
}

fn count_severity(text: &str, severity: &str) -> usize {
    text.lines()
        .filter(|line| line.contains(severity))
        .count()
}