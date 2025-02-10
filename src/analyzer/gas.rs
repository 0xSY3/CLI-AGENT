use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;
use crate::parser::ParsedContract;
use crate::parser::ContractType;

pub struct GasAnalyzer;

#[async_trait::async_trait]
impl Analyzer for GasAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
        println!("\n🔍 Starting Stylus Contract Analysis...");

        let content = fs::read_to_string(file)?;
        let parsed = ParsedContract::new(content.clone())?;

        // Initialize AI context with contract type
        let mut context = crate::ai::AnalysisContext::new();
        context.contract_type = match parsed.contract_type {
            ContractType::Solidity => "Solidity".to_string(),
            ContractType::Stylus => "Stylus".to_string(),
        };

        println!("⚡ Analyzing gas patterns...");
        let analysis = ai::analyze_gas_usage(&content).await?;

        let contract_patterns = parsed.analyze_patterns();
        let gas_patterns = parsed.analyze_gas_patterns();

        // Enhanced analysis with L2-specific insights
        let l2_analysis = analyze_l2_patterns(&content);
        let stylus_patterns = format_stylus_patterns(&analysis, &parsed);
        let memory_analysis = analyze_memory_patterns(&content);
        let environmental = format_environmental_impact(&analysis);
        let recommendations = generate_recommendations(&contract_patterns, &gas_patterns, &parsed);
        let summary = format_summary(&analysis);

        println!("📊 Generating final report...");
        println!("✨ Analysis complete!\n");

        // Include follow-up questions and improvements in the report
        let follow_ups = self.get_follow_up_questions(&analysis, &parsed)
            .iter()
            .map(|q| format!("❓ {}", q))
            .collect::<Vec<_>>()
            .join("\n");

        let improvements = self.get_suggested_improvements(&analysis, &parsed)
            .iter()
            .map(|i| format!("💡 {}", i))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            format_l2_metrics(&analysis),
            l2_analysis,
            stylus_patterns,
            memory_analysis,
            environmental,
            recommendations,
            summary,
            format!("🤔 Follow-up Questions:\n{}", follow_ups),
            format!("✨ Suggested Improvements:\n{}", improvements)
        ))
    }

    fn get_follow_up_questions(&self, analysis: &str, parsed: &ParsedContract) -> Vec<String> {
        let mut questions = Vec::new();

        // Gas-specific follow-up questions
        if analysis.contains("storage") {
            questions.push("Would you like to see examples of optimized storage patterns?".to_string());
        }

        if analysis.contains("cross-chain") {
            questions.push("Should I explain the L1/L2 cross-chain optimizations in more detail?".to_string());
        }

        if analysis.contains("batch") {
            questions.push("Would you like examples of efficient batch processing patterns?".to_string());
        }

        // Stylus-specific questions
        match parsed.contract_type {
            ContractType::Stylus => {
                questions.push("Would you like to see Rust-specific optimization examples?".to_string());
                questions.push("Should I explain the Stylus storage patterns in more detail?".to_string());
            }
            ContractType::Solidity => {
                questions.push("Would you like to see how this contract could be optimized for Arbitrum?".to_string());
                questions.push("Should I show you the Stylus equivalent of this contract?".to_string());
            }
        }

        questions
    }

    fn get_suggested_improvements(&self, analysis: &str, parsed: &ParsedContract) -> Vec<String> {
        let mut improvements = Vec::new();

        // Gas-specific improvements
        if analysis.contains("storage") {
            improvements.push("Consider implementing caching mechanisms for frequently accessed storage".to_string());
            improvements.push("Review storage slot packing opportunities".to_string());
        }

        if analysis.contains("event") {
            improvements.push("Review and optimize event emission patterns".to_string());
            improvements.push("Consider selective event parameter indexing".to_string());
        }

        if analysis.contains("loop") {
            improvements.push("Consider implementing batch operations for arrays".to_string());
            improvements.push("Evaluate loop termination conditions for gas efficiency".to_string());
        }

        // Contract-type specific improvements
        match parsed.contract_type {
            ContractType::Stylus => {
                improvements.push("Use Rust's zero-copy optimizations where possible".to_string());
                improvements.push("Implement efficient error handling with Result types".to_string());
                improvements.push("Consider using fixed-size arrays where applicable".to_string());
            }
            ContractType::Solidity => {
                improvements.push("Consider converting to Stylus for L2 optimization".to_string());
                improvements.push("Implement assembly blocks for hot paths".to_string());
                improvements.push("Use unchecked blocks for arithmetic operations".to_string());
            }
        }

        improvements
    }
}

fn format_l2_metrics(operations: &str) -> String {
    let mut formatted = String::new();
    formatted.push_str("\n🚀 Stylus Optimization Summary\n");
    formatted.push_str("═══════════════════════════\n");

    let patterns = analyze_optimization_patterns(operations);
    for pattern in patterns {
        match pattern.severity.as_str() {
            "Critical" => formatted.push_str(&format!("❌ {}\n", pattern.message.red().bold())),
            "High" => formatted.push_str(&format!("⚠️ {}\n", pattern.message.yellow().bold())),
            "Medium" => formatted.push_str(&format!("ℹ️ {}\n", pattern.message.blue())),
            _ => formatted.push_str(&format!("✅ {}\n", pattern.message.green())),
        }
    }

    formatted.push_str("\n\n");
    formatted
}

#[derive(Debug)]
struct OptimizationPattern {
    severity: String,
    message: String,
}

fn analyze_optimization_patterns(content: &str) -> Vec<OptimizationPattern> {
    let mut patterns = Vec::new();

    // L2-specific patterns
    if content.contains("cross-chain") || content.contains("L1") {
        patterns.push(OptimizationPattern {
            severity: "High".to_string(),
            message: "Cross-chain operations detected - optimize for L2 efficiency".to_string(),
        });
    }

    // Storage patterns
    if content.contains("storage") && content.contains("frequent") {
        patterns.push(OptimizationPattern {
            severity: "Medium".to_string(),
            message: "Frequent storage operations - consider caching or batching".to_string(),
        });
    }

    // Memory patterns
    if content.contains("memory[]") || content.contains("new bytes") {
        patterns.push(OptimizationPattern {
            severity: "Medium".to_string(),
            message: "Dynamic memory allocation - consider fixed-size alternatives".to_string(),
        });
    }

    // Loop patterns
    if content.contains("for") || content.contains("while") {
        patterns.push(OptimizationPattern {
            severity: "Medium".to_string(),
            message: "Loop operations - evaluate gas limits and batch processing".to_string(),
        });
    }

    // Event patterns
    if content.contains("emit") && !content.contains("indexed") {
        patterns.push(OptimizationPattern {
            severity: "Low".to_string(),
            message: "Non-indexed events - consider selective indexing for efficiency".to_string(),
        });
    }

    patterns
}

fn format_stylus_patterns(analysis: &str, parsed: &ParsedContract) -> String {
    let mut formatted = String::new();
    formatted.push_str("\n🔄 Solidity to Stylus Conversion Guide\n");
    formatted.push_str("══════════════════════════════════\n");

    match parsed.contract_type {
        ContractType::Solidity => {
            formatted.push_str("Convert your Solidity patterns to gas-efficient Stylus code:\n\n");

            // Add storage patterns if detected
            if analysis.contains("storage") {
                formatted.push_str("💡 Storage Pattern Optimization\n");
                formatted.push_str("  Solidity:\n");
                formatted.push_str("    mapping(address => uint256) balances;\n");
                formatted.push_str("  Stylus (More Efficient):\n");
                formatted.push_str("    StorageMap<Address, U256>\n");
                formatted.push_str("  💰 Potential Gas Savings: 30-40%\n\n");
            }

            // Add memory patterns if detected
            if analysis.contains("memory") {
                formatted.push_str("💡 Memory Management\n");
                formatted.push_str("  Solidity:\n");
                formatted.push_str("    bytes memory data;\n");
                formatted.push_str("  Stylus (Zero-Copy):\n");
                formatted.push_str("    &[u8]\n");
                formatted.push_str("  💰 Potential Gas Savings: 20-30%\n\n");
            }

            // Add event patterns if detected
            if analysis.contains("event") {
                formatted.push_str("💡 Event Optimization\n");
                formatted.push_str("  Solidity:\n");
                formatted.push_str("    event Transfer(address indexed from, address to, uint256 value);\n");
                formatted.push_str("  Stylus (Optimized):\n");
                formatted.push_str("    #[event] Transfer(from: Address, to: Address, value: U256)\n");
                formatted.push_str("  💰 Potential Gas Savings: 10-20%\n\n");
            }
        }
        ContractType::Stylus => {
            formatted.push_str("Your contract is already using Stylus! Here are some advanced optimizations:\n\n");
            formatted.push_str("💡 Advanced Stylus Patterns\n");
            formatted.push_str("  1️⃣ Use zero-copy operations with references\n");
            formatted.push_str("  2️⃣ Implement const generics for fixed-size arrays\n");
            formatted.push_str("  3️⃣ Leverage Rust's type system for compile-time guarantees\n");
            formatted.push_str("  4️⃣ Use enum variants for error handling\n");
        }
    }

    formatted
}

fn analyze_memory_patterns(content: &str) -> String {
    let mut analysis = String::new();
    analysis.push_str("\n🧠 Memory Usage Analysis\n");
    analysis.push_str("══════════════════════\n\n");

    let mut concerns = Vec::new();
    let mut suggestions = Vec::new();

    // Memory allocation patterns
    if content.contains("memory") || content.contains("calldata") {
        concerns.push(("Memory Operations", "High",
            "Inefficient memory usage in parameters and variables"));
        suggestions.push("Use calldata for read-only parameters");
        suggestions.push("Minimize memory allocations in loops");
    }

    // Dynamic allocation patterns
    if content.contains("new") || content.contains("alloc") {
        concerns.push(("Dynamic Memory", "High",
            "Dynamic allocations increase gas costs"));
        suggestions.push("Use fixed-size data structures");
        suggestions.push("Pre-allocate arrays when possible");
    }

    // Storage access patterns
    if content.contains("storage") || content.contains("mapping") {
        concerns.push(("Storage Interaction", "Medium",
            "Frequent storage access detected"));
        suggestions.push("Cache frequently accessed values");
        suggestions.push("Batch storage operations");
    }

    // Format the output
    if concerns.is_empty() {
        analysis.push_str("✅ Memory usage is well optimized\n");
        analysis.push_str("  • Continue monitoring memory patterns\n");
    } else {
        // Format concerns
        for (issue, severity, description) in concerns {
            let (icon, colored_text) = match severity {
                "High" => ("⚠️", format!("{}", issue).red().bold()),
                _ => ("ℹ️", format!("{}", issue).blue()),
            };
            analysis.push_str(&format!("{} {} ({} Impact)\n  • {}\n\n", 
                icon, colored_text, severity, description));
        }

        // Add suggestions if present
        if !suggestions.is_empty() {
            analysis.push_str("💡 Recommendations:\n");
            for suggestion in suggestions.into_iter().take(3) { // Limit to top 3 suggestions
                analysis.push_str(&format!("  • {}\n", suggestion));
            }
        }
    }

    analysis
}

fn format_environmental_impact(analysis: &str) -> String {
    // Updated CO2 calculations based on more accurate estimates
    let co2_per_gas = 0.0000002; // kg CO2 per gas unit (refined estimate)
    let total_gas = extract_total_gas(analysis);
    let total_co2 = total_gas as f64 * co2_per_gas;

    // Energy calculation improvements
    let energy_kwh = total_gas as f64 * 0.000001; // kWh per gas unit

    // Enhanced comparisons for better understanding
    let (co2_comparison, energy_comparison) = if total_co2 > 0.5 {
        ("❗ High environmental impact - consider optimizing gas usage".red().to_string(),
         format!("🔸 Energy equivalent to {} smartphone charges", (energy_kwh * 100.0) as u64).yellow())
    } else if total_co2 > 0.1 {
        ("⚠️ Medium environmental impact - optimization recommended".yellow().to_string(),
         format!("🔸 Energy equivalent to {} hours of laptop usage", (energy_kwh * 50.0) as u64).yellow())
    } else {
        ("✅ Low environmental impact - good optimization".green().to_string(),
         format!("🔹 Energy equivalent to {} LED bulb hours", (energy_kwh * 200.0) as u64).green())
    };

    format!(
        "\n🌱 Environmental Impact\n{}\n\n{}\n{}\n{}\n{}\n{}\n\n{}\n",
        "═".repeat(35).bright_yellow(),
        format!("⚡ Gas Usage: {} units", total_gas.to_string().green()),
        format!("💨 CO2 Emission: {:.4} kg", total_co2),
        format!("🔋 Energy Consumption: {:.4} kWh", energy_kwh),
        co2_comparison,
        energy_comparison,
        "Note: Estimates based on average network conditions".bright_black()
    )
}

fn extract_total_gas(analysis: &str) -> u64 {
    let base_cost = 21000; // Base transaction cost
    let mut total_gas = base_cost;

    // Core operation costs
    if analysis.contains("storage write") {
        total_gas += 5000;
    }
    if analysis.contains("event emission") {
        total_gas += 1000;
    }
    if analysis.contains("external call") {
        total_gas += 2500;
    }
    if analysis.contains("memory allocation") {
        total_gas += 1500;
    }
    if analysis.contains("array operation") {
        total_gas += 3000;
    }

    // Stylus-specific costs
    if analysis.contains("wasm") {
        total_gas += 800; // Reduced from 1000 based on Stylus optimization
    }
    if analysis.contains("precompile") {
        total_gas += 400; // Reduced from 500 based on Stylus optimization
    }

    // L2 specific adjustments
    total_gas = (total_gas as f64 * 0.9) as u64; // 10% reduction for L2

    total_gas
}

fn generate_recommendations(patterns: &[String], gas_patterns: &[String], parsed: &ParsedContract) -> String {
    let mut recommendations = String::new();
    recommendations.push_str("\n🔍 Layer 2 Optimization Recommendations:\n");

    // Add contract-specific recommendations first
    match parsed.contract_type {
        ContractType::Stylus => {
            recommendations.push_str("Rust-Specific Optimizations:\n");
            recommendations.push_str("  • Use const generics for fixed-size arrays\n");
            recommendations.push_str("  • Implement zero-copy operations where possible\n");
            recommendations.push_str("  • Leverage Rust's type system for safety\n");
        }
        ContractType::Solidity => {
            recommendations.push_str("Solidity-to-Stylus Migration:\n");
            recommendations.push_str("  • Consider converting to Stylus for L2 benefits\n");
            recommendations.push_str("  • Use assembly for critical paths\n");
            recommendations.push_str("  • Optimize calldata encoding\n");
        }
    }

    // Add pattern-based recommendations
    let mut storage_recommendations = Vec::new();
    let mut gas_recommendations = Vec::new();
    let mut event_recommendations = Vec::new();

    for pattern in patterns {
        if pattern.contains("storage") {
            storage_recommendations.push("  • Optimize storage access patterns");
            storage_recommendations.push("  • Consider memory caching");
            storage_recommendations.push("  • Review storage slot packing");
        }
        if pattern.contains("gas") {
            gas_recommendations.push("  • Use unchecked blocks where safe");
            gas_recommendations.push("  • Optimize loop conditions");
        }
        if pattern.contains("event") {
            event_recommendations.push("  • Review event emission strategy");
            event_recommendations.push("  • Consider selective indexing");
        }
    }

    // Add gas-specific recommendations
    for pattern in gas_patterns {
        if pattern.contains("batch") {
            gas_recommendations.push("  • Implement batch processing");
        }
    }

    // Format recommendations by category
    if !storage_recommendations.is_empty() {
        recommendations.push_str("\nStorage Optimizations:\n");
        recommendations.push_str(&storage_recommendations.join("\n"));
    }

    if !gas_recommendations.is_empty() {
        recommendations.push_str("\nGas Optimizations:\n");
        recommendations.push_str(&gas_recommendations.join("\n"));
    }

    if !event_recommendations.is_empty() {
        recommendations.push_str("\nEvent Optimizations:\n");
        recommendations.push_str(&event_recommendations.join("\n"));
    }

    recommendations
}

fn format_summary(operations: &str) -> String {
    let critical_count = count_severity(operations, "Critical");
    let high_count = count_severity(operations, "High");
    let medium_count = count_severity(operations, "Medium");
    let low_count = count_severity(operations, "Low");

    // Calculate overall severity based on findings
    let severity_status = if critical_count > 0 {
        "Critical: Address vulnerabilities immediately"
    } else if high_count > 0 {
        "High: Important optimizations needed"
    } else if medium_count > 0 {
        "Medium: Consider improvements"
    } else {
        "Low: Well optimized"
    };

    let mut summary = String::new();
    summary.push_str(&format!("\n{}\n{}\n\n",
        "📈 Gas Optimization Summary".bright_yellow().bold(),
        "═".repeat(35).bright_yellow(),
    ));

    summary.push_str(&format!("💥 Critical Impact: {} issues\n", critical_count).red().bold());
    summary.push_str(&format!("🚨 High Impact: {} issues\n", high_count).red());
    summary.push_str(&format!("⚠️  Medium Impact: {} issues\n", medium_count).yellow());
    summary.push_str(&format!("✅ Low Impact: {} issues\n", low_count).green());

    summary.push_str(&format!("\n{}\n", "🎯 Next Steps:".bright_yellow().bold()));
    match severity_status {
        s if s.starts_with("Critical") => {
            summary.push_str("❗ Address critical gas optimizations immediately\n");
            summary.push_str("  • Focus on L2-specific optimizations first\n");
            summary.push_str("  • Review memory management patterns\n");
        },
        s if s.starts_with("High") => {
            summary.push_str("⚠️ Important optimizations required\n");
            summary.push_str("  • Implement suggested gas saving patterns\n");
            summary.push_str("  • Consider L2-specific improvements\n");
        },
        s if s.starts_with("Medium") => {
            summary.push_str("📝 Schedule medium-impact improvements\n");
            summary.push_str("  • Review code for optimization opportunities\n");
            summary.push_str("  • Consider batch operations where possible\n");
        },
        _ => {
            summary.push_str("✅ Contract is well-optimized\n");
            summary.push_str("  • Continue monitoring gas usage\n");
            summary.push_str("  • Watch for new optimization patterns\n");
        }
    };

    summary
}

fn count_severity(operations: &str, severity: &str) -> usize {
    operations.lines()
        .filter(|line| {
            let contains_severity = line.contains(severity);
            let is_header = line.contains("Impact:") || 
                          line.contains("Summary") || 
                          line.contains("═");
            let is_suggestion = line.contains("Consider") || 
                              line.contains("Implement");
            contains_severity && !is_header && !is_suggestion
        })
        .count()
}

fn analyze_l2_patterns(content: &str) -> String {
    let mut analysis = String::new();
    analysis.push_str("\n🚀 Layer 2 Optimization Analysis\n");
    analysis.push_str("═══════════════════════════\n\n");

    let mut insights = Vec::new();

    // Cross-chain patterns
    if content.contains("L1") || content.contains("L2") || content.contains("bridge") {
        insights.push((
            "Cross-Chain Communication",
            "High",
            "• Optimize message encoding and batching\n  • Use efficient cross-chain calls\n  • Implement retry mechanisms"
        ));
    }

    // Storage optimization
    if content.contains("storage") || content.contains("mapping") {
        insights.push((
            "Storage Optimization",
            "Medium",
            "• Use StorageMap for efficient access\n  • Implement caching strategies\n  • Optimize slot packing"
        ));
    }

    // WASM/Stylus specific
    if content.contains("wasm") || content.contains("precompile") {
        insights.push((
            "WASM Optimization",
            "High",
            "• Use Rust zero-copy types\n  • Leverage WASM precompiles\n  • Minimize memory operations"
        ));
    }

    // Gas optimization
    if content.contains("gas") || content.contains("calldata") {
        insights.push((
            "L2 Gas Optimization",
            "Medium",
            "• Optimize calldata encoding\n  • Use compact data structures\n  • Implement batching"
        ));
    }

    // Format insights
    if insights.is_empty() {
        analysis.push_str("✅ No specific L2 concerns detected\n");
        analysis.push_str("  • Consider L2-aware features\n");
        analysis.push_str("  • Monitor gas costs\n");
    } else {
        for (category, severity, details) in &insights {
            let severity_icon = match *severity {
                "High" => "🚨",
                "Medium" => "⚠️",
                _ => "ℹ️",
            };

            analysis.push_str(&format!("{} {} ({}):\n{}\n\n",
                severity_icon,
                category,
                severity,
                details
            ));
        }
    }

    analysis
}