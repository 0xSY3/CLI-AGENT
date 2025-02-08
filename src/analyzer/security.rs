use std::path::PathBuf;
use std::error::Error;
use std::fs;
use colored::*;
use crate::ai;
use crate::analyzer::Analyzer;

pub struct SecurityAnalyzer;

#[derive(Debug)]
pub struct SecurityAnalysis {
    pub vulnerabilities: String,
    pub recommendations: String,
}

#[async_trait::async_trait]
impl Analyzer for SecurityAnalyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(file)?;
        let analysis = ai::analyze_security_issues(&content).await?;
        Ok(format!("{}\n{}", analysis.vulnerabilities, analysis.recommendations))
    }

    fn format_output(&self, analysis: &str) -> String {
        format!(
            "{}\n{}\n\n{}\n",
            analysis.lines()
                .filter(|line| !line.contains("Performing") && !line.contains("This may") && !line.starts_with('🔒'))
                .collect::<Vec<_>>()
                .join("\n"),
            "Security Summary".bright_yellow().bold(),
            format_security_findings(analysis)
        )
    }
}

fn format_security_findings(analysis: &str) -> String {
    let critical = analysis.lines()
        .filter(|line| line.contains("Critical"))
        .count();
    let high = analysis.lines()
        .filter(|line| line.contains("High"))
        .count();

    let mut findings = String::new();
    if critical > 0 || high > 0 {
        findings.push_str("High Priority:\n");
        findings.push_str("• Access Control: Review\n");
        findings.push_str("• Memory Safety: Verify\n");
        findings.push_str("• Trust Boundaries: Check\n");
    }
    findings
}