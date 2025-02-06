use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start an interactive chat session
    Chat,
    /// Send a single query and get a response
    Query {
        /// The prompt to send to GPT-3.5
        #[arg(value_name = "PROMPT")]
        prompt: String,
    },
    /// Analyze Stylus code for optimizations and security
    Analyze {
        /// Path to the Stylus source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Analysis type: gas, memory, size, security, or all
        #[arg(short = 't', long = "analysis-type", value_name = "TYPE", default_value = "all")]
        analysis_type: String,
        /// Enable detailed memory usage analysis
        #[arg(long)]
        memory_details: bool,
        /// Show Solidity vs Stylus comparison metrics
        #[arg(long)]
        compare_solidity: bool,
    },
}