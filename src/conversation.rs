use crate::error::ConversationError;
use colored::*;
use rig::completion::Prompt;
use rig::providers::openai::{self, CompletionModel};
use rig::model::Model;
use std::io::{self, Write};

const SYSTEM_INSTRUCTIONS: &str = r#"You are an expert Stylus smart contract analyzer and assistant. Your role is to:
1. Help developers write efficient and secure Stylus smart contracts
2. Provide detailed explanations of gas optimization techniques
3. Identify potential security vulnerabilities
4. Suggest best practices for smart contract development
5. Answer questions about the Stylus ecosystem and its features

When analyzing code:
- Focus on gas efficiency and security best practices
- Provide concrete examples and explanations
- Consider the trade-offs between optimization and readability
- Reference relevant documentation and standards"#;

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
        println!("❓ Answering questions about Stylus development");
        println!("{}", "\nType 'exit' to quit. Type 'help' for more information.".cyan());
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
        println!("\n{}", "=== Available Commands ===".yellow().bold());
        println!("- analyze <file>: Analyze a Stylus smart contract for gas optimizations");
        println!("- security <file>: Check for security vulnerabilities");
        println!("- explain <topic>: Get detailed explanations about Stylus concepts");
        println!("- help: Display this help message");
        println!("- exit: Exit the assistant\n");

        println!("{}", "=== Example Questions ===".yellow().bold());
        println!("- How can I optimize gas usage in my contract?");
        println!("- What are common security vulnerabilities in Stylus contracts?");
        println!("- How do I implement safe transfer patterns?");
        println!("- What's the best way to handle state variables?\n");
    }
}