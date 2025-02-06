mod cli;
mod conversation;
mod error;
mod stylus;

use clap::Parser;
use cli::{Cli, Command};
use conversation::Conversation;
use dotenv::dotenv;
use std::process;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::Chat => {
            let mut conversation = Conversation::new()?;
            conversation.start_interactive().await?;
        }
        Command::Query { prompt } => {
            let mut conversation = Conversation::new()?;
            let response = conversation.single_query(&prompt).await?;
            println!("{}", response);
        }
        Command::Analyze { file, analysis_type, memory_details, compare_solidity } => {
            println!("Starting analysis of file: {:?}", file);

            match stylus::analyze_code(&file, &analysis_type, memory_details, compare_solidity) {
                Ok(analysis) => {
                    if analysis.is_empty() {
                        println!("No issues found in the analysis.");
                    } else {
                        println!("\nAnalysis Results:\n{}", analysis);
                    }
                }
                Err(e) => {
                    eprintln!("Analysis failed: {}", e);
                    process::exit(1);
                }
            }
        }
    }

    Ok(())
}