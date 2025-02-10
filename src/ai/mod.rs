use std::error::Error;
use rig::{completion::Prompt, providers::openai};
use colored::*;
use dotenv::dotenv;

#[derive(Debug)]
pub struct AnalysisContext {
    pub contract_type: String,
    pub patterns_found: Vec<String>,
    pub security_concerns: Vec<String>,
    pub optimization_suggestions: Vec<String>,
    pub complexity_metrics: Vec<String>,
    pub ai_insights: Vec<String>,
    pub chat_history: Vec<ChatMessage>,
}

#[derive(Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl AnalysisContext {
    pub fn new() -> Self {
        Self {
            contract_type: String::new(),
            patterns_found: Vec::new(),
            security_concerns: Vec::new(),
            optimization_suggestions: Vec::new(),
            complexity_metrics: Vec::new(),
            ai_insights: Vec::new(),
            chat_history: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns_found.push(pattern);
    }

    pub fn add_security_concern(&mut self, concern: String) {
        self.security_concerns.push(concern);
    }

    pub fn add_optimization(&mut self, optimization: String) {
        self.optimization_suggestions.push(optimization);
    }

    pub fn add_complexity_metric(&mut self, metric: String) {
        self.complexity_metrics.push(metric);
    }

    pub fn add_insight(&mut self, insight: String) {
        self.ai_insights.push(insight);
    }

    pub fn add_chat_message(&mut self, role: &str, content: &str) {
        self.chat_history.push(ChatMessage {
            role: role.to_string(),
            content: content.to_string(),
        });
    }

    pub fn get_chat_context(&self) -> String {
        self.chat_history
            .iter()
            .map(|msg| format!("{}: {}", msg.role, msg.content))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn generate_summary(&self) -> String {
        let mut summary = String::new();

        // Start with AI Assistant introduction
        summary.push_str("\n🤖 AI Assistant Analysis:\n");
        summary.push_str("I've analyzed your smart contract and here are my findings:\n\n");

        if !self.patterns_found.is_empty() {
            summary.push_str("🔍 Key Patterns I've Detected:\n");
            for pattern in &self.patterns_found {
                summary.push_str(&format!("• {}\n", pattern));
            }
        }

        if !self.security_concerns.is_empty() {
            summary.push_str("\n⚠️ Security Concerns I've Identified:\n");
            for concern in &self.security_concerns {
                summary.push_str(&format!("• {}\n", concern.red().bold()));
            }
        }

        if !self.optimization_suggestions.is_empty() {
            summary.push_str("\n💡 Optimization Opportunities I'd Recommend:\n");
            for suggestion in &self.optimization_suggestions {
                summary.push_str(&format!("• {}\n", suggestion.green()));
            }
        }

        if !self.complexity_metrics.is_empty() {
            summary.push_str("\n📊 Complexity Analysis Results:\n");
            for metric in &self.complexity_metrics {
                summary.push_str(&format!("• {}\n", metric.blue()));
            }
        }

        if !self.ai_insights.is_empty() {
            summary.push_str("\n🧠 My Additional Insights:\n");
            for insight in &self.ai_insights {
                summary.push_str(&format!("• {}\n", insight.cyan()));
            }
        }

        // Add suggestions for next steps
        summary.push_str("\n📝 Suggested Next Steps:\n");
        if !self.security_concerns.is_empty() {
            summary.push_str("1. Address the security concerns I've highlighted above\n");
        }
        if !self.optimization_suggestions.is_empty() {
            summary.push_str("2. Implement the suggested optimizations to improve efficiency\n");
        }
        summary.push_str("3. Consider running additional analysis on specific areas of interest\n");

        // Add interactive prompt
        summary.push_str("\n💬 Would you like me to provide more details about any specific finding?\n");

        summary
    }
}

pub async fn analyze_with_context(content: &str, context: &mut AnalysisContext) -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env file");
    let openai_client = openai::Client::new(api_key.as_str());
    let gpt = openai_client.model("gpt-4-turbo-preview").build();

    // Add system message to chat history
    context.add_chat_message(
        "system",
        "You are an expert Arbitrum Stylus smart contract analyzer with deep knowledge of security, optimization, and best practices. \
         Format your responses without markdown syntax (no ###, **, or -). Use plain text with proper spacing and bullet points (•) where needed.",
    );

    // Enhanced prompt with more conversational style and formatting instructions
    let contextual_prompt = format!(
        "As an AI assistant specializing in Arbitrum Stylus smart contract analysis, please analyze this contract with the following focus areas \
         and provide your response in plain text format (no markdown):\n\
         1. Rust/Solidity Patterns & Best Practices\n\
         2. Layer 2 Optimization Techniques\n\
         3. Security Vulnerabilities & Mitigations\n\
         4. Gas Optimization Strategies\n\
         5. Cross-Contract Interaction Patterns\n\
         6. Memory Safety & Resource Management\n\
         7. Error Handling & Recovery Mechanisms\n\
         8. Testing & Verification Approaches\n\n\
         Previous Context:\n{}\n\n\
         Contract Type: {}\n\
         Previous Findings:\n\
         • Patterns: {}\n\
         • Security Issues: {}\n\
         • Optimizations: {}\n\
         • Complexity: {}\n\n\
         Contract to Analyze:\n\n{}\n\n\
         For each issue found, provide:\n\
         1. Severity (Critical/High/Medium/Low)\n\
         2. Impact Description\n\
         3. Specific Code Location\n\
         4. Recommended Fix\n\
         5. Best Practices Reference\n\
         Format the response in plain text with proper spacing and bullet points (•).",
        context.get_chat_context(),
        context.contract_type,
        context.patterns_found.join(", "),
        context.security_concerns.join(", "),
        context.optimization_suggestions.join(", "),
        context.complexity_metrics.join(", "),
        content
    );

    // Add analysis request to chat history
    context.add_chat_message("user", "Please analyze this smart contract.");

    let response = gpt.prompt(&contextual_prompt).await?;

    // Clean up any remaining markdown syntax from the response
    let cleaned_response = response
        .lines()
        .map(|line| {
            line.trim_start_matches("###")
                .trim_start_matches("**")
                .trim_end_matches("**")
                .trim_start_matches("- ")
                .trim()
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Add AI response to chat history
    context.add_chat_message("assistant", &cleaned_response);

    update_context_from_response(&cleaned_response, context);

    // Return the combined analysis
    Ok(format!("{}\n\n{}", cleaned_response, context.generate_summary()))
}

fn update_context_from_response(response: &str, context: &mut AnalysisContext) {
    // Extract patterns
    if let Some(patterns_section) = response.split("Patterns Found:").nth(1) {
        for line in patterns_section.lines().take_while(|l| !l.contains("Security")) {
            if line.starts_with("-") || line.starts_with("•") {
                context.add_pattern(line.trim().to_string());
            }
        }
    }

    // Extract security concerns
    if let Some(security_section) = response.split("Security Analysis:").nth(1) {
        for line in security_section.lines() {
            if line.contains("Critical") || line.contains("High") {
                context.add_security_concern(line.trim().to_string());
            }
        }
    }

    // Extract optimization suggestions
    if let Some(optimization_section) = response.split("Optimization Opportunities:").nth(1) {
        for line in optimization_section.lines().take_while(|l| !l.contains("Complexity")) {
            if line.starts_with("-") || line.starts_with("•") {
                context.add_optimization(line.trim().to_string());
            }
        }
    }

    // Extract complexity metrics
    if let Some(complexity_section) = response.split("Complexity Analysis:").nth(1) {
        for line in complexity_section.lines().take_while(|l| !l.contains("Summary")) {
            if line.contains("score") || line.contains("metric") {
                context.add_complexity_metric(line.trim().to_string());
            }
        }
    }

    // Extract AI insights
    if let Some(insights_section) = response.split("AI Insights:").nth(1) {
        for line in insights_section.lines() {
            if line.starts_with("-") || line.starts_with("•") {
                context.add_insight(line.trim().to_string());
            }
        }
    }
}

pub async fn analyze_gas_usage(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Gas Analysis".to_string();
    analyze_with_context(content, &mut context).await
}

pub async fn analyze_contract_size(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Size Analysis".to_string();
    analyze_with_context(content, &mut context).await
}

pub async fn analyze_security_issues(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Security Analysis".to_string();
    analyze_with_context(content, &mut context).await
}

pub async fn analyze_upgrade_patterns(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Upgrade Pattern Analysis".to_string();
    analyze_with_context(content, &mut context).await
}

pub async fn analyze_function_complexity(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Function Complexity Analysis".to_string();
    analyze_with_context(content, &mut context).await
}

pub async fn analyze_contract_interactions(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Contract Interactions Analysis".to_string();
    analyze_with_context(content, &mut context).await
}

pub async fn analyze_stylus_patterns(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Stylus Pattern Analysis".to_string();
    analyze_with_context(content, &mut context).await
}

pub async fn analyze_error_patterns(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Error Pattern Analysis".to_string();
    analyze_with_context(content, &mut context).await
}

pub async fn analyze_code_quality(content: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut context = AnalysisContext::new();
    context.contract_type = "Code Quality Analysis".to_string();
    analyze_with_context(content, &mut context).await
}