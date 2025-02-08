use std::path::PathBuf;
use std::error::Error;

pub mod gas;
pub mod size;
pub mod security;
pub mod complexity;
pub mod interactions;
pub mod quality;

#[async_trait::async_trait]
pub trait Analyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error>>;
    fn format_output(&self, analysis: &str) -> String;
}