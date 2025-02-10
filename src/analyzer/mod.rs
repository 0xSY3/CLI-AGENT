use std::path::PathBuf;
use std::error::Error;

pub mod gas;
pub mod size;
pub mod security;
pub mod complexity;
pub mod interactions;
pub mod quality;

use crate::parser::ParsedContract;

#[async_trait::async_trait]
pub trait Analyzer {
    async fn analyze(&self, file: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>>;

    fn get_follow_up_questions(&self, analysis: &str, _parsed: &ParsedContract) -> Vec<String> {
        let mut questions = Vec::new();

        // Common follow-up questions based on analysis content
        if analysis.contains("Critical") || analysis.contains("High") {
            questions.push("Would you like me to explain the high-priority issues in more detail?".to_string());
        }

        if analysis.contains("optimization") {
            questions.push("Should I provide specific code examples for the suggested optimizations?".to_string());
        }

        if analysis.contains("security") {
            questions.push("Would you like a detailed security audit report with mitigation strategies?".to_string());
        }

        if analysis.contains("test") {
            questions.push("Should I generate a test suite template for your contract?".to_string());
        }

        questions
    }

    fn get_suggested_improvements(&self, analysis: &str, _parsed: &ParsedContract) -> Vec<String> {
        let mut improvements = Vec::new();

        // Extract and suggest improvements based on analysis
        if analysis.contains("gas") {
            improvements.push("💡 Consider implementing batch operations for improved gas efficiency".to_string());
        }

        if analysis.contains("storage") {
            improvements.push("💡 Review storage patterns for better L2 optimization".to_string());
        }

        if analysis.contains("test") {
            improvements.push("💡 Add comprehensive test coverage with unit and integration tests".to_string());
        }

        if analysis.contains("event") {
            improvements.push("💡 Optimize event emissions for better gas usage".to_string());
        }

        improvements
    }
}