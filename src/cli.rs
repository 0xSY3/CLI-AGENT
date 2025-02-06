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
    /// Analyze Stylus code for gas optimizations
    Analyze {
        /// Path to the Stylus source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Analysis type: gas, security, or all
        #[arg(long, short)]
        analysis_type: Option<String>,
    },
    /// Generate test cases for a Stylus contract
    GenerateTests {
        /// Path to the Stylus source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Test type: unit, fuzz, or both
        #[arg(long, short)]
        test_type: Option<String>,
    },
}