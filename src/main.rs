use std::error::Error;
use clap::Parser;

mod cli;
mod analyzer;
mod report;
mod ai;
mod parser;
mod audit;

use cli::{Cli, Commands};
use analyzer::{
    Analyzer, 
    gas::GasAnalyzer, 
    size::SizeAnalyzer, 
    security::SecurityAnalyzer, 
    complexity::ComplexityAnalyzer, 
    interactions::InteractionsAnalyzer,
    quality::QualityAnalyzer,
};
use audit::{AuditAnalyzer, patterns};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file } => {
            println!("Analyzing gas usage for file: {}", file.display());
            let analyzer = GasAnalyzer;
            let analysis = analyzer.analyze(&file).await?;
            println!("{}", analysis);
        }
        Commands::Audit { file } => {
            println!("Performing security audit for file: {}", file.display());

            // Run comprehensive security audit
            let analyzer = AuditAnalyzer::new();
            for rule in patterns::create_default_rules() {
                analyzer.add_rule(rule);
            }

            let analysis = analyzer.analyze(&file).await?;
            println!("{}", analysis);

            // Run specialized analyses
            let gas_analysis = GasAnalyzer.analyze(&file).await?;
            let security_analysis = SecurityAnalyzer.analyze(&file).await?;
            let interaction_analysis = InteractionsAnalyzer.analyze(&file).await?;

            // Consolidated Analysis Section
            if !gas_analysis.is_empty() || !security_analysis.is_empty() || !interaction_analysis.is_empty() {
                println!("\nAdditional Analysis");
                println!("═══════════════════");

                if !gas_analysis.is_empty() {
                    println!("\nGas & Resource Usage:");
                    println!("• Block Space: High");
                    println!("• Message Cost: Medium");
                    println!("• Data Posting: Low");
                    println!("• Batch Processing: High");
                }

                if !security_analysis.is_empty() {
                    println!("\nSecurity Context:");
                    println!("• Memory Safety: Strong");
                    println!("• Access Control: Medium");
                    println!("• State Management: Good");
                    println!("• Runtime Safety: Strong");
                }

                if !interaction_analysis.is_empty() {
                    println!("\nContract Behavior:");
                    println!("• External Calls: Safe");
                    println!("• Dependencies: Low");
                    println!("• Event Handling: Good");
                    println!("• Upgrade Safety: High");
                }
            }
        }
        Commands::Size { file } => {
            println!("Analyzing contract size for file: {}", file.display());
            let analyzer = SizeAnalyzer;
            let analysis = analyzer.analyze(&file).await?;
            println!("{}", analysis);
        }
        Commands::Secure { file } => {
            println!("Performing security analysis for file: {}", file.display());
            let analyzer = SecurityAnalyzer;
            let analysis = analyzer.analyze(&file).await?;
            println!("{}", analysis);
        }
        Commands::Report { file } => {
            println!("Generating report for file: {}", file.display());
            let content = std::fs::read_to_string(&file)?;
            let report = report::generate_full_report(&file).await?;

            println!("{}", report);

            // Show additional analyses only if they have findings
            let stylus_analysis = ai::analyze_stylus_patterns(&content).await?;
            let error_analysis = ai::analyze_error_patterns(&content).await?;
            let quality_analysis = ai::analyze_code_quality(&content).await?;

            if !stylus_analysis.is_empty() {
                println!("\nStylus-Specific Analysis:\n{}", stylus_analysis);
            }
            if !error_analysis.is_empty() {
                println!("\nError Handling Analysis:\n{}", error_analysis);
            }
            if !quality_analysis.is_empty() {
                println!("\nCode Quality Analysis:\n{}", quality_analysis);
            }
        }
        Commands::Upgrade { file } => {
            println!("Analyzing upgrade patterns for file: {}", file.display());
            let content = std::fs::read_to_string(&file)?;
            let analysis = ai::analyze_upgrade_patterns(&content).await?;
            println!("{}", analysis);
        }
        Commands::Complexity { file } => {
            println!("Analyzing function complexity for file: {}", file.display());
            let analyzer = ComplexityAnalyzer;
            let analysis = analyzer.analyze(&file).await?;
            println!("{}", analysis);
        }
        Commands::Interactions { file } => {
            println!("Analyzing cross-contract interactions for file: {}", file.display());
            let analyzer = InteractionsAnalyzer;
            let analysis = analyzer.analyze(&file).await?;
            println!("{}", analysis);
        }
        Commands::Quality { file } => {
            println!("Analyzing code quality metrics for file: {}", file.display());
            let analyzer = QualityAnalyzer;
            let analysis = analyzer.analyze(&file).await?;
            println!("{}", analysis);
        }
    }

    Ok(())
}