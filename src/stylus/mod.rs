pub mod analyzer;
pub mod gas;
pub mod vulnerability;
pub mod test_gen;

use thiserror::Error;
use std::path::Path;
use std::fs;
use colored::*;
use analyzer::StylusAnalyzer;

#[derive(Error, Debug)]
pub enum StylusError {
    #[error("Failed to parse Stylus code: {0}")]
    ParseError(String),

    #[error("Gas optimization analysis failed: {0}")]
    GasAnalysisError(String),

    #[error("Vulnerability scan failed: {0}")]
    VulnerabilityError(String),

    #[error("Test generation failed: {0}")]
    TestGenError(String),
}

#[derive(Debug)]
pub struct GasOptimization {
    pub line: usize,
    pub description: String,
    pub suggestion: String,
    pub estimated_savings: u64,
}

#[derive(Debug)]
pub struct Vulnerability {
    pub severity: VulnerabilitySeverity,
    pub line: usize,
    pub description: String,
    pub recommendation: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VulnerabilitySeverity {
    High,
    Medium,
    Low,
}

pub fn analyze_code(file_path: &Path, analysis_type: &str) -> Result<String, StylusError> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| StylusError::ParseError(e.to_string()))?;

    let analyzer = StylusAnalyzer::from_string(content.clone());
    let mut report = String::new();

    // Add contract overview section
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

    match analysis_type {
        "gas" => {
            report.push_str(&format!("\n{}\n", "=== Gas Optimization Analysis ===".bold()));
            let optimizations = gas::analyze_gas_usage(content)?;
            if optimizations.is_empty() {
                report.push_str("\nNo gas optimization issues found.\n");
            } else {
                for opt in optimizations {
                    report.push_str(&format!(
                        "\n🔍 Line {}: {}\n💡 Suggestion: {}\n💰 Estimated gas savings: {} wei\n",
                        opt.line.to_string().yellow(),
                        opt.description.red(),
                        opt.suggestion.green(),
                        opt.estimated_savings.to_string().cyan()
                    ));
                }
            }
        }
        "security" => {
            report.push_str(&format!("\n{}\n", "=== Security Vulnerability Analysis ===".bold()));
            let vulnerabilities = vulnerability::scan_vulnerabilities(content)?;
            if vulnerabilities.is_empty() {
                report.push_str("\nNo security vulnerabilities found.\n");
            } else {
                for vuln in vulnerabilities {
                    report.push_str(&format!(
                        "\n⚠️  Line {}: {} Severity - {}\n🛡️  Recommendation: {}\n",
                        vuln.line.to_string().yellow(),
                        format!("{:?}", vuln.severity).red(),
                        vuln.description,
                        vuln.recommendation.green()
                    ));
                }
            }
        }
        "all" => {
            // Gas analysis section
            report.push_str(&format!("\n{}\n", "=== Gas Optimization Analysis ===".bold()));
            let optimizations = gas::analyze_gas_usage(content.clone())?;
            if optimizations.is_empty() {
                report.push_str("\nNo gas optimization issues found.\n");
            } else {
                for opt in optimizations {
                    report.push_str(&format!(
                        "\n🔍 Line {}: {}\n💡 Suggestion: {}\n💰 Estimated gas savings: {} wei\n",
                        opt.line.to_string().yellow(),
                        opt.description.red(),
                        opt.suggestion.green(),
                        opt.estimated_savings.to_string().cyan()
                    ));
                }
            }

            // Security analysis section
            report.push_str(&format!("\n\n{}\n", "=== Security Vulnerability Analysis ===".bold()));
            let vulnerabilities = vulnerability::scan_vulnerabilities(content)?;
            if vulnerabilities.is_empty() {
                report.push_str("\nNo security vulnerabilities found.\n");
            } else {
                for vuln in vulnerabilities {
                    report.push_str(&format!(
                        "\n⚠️  Line {}: {} Severity - {}\n🛡️  Recommendation: {}\n",
                        vuln.line.to_string().yellow(),
                        format!("{:?}", vuln.severity).red(),
                        vuln.description,
                        vuln.recommendation.green()
                    ));
                }
            }
        }
        _ => return Err(StylusError::ParseError("Invalid analysis type".into())),
    }

    Ok(report)
}

pub fn generate_tests(file_path: &Path, test_type: &str) -> Result<String, StylusError> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| StylusError::ParseError(e.to_string()))?;

    test_gen::generate_tests(content, test_type)
}