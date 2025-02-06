use thiserror::Error;
use std::path::Path;
use std::fs;
use colored::*;

#[derive(Error, Debug)]
pub enum StylusError {
    #[error("Failed to parse Stylus code: {0}")]
    ParseError(String),
}

pub mod analyzer;
pub mod gas;
pub mod vulnerability;

pub use analyzer::StylusAnalyzer;
pub use vulnerability::scan_vulnerabilities;

pub fn analyze_code(
    file_path: &Path,
    analysis_type: &str,
    memory_details: bool,
    compare_solidity: bool
) -> Result<String, StylusError> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| StylusError::ParseError(e.to_string()))?;

    let analyzer = StylusAnalyzer::from_string(content.clone());
    let mut report = String::new();

    // Contract Overview
    report.push_str(&format!("\n{}\n", "=== Contract Overview ===".bold()));
    let functions = analyzer.extract_functions();
    let state_vars = analyzer.extract_state_variables();

    report.push_str("\n📋 Public Functions:\n");
    for func in functions {
        report.push_str(&format!("  • {}\n", func.cyan()));
    }

    report.push_str("\n💾 State Variables:\n");
    for var in state_vars {
        report.push_str(&format!("  • {}\n", var.yellow()));
    }

    // Add specific analysis based on type
    let analysis_result = match analysis_type {
        "gas" => {
            let mut gas_report = String::new();
            gas_report.push_str(&format!("\n{}\n", "=== Gas Analysis ===".bold()));
            let gas_analysis = gas::analyze_gas_usage(content.clone())?;

            if gas_analysis.is_empty() {
                gas_report.push_str("\n✅ No gas optimization issues found.\n");
            } else {
                // Group by category
                let mut storage_opts = Vec::new();
                let mut memory_opts = Vec::new();

                for opt in gas_analysis {
                    match opt.category {
                        gas::OptimizationCategory::Storage => storage_opts.push(opt),
                        gas::OptimizationCategory::Memory => memory_opts.push(opt),
                    }
                }

                if !storage_opts.is_empty() {
                    gas_report.push_str("\n📦 Storage Optimizations:\n");
                    for opt in storage_opts {
                        gas_report.push_str(&format!(
                            "Line {}: {}\n  Suggestion: {}\n  Gas Savings: ~{} gas\n\n",
                            opt.line.to_string().yellow(),
                            opt.description,
                            opt.suggestion.green(),
                            opt.estimated_savings
                        ));
                    }
                }

                if !memory_opts.is_empty() {
                    gas_report.push_str("\n💻 Memory Optimizations:\n");
                    for opt in memory_opts {
                        gas_report.push_str(&format!(
                            "Line {}: {}\n  Suggestion: {}\n  Gas Savings: ~{} gas\n\n",
                            opt.line.to_string().yellow(),
                            opt.description,
                            opt.suggestion.green(),
                            opt.estimated_savings
                        ));
                    }
                }
            }

            gas_report
        }
        "memory" => {
            let mut memory_report = String::new();
            memory_report.push_str(&format!("\n{}\n", "=== Memory Analysis ===".bold()));

            let memory_analysis = gas::analyze_memory_usage(content.clone(), memory_details)?;

            if memory_analysis.is_empty() {
                memory_report.push_str("\n✅ No memory optimization issues found.\n");
            } else {
                // Group by pattern type
                use gas::MemoryPatternType::*;
                let mut alloc_intensive = Vec::new();
                let mut nested_iterations = Vec::new();
                let mut temp_allocs = Vec::new();
                let mut unoptimized_cache = Vec::new();

                for analysis in memory_analysis {
                    match analysis.pattern_type {
                        AllocationIntensive => alloc_intensive.push(analysis),
                        NestedIterations => nested_iterations.push(analysis),
                        TemporaryAllocations => temp_allocs.push(analysis),
                        UnoptimizedCaching => unoptimized_cache.push(analysis),
                    }
                }

                if !alloc_intensive.is_empty() {
                    memory_report.push_str("\n🔴 Allocation-Intensive Patterns:\n");
                    for analysis in alloc_intensive {
                        memory_report.push_str(&format!(
                            "Line {}: {}\n  Suggestion: {}\n\n",
                            analysis.line.to_string().yellow(),
                            analysis.description,
                            analysis.suggestion.green()
                        ));
                    }
                }

                if !nested_iterations.is_empty() {
                    memory_report.push_str("\n🔄 Nested Iterations:\n");
                    for analysis in nested_iterations {
                        memory_report.push_str(&format!(
                            "Line {}: {}\n  Suggestion: {}\n\n",
                            analysis.line.to_string().yellow(),
                            analysis.description,
                            analysis.suggestion.green()
                        ));
                    }
                }

                if !temp_allocs.is_empty() {
                    memory_report.push_str("\n⚡ Temporary Allocations:\n");
                    for analysis in temp_allocs {
                        memory_report.push_str(&format!(
                            "Line {}: {}\n  Suggestion: {}\n\n",
                            analysis.line.to_string().yellow(),
                            analysis.description,
                            analysis.suggestion.green()
                        ));
                    }
                }

                if !unoptimized_cache.is_empty() {
                    memory_report.push_str("\n🔍 Unoptimized Caching:\n");
                    for analysis in unoptimized_cache {
                        memory_report.push_str(&format!(
                            "Line {}: {}\n  Suggestion: {}\n\n",
                            analysis.line.to_string().yellow(),
                            analysis.description,
                            analysis.suggestion.green()
                        ));
                    }
                }
            }

            memory_report
        }
        "security" => {
            let mut security_report = String::new();
            security_report.push_str(&format!("\n{}\n", "=== Security Analysis ===".bold()));
            let vulnerabilities = scan_vulnerabilities(content)?;

            if vulnerabilities.is_empty() {
                security_report.push_str("\n✅ No security issues found.\n");
            } else {
                // Group by category
                use vulnerability::VulnerabilityCategory::*;
                let mut reentrancy = Vec::new();
                let mut integer = Vec::new();
                let mut access = Vec::new();
                let mut state = Vec::new();
                let mut data = Vec::new();
                let mut logic = Vec::new();

                for vuln in vulnerabilities {
                    match vuln.category {
                        Reentrancy => reentrancy.push(vuln),
                        IntegerOverflow => integer.push(vuln),
                        AccessControl => access.push(vuln),
                        StateManipulation => state.push(vuln),
                        DataValidation => data.push(vuln),
                        LogicError => logic.push(vuln),
                    }
                }

                if !reentrancy.is_empty() {
                    security_report.push_str("\n🔄 Reentrancy:\n");
                    for vuln in reentrancy {
                        security_report.push_str(&format!(
                            "⚠️  Line {}: {} - {}\n  Recommendation: {}\n\n",
                            vuln.line.to_string().yellow(),
                            format!("{:?}", vuln.severity).red(),
                            vuln.description,
                            vuln.recommendation.green()
                        ));
                    }
                }

                if !integer.is_empty() {
                    security_report.push_str("\n🔢 Integer Overflow/Underflow:\n");
                    for vuln in integer {
                        security_report.push_str(&format!(
                            "⚠️  Line {}: {} - {}\n  Recommendation: {}\n\n",
                            vuln.line.to_string().yellow(),
                            format!("{:?}", vuln.severity).red(),
                            vuln.description,
                            vuln.recommendation.green()
                        ));
                    }
                }

                if !access.is_empty() {
                    security_report.push_str("\n🔐 Access Control:\n");
                    for vuln in access {
                        security_report.push_str(&format!(
                            "⚠️  Line {}: {} - {}\n  Recommendation: {}\n\n",
                            vuln.line.to_string().yellow(),
                            format!("{:?}", vuln.severity).red(),
                            vuln.description,
                            vuln.recommendation.green()
                        ));
                    }
                }

                if !state.is_empty() {
                    security_report.push_str("\n💾 State Management:\n");
                    for vuln in state {
                        security_report.push_str(&format!(
                            "⚠️  Line {}: {} - {}\n  Recommendation: {}\n\n",
                            vuln.line.to_string().yellow(),
                            format!("{:?}", vuln.severity).red(),
                            vuln.description,
                            vuln.recommendation.green()
                        ));
                    }
                }

                if !data.is_empty() {
                    security_report.push_str("\n📝 Data Validation:\n");
                    for vuln in data {
                        security_report.push_str(&format!(
                            "⚠️  Line {}: {} - {}\n  Recommendation: {}\n\n",
                            vuln.line.to_string().yellow(),
                            format!("{:?}", vuln.severity).red(),
                            vuln.description,
                            vuln.recommendation.green()
                        ));
                    }
                }

                if !logic.is_empty() {
                    security_report.push_str("\n⚙️ Logic Issues:\n");
                    for vuln in logic {
                        security_report.push_str(&format!(
                            "⚠️  Line {}: {} - {}\n  Recommendation: {}\n\n",
                            vuln.line.to_string().yellow(),
                            format!("{:?}", vuln.severity).red(),
                            vuln.description,
                            vuln.recommendation.green()
                        ));
                    }
                }
            }

            security_report
        }
        "all" => {
            let mut all_report = String::new();
            let analysis_types = ["gas", "memory", "security"];

            for analysis in &analysis_types {
                let single_analysis = analyze_code(
                    file_path,
                    analysis,
                    memory_details,
                    false  // Don't show Solidity comparison for individual analyses
                )?;

                // Skip contract overview for subsequent analyses
                if let Some(analysis_start) = single_analysis.find("=== ") {
                    let analysis_part = &single_analysis[analysis_start..];
                    if !analysis_part.contains("=== Contract Overview ===") {
                        all_report.push_str(analysis_part);
                    }
                }
            }
            all_report
        }
        _ => {
            String::from("\nPlease specify a valid analysis type (gas/memory/security/all)")
        }
    };

    report.push_str(&analysis_result);

    if compare_solidity {
        report.push_str(&format!("\n{}\n", "=== Solidity Comparison ===".bold()));
        let solidity_comparison = analyzer.compare_with_solidity()?;
        report.push_str(&format!(
            "\n💡 Memory Model Differences:\n{}\n",
            solidity_comparison.memory_differences.green()
        ));
        report.push_str(&format!(
            "🔄 Optimization Opportunities:\n{}\n",
            solidity_comparison.optimization_suggestions.yellow()
        ));
    }

    Ok(report)
}