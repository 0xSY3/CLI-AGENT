use crate::error::ConversationError;
use colored::*;
use rig::completion::Prompt;
use rig::providers::openai::{self, CompletionModel};
use rig::model::Model;
use std::io::{self, Write};

const SYSTEM_INSTRUCTIONS: &str = r#"You are an expert Stylus smart contract analyzer and assistant. Your role is to:
1. Help developers write efficient and secure Stylus smart contracts 
2. Provide detailed explanations of gas and memory optimization techniques specific to Stylus
3. Identify potential security vulnerabilities and provide concrete fixes
4. Suggest best practices for Stylus development
5. Compare Stylus and Solidity patterns for optimal implementation

When analyzing contracts:
- Focus on Stylus-specific optimizations and security best practices
- Analyze memory usage patterns and suggest improvements
- Consider contract size limitations (24kb limit)
- Compare Solidity vs Stylus memory patterns
- Explain the reasoning behind each suggestion
- Consider the trade-offs between optimization and readability
- Reference relevant Stylus/Arbitrum documentation
- Highlight common patterns that may lead to vulnerabilities
- Suggest concrete code examples for fixes
- Consider both gas costs and security implications
- Evaluate state variable access patterns
- Check for proper error handling
- Look for reentrancy risks
- Verify integer overflow protections

Memory-Specific Analysis:
- Identify inefficient memory operations
- Suggest optimal data structures
- Check for unnecessary memory allocations
- Analyze temporary variable usage
- Evaluate stack vs heap usage
- Consider memory layout optimizations
"#;

pub struct Conversation {
    model: Model<CompletionModel>,
}

impl Conversation {
    pub fn new() -> Result<Self, ConversationError> {
        let openai_client = openai::Client::from_env();
        let model = openai_client.model("gpt-3.5-turbo").build();
        Ok(Self { model })
    }

    pub async fn start_interactive(&mut self) -> Result<(), ConversationError> {
        println!("{}", "\n=== Stylus Smart Contract Assistant ===".green().bold());
        println!("{}", "I can help you with:".yellow());
        println!("🔍 Analyzing smart contracts for gas optimization");
        println!("🛡️ Identifying security vulnerabilities");
        println!("💡 Suggesting best practices");
        println!("❓ Answering questions about Stylus development\n");

        println!("{}", "Available Commands:".yellow());
        println!("- analyze <file>: Analyze contract for gas & security issues");
        println!("- help: Show detailed help");
        println!("- exit: Quit the assistant\n");

        println!("{}", "Smart Contract Review Tips:".yellow());
        println!("1. Start with 'analyze' to get an overview");
        println!("2. Review gas optimizations first");
        println!("3. Check security findings carefully");
        println!("4. Ask questions about specific findings\n");

        println!("{}", "\nType 'help' for more information or start with a command.".cyan());
        println!("{}", "===========================================\n".green().bold());

        loop {
            print!("{} ", "You>".blue().bold());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            match input.to_lowercase().as_str() {
                "exit" => {
                    println!("{}", "\nThank you for using the Stylus Smart Contract Assistant!".green());
                    break;
                }
                "help" => {
                    self.display_help();
                    continue;
                }
                "" => continue,
                _ => {
                    match self.query_with_context(input).await {
                        Ok(response) => println!("{} {}", "\nAssistant>".green().bold(), response),
                        Err(e) => eprintln!("{} {}", "\nError:".red().bold(), e),
                    }
                    println!(); // Add a newline for better readability
                }
            }
        }

        Ok(())
    }

    pub async fn single_query(&mut self, prompt: &str) -> Result<String, ConversationError> {
        self.query_with_context(prompt).await
    }

    async fn query_with_context(&mut self, prompt: &str) -> Result<String, ConversationError> {
        let context_prompt = format!("{}\n\nUser question: {}", SYSTEM_INSTRUCTIONS, prompt);
        let response = self
            .model
            .prompt(&context_prompt)
            .await
            .map_err(|e| ConversationError::ApiError(e.to_string()))?;

        Ok(response.to_string())
    }

    fn display_help(&self) {
        println!("\n{}", "=== Detailed Assistant Usage ===".yellow().bold());

        println!("\n{}", "Analysis Commands:".cyan().bold());
        println!("analyze <file> [--type gas|security|all] [--memory-details] [--compare-solidity]");
        println!("  Analyze a contract with options:");
        println!("  - --memory-details: Include detailed memory analysis");
        println!("  - --compare-solidity: Compare with Solidity patterns");
        println!("Examples:");
        println!("  - analyze token.rs --type gas");
        println!("  - analyze token.rs --memory-details");

        println!("\n{}", "Memory Analysis Focus:".cyan().bold());
        println!("- Review memory allocation patterns");
        println!("- Check stack vs heap usage");
        println!("- Analyze storage access patterns");
        println!("- Verify contract size limits");

        println!("\n{}", "Common Questions to Ask:".yellow().bold());
        println!("- How can I optimize this storage pattern?");
        println!("- What are the security implications of this design?");
        println!("- How can I make this function more gas efficient?");
        println!("- How does this compare to Solidity?");

        println!("\n{}", "Analysis Process:".yellow().bold());
        println!("1. Run initial analysis on your contract");
        println!("2. Review memory and gas optimization suggestions");
        println!("3. Check security findings");
        println!("4. Compare with Solidity patterns if needed");
        println!("5. Implement fixes");
        println!("6. Re-run analysis to verify improvements\n");

        println!("{}", "Best Practices:".yellow().bold());
        println!("- Always verify security findings");
        println!("- Consider memory and gas costs");
        println!("- Document security assumptions");
        println!("- Follow Stylus patterns");
        println!("- Keep contracts under 24kb\n");
    }
}