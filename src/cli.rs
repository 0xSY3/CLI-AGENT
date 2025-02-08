use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "stylus-analyzer")]
#[command(about = "AI-powered Arbitrum Stylus smart contract analyzer", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze gas usage in the contract
    Analyze {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
    /// Perform comprehensive security audit
    Audit {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
    /// Analyze contract size
    Size {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
    /// Perform security analysis
    Secure {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
    /// Generate comprehensive report
    Report {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
    /// Analyze upgrade patterns
    Upgrade {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
    /// Analyze function complexity
    Complexity {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
    /// Analyze cross-contract interactions
    Interactions {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
    /// Analyze code quality metrics
    Quality {
        /// Path to the Stylus contract file
        file: PathBuf,
    },
}