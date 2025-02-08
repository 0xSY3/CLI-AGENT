use std::error::Error;
use rig::{completion::Prompt, providers::openai};
use crate::analyzer::{gas::GasAnalysis, size};

pub async fn analyze_gas_usage(content: &str) -> Result<GasAnalysis, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus smart contract auditor specializing in L2-aware gas optimization. \
         Analyze this contract with deep understanding of Rust memory patterns, Arbitrum's L2 specifics, and Stylus runtime:\n\n{}\n\n\
         Provide a detailed analysis following this structure:\n\
         1. L2-Specific Gas Analysis:\n\
            - Arbitrum Transaction Costs:\n\
              * L2 block space efficiency\n\
              * Cross-chain messaging costs\n\
              * Sequencer fee optimization\n\
              * Challenge period overhead\n\
              * L1 data posting overhead\n\
              * L2-to-L1 message efficiency\n\
              * Batch posting optimization\n\
              * ArbOS resource usage\n\
            - Stylus Runtime Optimization:\n\
              * Memory allocation patterns\n\
              * Type system efficiency\n\
              * ABI encoding optimization\n\
              * Native type utilization\n\
              * State access patterns\n\
              * Zero-copy opportunities\n\
              * Rust runtime overhead\n\
              * Resource cleanup costs\n\
            Rate impact: Critical (>250k gas), High (100-250k), Medium (50-100k), Low (<50k)\n\n\
         2. Memory Management Analysis:\n\
            - Rust Memory Optimization:\n\
              * Stack vs heap allocation\n\
              * Zero-copy implementation\n\
              * Reference borrowing\n\
              * Drop trait efficiency\n\
              * Custom allocator usage\n\
              * Arena allocation patterns\n\
              * Smart pointer overhead\n\
              * Lifetime optimization\n\
            - Contract Memory Layout:\n\
              * Temporary data handling\n\
              * Memory expansion control\n\
              * Buffer management\n\
              * Resource cleanup\n\
              * Memory pooling\n\
              * Page allocation\n\
              * Cache line alignment\n\
              * Memory fragmentation\n\
            For each pattern provide:\n\
            * Current gas cost\n\
            * Optimized cost\n\
            * Implementation complexity\n\
            * Example optimization\n\n\
         3. Storage Optimization:\n\
            - Storage Strategy:\n\
              * Slot packing efficiency\n\
              * Hot/cold data separation\n\
              * Access pattern analysis\n\
              * Collision prevention\n\
              * State trie optimization\n\
              * Merkle proof efficiency\n\
              * State compression\n\
              * Delta encoding\n\
            - Stylus Storage Features:\n\
              * Native type storage\n\
              * Serialization optimization\n\
              * Cache utilization\n\
              * Batch operations\n\
              * State transition costs\n\
              * Rollup-specific patterns\n\
              * State diff optimization\n\
              * Pruning strategies\n\
            Rate optimization impact\n\n\
         4. Function Optimization:\n\
            - Call Pattern Analysis:\n\
              * Internal vs external calls\n\
              * Parameter passing\n\
              * Return value handling\n\
              * Function inlining\n\
              * Cross-contract calls\n\
              * L1/L2 bridge calls\n\
              * Callback optimization\n\
              * Error handling costs\n\
            - Batch Operations:\n\
              * Multi-call aggregation\n\
              * State update batching\n\
              * Event emission strategy\n\
              * Cross-contract batching\n\
              * Sequencer optimization\n\
              * Challenge handling\n\
              * Proof generation\n\
              * Data availability\n\
            Provide gas metrics\n\n\
         5. Stylus Optimizations:\n\
            - Rust Implementation:\n\
              * Generic vs concrete types\n\
              * Trait implementation cost\n\
              * Iterator optimization\n\
              * Smart pointer usage\n\
              * Custom allocator patterns\n\
              * Native type utilization\n\
              * Compiler optimization\n\
              * Code generation\n\
            - L2 Architecture:\n\
              * Data availability cost\n\
              * State commitment\n\
              * Proof generation\n\
              * Challenge readiness\n\
              * Sequencer interaction\n\
              * Cross-chain bridges\n\
              * Rollup compression\n\
              * ArbOS integration\n\
            For each optimization provide:\n\
            * Current implementation\n\
            * Optimized approach\n\
            * Gas savings estimate\n\
            * Implementation guide\n\n\
         6. Benchmarking Strategy:\n\
            - Test Coverage:\n\
              * Common operations\n\
              * Worst-case scenarios\n\
              * Batch operations\n\
              * Cross-chain calls\n\
              * L2 operations\n\
              * Bridge interactions\n\
              * State transitions\n\
              * Challenge scenarios\n\
            - Monitoring Plan:\n\
              * Gas usage tracking\n\
              * Threshold alerts\n\
              * Performance metrics\n\
              * L2 cost analysis\n\
              * Cross-chain costs\n\
              * Challenge impact\n\
              * Sequencer fees\n\
              * Resource usage\n\
            Provide specific thresholds and metrics",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    let sections: Vec<&str> = response.split("\n\n").collect();

    Ok(GasAnalysis {
        operations: sections.get(0..2)
            .map(|s| s.join("\n\n"))
            .unwrap_or_else(|| "No memory operations analysis available".to_string()),
        recommendations: sections.get(2..)
            .map(|s| s.join("\n\n"))
            .unwrap_or_else(|| "No recommendations available".to_string()),
    })
}

pub async fn analyze_contract_size(content: &str) -> Result<size::SizeAnalysis, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus smart contract auditor focusing on contract size optimization. \
         Analyze this contract with deep understanding of Rust compilation and Arbitrum deployment constraints:\n\n{}\n\n\
         Provide a detailed analysis following this structure:\n\
         1. Contract Size Analysis:\n\
            - Bytecode Size Assessment:\n\
              * Current compiled size estimation\n\
              * Size category impact on deployment\n\
              * Contract splitting necessity\n\
            Rate impact: Critical (>64KB), Major (32-64KB), Medium (16-32KB), Minor (<16KB)\n\
            - Code Component Analysis:\n\
              * Function contribution breakdown\n\
              * Library code impact\n\
              * Constant and static data size\n\
              * Generated code overhead\n\
            Provide size contribution percentage for each component\n\n\
         2. Rust-Specific Size Impact:\n\
            - Generic Code Analysis:\n\
              * Monomorphization impact\n\
              * Trait implementation size\n\
              * Type-level optimizations\n\
            - Standard Library Usage:\n\
              * Required vs optional features\n\
              * Alternative implementations\n\
              * Custom type opportunities\n\
            Rate each aspect's size impact\n\n\
         3. Arbitrum-Specific Considerations:\n\
            - Deployment Constraints:\n\
              * Nitro execution environment\n\
              * Cross-chain interaction code\n\
              * ABI encoding overhead\n\
            - Optimization Strategies:\n\
              * Code modularization\n\
              * Proxy pattern benefits\n\
              * Library externalization\n\
            Provide specific size reduction strategies\n\n\
         4. Implementation Recommendations:\n\
            - Code Organization:\n\
              * Function consolidation\n\
              * Type simplification\n\
              * Dead code elimination\n\
            - Storage Optimization:\n\
              * Struct layout efficiency\n\
              * Enum optimization\n\
              * Bit packing strategies\n\
            - Dependency Management:\n\
              * Required vs optional features\n\
              * Alternative implementations\n\
              * Custom solutions\n\
            For each recommendation provide:\n\
            * Current size impact\n\
            * Expected size reduction\n\
            * Implementation complexity\n\
            * Migration risk assessment\n\
            * Code example",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    let sections: Vec<&str> = response.split("\n\n").collect();

    Ok(size::SizeAnalysis {
        issues: sections.get(0..2)
            .map(|s| s.join("\n\n"))
            .unwrap_or_else(|| "No size analysis available".to_string()),
        suggestions: sections.get(2..)
            .map(|s| s.join("\n\n"))
            .unwrap_or_else(|| "No suggestions available".to_string()),
    })
}

pub async fn analyze_security_issues(content: &str) -> Result<crate::analyzer::security::SecurityAnalysis, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus smart contract security auditor with deep knowledge of Rust safety, L2 security, and Stylus-specific vulnerabilities. \
         Analyze this contract with emphasis on L2-specific security concerns:\n\n{}\n\n\
         Provide a comprehensive security audit following this structure:\n\
         1. L2-Specific Security Analysis:\n\
            - Cross-Chain Vulnerabilities:\n\
              * Message passing validation\n\
              * State replication attacks\n\
              * Bridge manipulation vectors\n\
              * Replay protection mechanisms\n\
              * Challenge period manipulation\n\
              * L1/L2 state synchronization\n\
            - Sequencer Security:\n\
              * Transaction ordering attacks\n\
              * MEV protection measures\n\
              * Sequencer failure handling\n\
              * Censorship resistance\n\
              * Backrunning protection\n\
              * Priority fee manipulation\n\
            - State Management:\n\
              * State commitment integrity\n\
              * Fraud proof readiness\n\
              * State transition validation\n\
              * Data availability proofs\n\
              * State root verification\n\
              * Merkle proof validation\n\
            Rate severity: Critical/High/Medium/Low\n\n\
         2. Rust Safety Analysis:\n\
            - Memory Safety:\n\
              * Unsafe block analysis\n\
              * Lifetime validation\n\
              * Reference safety\n\
              * Integer overflow protection\n\
              * Buffer overflow prevention\n\
              * Resource cleanup validation\n\
            - Type System Security:\n\
              * Generic type safety\n\
              * Trait implementation safety\n\
              * Custom type validation\n\
              * Type state programming\n\
              * Zero-sized type usage\n\
              * Drop trait safety\n\
            - Error Handling:\n\
              * Result type usage\n\
              * Error propagation patterns\n\
              * Panic prevention\n\
              * Recovery mechanisms\n\
              * Error type design\n\
              * Error conversion safety\n\
            Provide detailed vulnerability analysis\n\n\
         3. Smart Contract Security:\n\
            - Access Control:\n\
              * Permission validation\n\
              * Role management\n\
              * Function visibility\n\
              * Upgrade controls\n\
              * Emergency procedures\n\
              * Time-lock mechanisms\n\
            - State Protection:\n\
              * Reentrancy guards\n\
              * Check-Effect-Interaction\n\
              * Storage layout safety\n\
              * State consistency checks\n\
              * Race condition prevention\n\
              * Front-running protection\n\
            - Value Handling:\n\
              * Asset transfer safety\n\
              * Balance management\n\
              * Integer arithmetic safety\n\
              * Fee calculation safety\n\
              * Value locking patterns\n\
              * Withdrawal patterns\n\
            For each finding provide:\n\
            * Vulnerability description\n\
            * Attack scenarios\n\
            * Severity assessment\n\
            * Mitigation strategy\n\
            * Testing approach\n\n\
         4. Stylus Runtime Security:\n\
            - ABI Safety:\n\
              * Parameter validation\n\
              * Return value safety\n\
              * Encoding/decoding security\n\
              * Cross-contract call safety\n\
              * Type conversion safety\n\
              * Memory safety patterns\n\
            - Resource Management:\n\
              * Gas exhaustion prevention\n\
              * Memory limit handling\n\
              * Stack depth protection\n\
              * Storage access safety\n\
              * Computation bounds\n\
              * Resource cleanup\n\
            - Runtime Constraints:\n\
              * Execution time limits\n\
              * Memory allocation limits\n\
              * Call depth restrictions\n\
              * Event emission safety\n\
              * State size limits\n\
              * Challenge window handling\n\
            Rate each finding's severity\n\n\
         5. Testing Coverage:\n\
            - Test Scenarios:\n\
              * L2-specific test cases\n\
              * Cross-chain scenarios\n\
              * Fuzzing coverage\n\
              * Property-based tests\n\
              * Integration tests\n\
              * Security-focused tests\n\
            - Monitoring:\n\
              * Security event logging\n\
              * Anomaly detection\n\
              * Alert mechanisms\n\
              * Audit logging\n\
              * Performance tracking\n\
              * Error monitoring\n\
            Provide coverage metrics\n\n\
         6. Security Recommendations:\n\
            For each vulnerability:\n\
            * Current implementation\n\
            * Security impact\n\
            * Attack vectors\n\
            * Mitigation steps\n\
            * Implementation guide\n\
            * Verification process",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    let sections: Vec<&str> = response.split("\n\n").collect();

    Ok(crate::analyzer::security::SecurityAnalysis {
        vulnerabilities: sections.get(0)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "No vulnerabilities found".to_string()),
        recommendations: sections.get(1..)
            .map(|s| s.join("\n\n"))
            .unwrap_or_else(|| "No recommendations available".to_string()),
    })
}

pub async fn analyze_upgrade_patterns(content: &str) -> Result<String, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus smart contract auditor specializing in upgrade patterns. \
         Analyze this contract's upgrade capabilities with deep understanding of Rust and proxy patterns:\n\n{}\n\n\
         Provide a detailed upgrade analysis following this structure:\n\
         1. Upgrade Pattern Implementation:\n\
            - Current Architecture:\n\
              * Proxy pattern usage\n\
              * Storage layout strategy\n\
              * Function delegation\n\
              * State management\n\
            - Upgrade Mechanism:\n\
              * Upgrade process flow\n\
              * Access control system\n\
              * State migration approach\n\
              * Validation checks\n\
            Rate implementation completeness\n\n\
         2. Safety Analysis:\n\
            - Storage Safety:\n\
              * Slot collision prevention\n\
              * Layout compatibility\n\
              * Type safety measures\n\
              * Initialization patterns\n\
            - Runtime Safety:\n\
              * Delegate call security\n\
              * Function selector handling\n\
              * Return data processing\n\
              * Error propagation\n\
            Identify potential vulnerabilities\n\n\
         3. Rust Implementation Details:\n\
            - Type System Usage:\n\
              * Trait implementations\n\
              * Generic parameters\n\
              * Storage abstractions\n\
              * Error handling\n\
            - Memory Safety:\n\
              * Ownership patterns\n\
              * Reference safety\n\
              * Drop behavior\n\
              * Zero-copy upgrades\n\
            Rate implementation complexity\n\n\
         4. Arbitrum Considerations:\n\
            - Layer 2 Specifics:\n\
              * State transition handling\n\
              * Cross-chain upgrade coordination\n\
              * Challenge period impact\n\
              * Gas optimization\n\
            - Runtime Constraints:\n\
              * Code size limits\n\
              * Storage access patterns\n\
              * Execution costs\n\
              * Resource usage\n\
            Provide L2-specific recommendations\n\n\
         5. Best Practices Review:\n\
            For each component:\n\
            * Implementation status\n\
            * Compliance level\n\
            * Improvement suggestions\n\
            * Code examples\n\
            * Testing strategies",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    Ok(response)
}

pub async fn analyze_function_complexity(content: &str) -> Result<String, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus smart contract auditor specializing in code complexity analysis. \
         Analyze this contract's function complexity with deep understanding of Rust patterns:\n\n{}\n\n\
         Provide a detailed complexity analysis following this structure:\n\
         1. Function Overview:\n\
            - Function Purpose Analysis:\n\
              * Business logic role\n\
              * State modification patterns\n\
              * Cross-contract interactions\n\
              * Event emission patterns\n\
            - Implementation Strategy:\n\
              * Algorithm choice\n\
              * Data structure usage\n\
              * Memory management approach\n\
              * Error handling strategy\n\
            Rate each function's conceptual complexity\n\n\
         2. Cyclomatic Complexity Analysis:\n\
            - Control Flow Analysis:\n\
              * Decision points (if/else, match)\n\
              * Loop constructs\n\
              * Early returns\n\
              * Error paths\n\
            - Branch Analysis:\n\
              * Execution path count\n\
              * Nesting depth\n\
              * Branch probability\n\
              * Path interdependence\n\
            Rate complexity: High (>15), Medium (8-15), Low (<8)\n\n\
         3. Cognitive Complexity Analysis:\n\
            - Code Structure:\n\
              * Function length\n\
              * Expression complexity\n\
              * Nested closures\n\
              * Generic type usage\n\
            - Code Organization:\n\
              * Single responsibility principle\n\
              * Function cohesion\n\
              * Code duplication\n\
              * Component coupling\n\
            Rate complexity: High (>20), Medium (10-20), Low (<10)\n\n\
         4. Rust-Specific Complexity:\n\
            - Type System Usage:\n\
              * Generic constraints\n\
              * Trait bounds\n\
              * Lifetime parameters\n\
              * Type state complexity\n\
            - Memory Management:\n\
              * Ownership patterns\n\
              * Borrowing complexity\n\
              * Drop behavior\n\
              * Smart pointer usage\n\
            Rate complexity impact on maintenance\n\n\
         5. Smart Contract Considerations:\n\
            - State Management:\n\
              * Storage access patterns\n\
              * State transition complexity\n\
              * Concurrency considerations\n\
            - External Interactions:\n\
              * Cross-contract calls\n\
              * Input validation\n\
              * Error handling\n\
            Provide specific complexity metrics\n\n\
         6. Optimization Recommendations:\n\
            For each high/medium complexity item:\n\
            * Refactoring suggestions\n\
            * Code organization improvements\n\
            * Complexity reduction strategies\n\
            * Implementation examples\n\
            * Risk assessment",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    Ok(response)
}

pub async fn analyze_contract_interactions(content: &str) -> Result<String, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus smart contract auditor specializing in cross-contract interactions. \
         Analyze this contract's interaction patterns with deep understanding of Layer 2 specifics:\n\n{}\n\n\
         Provide a detailed interaction analysis following this structure:\n\
         1. Interaction Pattern Analysis:\n\
            - Direct Contract Calls:\n\
              * Call patterns and targets\n\
              * Value transfer safety\n\
              * Reentry protection\n\
              * Failure handling\n\
            - Interface Integration:\n\
              * Contract dependencies\n\
              * ABI compatibility\n\
              * Version management\n\
              * Upgrade impact\n\
            Rate each pattern's risk level\n\n\
         2. Layer 2 Specific Concerns:\n\
            - Cross-Chain Communication:\n\
              * Message passing patterns\n\
              * State synchronization\n\
              * Sequencer interaction\n\
              * Challenge period handling\n\
            - Arbitrum Runtime:\n\
              * ABI encoding safety\n\
              * Gas estimation accuracy\n\
              * Resource limitations\n\
              * Execution guarantees\n\
            Analyze potential failure modes\n\n\
         3. Trust and Security Analysis:\n\
            - Trust Assumptions:\n\
              * External contract trust level\n\
              * Privilege escalation risks\n\
              * State manipulation vectors\n\
              * Economic attack surfaces\n\
            - Security Measures:\n\
              * Access control implementation\n\
              * Value locking patterns\n\
              * State validation\n\
              * Emergency controls\n\
            Rate security impact: Critical/High/Medium/Low\n\n\
         4. Optimization Opportunities:\n\
            - Call Pattern Optimization:\n\
              * Batching opportunities\n\
              * State aggregation\n\
              * Callback reduction\n\
              * Gas optimization\n\
            - Data Management:\n\
              * Caching strategies\n\
              * State compression\n\
              * Event optimization\n\
              * Storage efficiency\n\
            Provide specific recommendations\n\n\
         5. Implementation Guidelines:\n\
            For each interaction pattern:\n\
            * Safety checklist\n\
            * Best practices\n\
            * Common pitfalls\n\
            * Example implementations\n\
            * Testing strategies",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    Ok(response)
}

pub async fn analyze_stylus_patterns(content: &str) -> Result<String, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus smart contract auditor with deep understanding of Rust-Stylus patterns. \
         Analyze this contract with focus on Stylus-specific optimizations and Arbitrum L2 characteristics:\n\n{}\n\n\
         Provide detailed analysis following this structure:\n\
         1. Stylus Runtime Integration:\n\
            - Rust Type System Usage:\n\
              * Generic vs concrete type choices\n\
              * Custom type implementation efficiency\n\
              * Zero-copy optimization opportunities\n\
              * Memory layout considerations\n\
            - ABI Integration:\n\
              * Parameter encoding efficiency\n\
              * Return value optimization\n\
              * Cross-contract data handling\n\
              * Event emission patterns\n\
            Rate implementation quality: Optimal/Good/Needs Improvement\n\n\
         2. L2-Aware Design Patterns:\n\
            - Data Availability:\n\
              * State compression techniques\n\
              * Batch operation patterns\n\
              * Calldata optimization\n\
              * Storage layout efficiency\n\
            - Cross-Chain Communication:\n\
              * Message passing patterns\n\
              * State synchronization\n\
              * Challenge period handling\n\
              * Sequencer interaction\n\
            Provide specific optimization metrics\n\n\
         3. Performance Optimization:\n\
            - Computation:\n\
              * Loop optimization\n\
              * Memory reuse patterns\n\
              * Stack vs heap usage\n\
              * Function inlining\n\
            - Storage Access:\n\
              * Caching strategies\n\
              * Batch updates\n\
              * Hot/cold separation\n\
              * Slot packing\n\
            Rate each optimization's impact\n\n\
         4. Security Integration:\n\
            - Access Control:\n\
              * Permission management\n\
              * Role validation\n\
              * Upgrade controls\n\
              * Emergency procedures\n\
            - State Protection:\n\
              * Reentrancy guards\n\
              * State consistency\n\
              * Race condition prevention\n\
              * Overflow protection\n\
            Identify security implications\n\n\
         5. Testing & Monitoring:\n\
            - Test Coverage:\n\
              * Unit test patterns\n\
              * Integration testing\n\
              * Property-based tests\n\
              * Fuzz testing\n\
            - Runtime Monitoring:\n\
              * Gas usage tracking\n\
              * Error detection\n\
              * Performance metrics\n\
              * State validation\n\
            Provide specific testing strategies\n\n\
         6. Maintenance & Upgrades:\n\
            - Code Organization:\n\
              * Module structure\n\
              * Dependency management\n\
              * Documentation quality\n\
              * Version control\n\
            - Upgrade Strategy:\n\
              * State migration\n\
              * Backward compatibility\n\
              * Testing approach\n\
              * Rollback procedures\n\
            Rate maintainability and provide recommendations",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    Ok(response)
}

pub async fn analyze_error_patterns(content: &str) -> Result<String, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus smart contract auditor specializing in error handling and recovery patterns. \
         Analyze this contract's error handling mechanisms:\n\n{}\n\n\
         Provide detailed analysis following this structure:\n\
         1. Error Classification:\n\
            - Runtime Errors:\n\
              * Panic conditions\n\
              * Memory safety violations\n\
              * Integer overflows\n\
              * Resource exhaustion\n\
            - Business Logic Errors:\n\
              * State inconsistencies\n\
              * Invalid operations\n\
              * Access control violations\n\
              * Input validation failures\n\
            Rate severity and impact\n\n\
         2. Error Handling Mechanisms:\n\
            - Implementation Patterns:\n\
              * Result type usage\n\
              * Custom error types\n\
              * Error propagation\n\
              * Recovery strategies\n\
            - Contract State:\n\
              * State consistency\n\
              * Rollback mechanisms\n\
              * Cleanup procedures\n\
              * Event emission\n\
            Evaluate effectiveness\n\n\
         3. Recovery Strategies:\n\
            - Immediate Recovery:\n\
              * Error correction\n\
              * State restoration\n\
              * User notification\n\
              * Logging mechanisms\n\
            - Long-term Mitigation:\n\
              * Pattern improvements\n\
              * Monitoring solutions\n\
              * Testing strategies\n\
              * Documentation needs\n\
            Rate recovery capabilities\n\n\
         4. Documentation Analysis:\n\
            - Error Documentation:\n\
              * Error descriptions\n\
              * Recovery procedures\n\
              * User guidance\n\
              * Maintenance notes\n\
            - Integration Guidelines:\n\
              * Error handling best practices\n\
              * Testing requirements\n\
              * Upgrade considerations\n\
              * Security implications\n\
            Assess documentation quality\n\n\
         5. Improvement Recommendations:\n\
            For each error pattern:\n\
            * Current implementation\n\
            * Proposed improvements\n\
            * Implementation priority\n\
            * Testing requirements\n\
            * Example solutions",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    Ok(response)
}

pub async fn analyze_code_quality(content: &str) -> Result<String, Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.model("gpt-3.5-turbo").build();

    let prompt = format!(
        "You are an expert Arbitrum Stylus code quality analyst specializing in Rust and smart contract best practices. \
         Analyze this contract's code quality metrics with deep understanding of L2-specific patterns:\n\n{}\n\n\
         Provide detailed analysis following this structure:\n\
         1. Code Organization:\n\
            - Module Structure:\n\
              * Component separation\n\
              * Dependency graph\n\
              * Interface design\n\
              * Coupling metrics\n\
            - Type System Usage:\n\
              * Generic abstraction quality\n\
              * Trait implementation patterns\n\
              * Error type design\n\
              * Memory safety patterns\n\
            Rate organization quality: Excellent/Good/Fair/Poor\n\n\
         2. Code Style & Documentation:\n\
            - Readability:\n\
              * Naming conventions\n\
              * Function length\n\
              * Code formatting\n\
              * Logic complexity\n\
            - Documentation:\n\
              * API documentation\n\
              * Implementation notes\n\
              * Example usage\n\
              * Security considerations\n\
            - Comments:\n\
              * Clarity and relevance\n\
              * Technical accuracy\n\
              * Maintenance status\n\
              * TODO handling\n\
            Rate documentation completeness\n\n\
         3. Testing Quality:\n\
            - Test Coverage:\n\
              * Unit test patterns\n\
              * Integration tests\n\
              * Property testing\n\
              * Fuzzing coverage\n\
            - Test Organization:\n\
              * Test structure\n\
              * Fixture usage\n\
              * Mock patterns\n\
              * Helper functions\n\
            - Edge Cases:\n\
              * Error conditions\n\
              * Boundary values\n\
              * Race conditions\n\
              * Resource limits\n\
            Rate testing quality\n\n\
         4. Error Handling:\n\
            - Error Types:\n\
              * Custom error definitions\n\
              * Error conversion\n\
              * Recovery strategies\n\
              * Panic prevention\n\
            - Validation:\n\
              * Input validation\n\
              * State validation\n\
              * Output validation\n\
              * Cross-contract validation\n\
            Rate error handling robustness\n\n\
         5. Performance Patterns:\n\
            - Resource Usage:\n\
              * Memory efficiency\n\
              * Storage optimization\n\
              * Computation costs\n\
              * Network usage\n\
            - Optimization:\n\
              * Algorithm choice\n\
              * Data structure usage\n\
              * Caching strategy\n\
              * Batch operations\n\
            Rate performance optimization level\n\n\
         6. Improvement Recommendations:\n\
            For each category provide:\n\
            * Current status\n\
            * Target improvements\n\
            * Implementation steps\n\
            * Priority level\n\
            * Example solutions",
        content
    );

    let response = gpt.prompt(&prompt).await?;
    Ok(response)
}