use solang_parser::pt::{SourceUnit, FunctionTy};
use solang_parser::parse;
use syn::{File as RustFile, Item};
use quote::ToTokens;
use std::error::Error;

#[derive(Debug)]
pub enum ContractType {
    Solidity,
    Stylus,
}

/// Represents a function in a smart contract
#[derive(Debug)]
#[allow(dead_code)]  // Fields are used in analysis
pub struct Function {
    pub name: String,
    pub visibility: String,
    pub params: Vec<String>,
    pub return_type: Option<String>,
    pub body: String,
}

/// Represents a structure in a smart contract
#[derive(Debug)]
#[allow(dead_code)]  // Fields are used in analysis
pub struct Structure {
    pub name: String,
    pub fields: Vec<(String, String)>, // (field_name, field_type)
}

/// Represents a parsed smart contract with its components
#[derive(Debug)]
#[allow(dead_code)]  // Fields are used in analysis
pub struct ParsedContract {
    pub contract_type: ContractType,
    pub functions: Vec<Function>,
    pub structs: Vec<Structure>,
    pub source: String,
}

impl ParsedContract {
    pub fn new(content: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Try parsing as Solidity first
        if let Ok((source_unit, _)) = parse(&content, 0) {
            return Ok(Self::from_solidity(source_unit, content));
        }

        // If not Solidity, try parsing as Rust
        if let Ok(rust_file) = syn::parse_file(&content) {
            return Ok(Self::from_rust(rust_file, content));
        }

        Err("Failed to parse contract as either Solidity or Rust".into())
    }

    fn get_visibility_string(_: &FunctionTy) -> String {
        // Simplified approach: return default visibility
        "internal".to_string()
    }

    fn from_solidity(source_unit: SourceUnit, content: String) -> Self {
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        for part in source_unit.0 {
            if let solang_parser::pt::SourceUnitPart::ContractDefinition(contract) = part {
                for part in contract.parts {
                    match part {
                        solang_parser::pt::ContractPart::FunctionDefinition(func) => {
                            if let Some(name) = func.name {
                                let mut params = Vec::new();
                                for (_, param_opt) in func.params {
                                    if let Some(param) = param_opt {
                                        if let Some(param_name) = param.name {
                                            params.push(format!("{}: {:?}", param_name.name, param.ty));
                                        }
                                    }
                                }

                                let return_type = if !func.returns.is_empty() {
                                    Some(format!("{:?}", func.returns))
                                } else {
                                    None
                                };

                                let body = match func.body {
                                    Some(body) => format!("{:?}", body),
                                    None => String::new(),
                                };

                                functions.push(Function {
                                    name: name.name,
                                    visibility: Self::get_visibility_string(&func.ty),
                                    params,
                                    return_type,
                                    body,
                                });
                            }
                        }
                        solang_parser::pt::ContractPart::StructDefinition(struct_def) => {
                            if let Some(name) = struct_def.name {
                                let mut fields = Vec::new();
                                for field in struct_def.fields {
                                    if let Some(field_name) = field.name {
                                        fields.push((field_name.name, format!("{:?}", field.ty)));
                                    }
                                }

                                structs.push(Structure {
                                    name: name.name,
                                    fields,
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Self {
            contract_type: ContractType::Solidity,
            functions,
            structs,
            source: content,
        }
    }

    fn from_rust(file: RustFile, content: String) -> Self {
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        for item in file.items {
            match item {
                Item::Fn(func) => {
                    let visibility = if matches!(func.vis, syn::Visibility::Public(_)) {
                        "public"
                    } else {
                        "private"
                    }.to_string();

                    functions.push(Function {
                        name: func.sig.ident.to_string(),
                        visibility,
                        params: func.sig.inputs.iter()
                            .map(|arg| arg.to_token_stream().to_string())
                            .collect(),
                        return_type: Some(func.sig.output.to_token_stream().to_string()),
                        body: func.block.to_token_stream().to_string(),
                    });
                }
                Item::Struct(struct_item) => {
                    let fields = struct_item.fields.iter()
                        .filter_map(|field| {
                            field.ident.as_ref().map(|ident| {
                                (ident.to_string(), field.ty.to_token_stream().to_string())
                            })
                        })
                        .collect();

                    structs.push(Structure {
                        name: struct_item.ident.to_string(),
                        fields,
                    });
                }
                _ => {}
            }
        }

        Self {
            contract_type: ContractType::Stylus,
            functions,
            structs,
            source: content,
        }
    }

    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    pub fn struct_count(&self) -> usize {
        self.structs.len()
    }

    pub fn analyze_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();

        // Analyze contract patterns based on contract type
        match self.contract_type {
            ContractType::Solidity => {
                // Analyze function patterns
                for function in &self.functions {
                    // Check visibility
                    if function.visibility == "public" || function.visibility == "external" {
                        patterns.push(format!("Public function '{}' - ensure proper access control", function.name));
                    }

                    // Check state modifications
                    if function.body.contains("storage") {
                        patterns.push(format!("Storage operation in function '{}' - consider optimization", function.name));
                    }

                    // Check loops
                    if function.body.contains("for") || function.body.contains("while") {
                        patterns.push(format!("Loop in function '{}' may have high gas cost", function.name));
                    }

                    // Check parameter count
                    if function.params.len() > 4 {
                        patterns.push(format!("Function '{}' has many parameters ({}) - consider grouping them",
                            function.name, function.params.len()));
                    }
                }

                // Analyze struct patterns
                for structure in &self.structs {
                    if structure.fields.len() > 5 {
                        patterns.push(format!("Struct '{}' has many fields ({}) - consider splitting",
                            structure.name, structure.fields.len()));
                    }
                }
            }
            ContractType::Stylus => {
                // Analyze function patterns
                for function in &self.functions {
                    // Check memory usage
                    if function.body.contains("Vec") || function.body.contains("HashMap") {
                        patterns.push(format!("Dynamic allocation in function '{}' - consider fixed size", function.name));
                    }

                    // Check cloning
                    if function.body.contains("clone") || function.body.contains("to_owned") {
                        patterns.push(format!("Memory clone in function '{}' - consider reference", function.name));
                    }

                    // Check error handling
                    if !function.body.contains("Result") && !function.body.contains("Option") {
                        patterns.push(format!("Function '{}' might need explicit error handling", function.name));
                    }
                }

                // Analyze struct patterns
                for structure in &self.structs {
                    // Check for serialization attributes
                    let has_serde = self.source.contains("#[derive(Serialize");
                    if !has_serde {
                        patterns.push(format!("Struct '{}' might need serialization attributes", structure.name));
                    }
                }
            }
        }

        patterns
    }

    pub fn analyze_gas_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();

        // Analyze gas usage patterns based on contract type
        match self.contract_type {
            ContractType::Solidity => {
                for function in &self.functions {
                    // Check storage operations
                    if function.body.contains("storage") {
                        patterns.push(format!("Function '{}' uses storage - optimize access patterns", function.name));
                    }

                    // Check loops and array operations
                    if function.body.contains("for") || function.body.contains("while") {
                        patterns.push(format!("Loop in function '{}' - consider gas limits", function.name));
                    }

                    // Check event emissions
                    if function.body.contains("emit") {
                        patterns.push(format!("Event emission in '{}' - consider log size", function.name));
                    }
                }
            }
            ContractType::Stylus => {
                for function in &self.functions {
                    // Check heap allocations
                    if function.body.contains("Vec") || function.body.contains("String") {
                        patterns.push(format!("Heap allocation in '{}' - use fixed size when possible", function.name));
                    }

                    // Check serialization
                    if function.body.contains("serialize") || function.body.contains("deserialize") {
                        patterns.push(format!("Serialization in '{}' - optimize encoding", function.name));
                    }
                }
            }
        }

        patterns
    }

    pub fn get_function_size(&self) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let mut size = 0;
        for function in &self.functions {
            size += function.name.len();
            size += function.body.len();
            for param in &function.params {
                size += param.len();
            }
            if let Some(ret) = &function.return_type {
                size += ret.len();
            }
        }
        Ok(size)
    }

    pub fn get_storage_size(&self) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let mut size = 0;
        for structure in &self.structs {
            size += structure.name.len();
            for (field_name, field_type) in &structure.fields {
                size += field_name.len() + field_type.len();
            }
        }
        Ok(size)
    }

    pub fn get_event_size(&self) -> Result<usize, Box<dyn Error + Send + Sync>> {
        // Calculate event size from source code
        let mut size = 0;
        let event_lines: Vec<&str> = self.source.lines()
            .filter(|line| line.contains("event") || line.contains("#[event]"))
            .collect();

        for line in event_lines {
            size += line.len();
        }
        Ok(size)
    }
}