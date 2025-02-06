use super::StylusError;
use quote::quote;
use syn::{parse_str, ItemFn};

pub struct TestGenerator {
    content: String,
}

impl TestGenerator {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn generate_unit_tests(&self) -> Result<String, StylusError> {
        let mut tests = String::new();
        
        // Parse the content to find functions
        let functions: Vec<ItemFn> = self.content.lines()
            .filter(|line| line.contains("pub fn"))
            .filter_map(|line| parse_str(line).ok())
            .collect();

        // Generate test for each function
        for func in functions {
            let func_name = &func.sig.ident;
            let test_name = format!("test_{}", func_name);
            
            let test = quote! {
                #[test]
                fn #test_name() {
                    // Setup test environment
                    let contract = Contract::new();
                    
                    // Call the function
                    let result = contract.#func_name();
                    
                    // Add assertions here
                    assert!(result.is_ok());
                }
            };
            
            tests.push_str(&test.to_string());
            tests.push_str("\n\n");
        }

        Ok(tests)
    }

    pub fn generate_fuzz_tests(&self) -> Result<String, StylusError> {
        let mut tests = String::new();
        
        // Parse the content to find functions
        let functions: Vec<ItemFn> = self.content.lines()
            .filter(|line| line.contains("pub fn"))
            .filter_map(|line| parse_str(line).ok())
            .collect();

        // Generate fuzz test for each function
        for func in functions {
            let func_name = &func.sig.ident;
            let test_name = format!("fuzz_test_{}", func_name);
            
            let test = quote! {
                proptest! {
                    #[test]
                    fn #test_name(
                        input in any::<Vec<u8>>(),
                        value in any::<u64>(),
                    ) {
                        let contract = Contract::new();
                        
                        // Call the function with fuzzed inputs
                        let result = contract.#func_name(input, value);
                        
                        // Property-based assertions
                        prop_assert!(result.is_ok());
                    }
                }
            };
            
            tests.push_str(&test.to_string());
            tests.push_str("\n\n");
        }

        Ok(tests)
    }
}

pub fn generate_tests(content: String, test_type: &str) -> Result<String, StylusError> {
    let generator = TestGenerator::new(content);
    
    match test_type {
        "unit" => generator.generate_unit_tests(),
        "fuzz" => generator.generate_fuzz_tests(),
        "both" => {
            let mut tests = String::new();
            tests.push_str(&generator.generate_unit_tests()?);
            tests.push_str("\n");
            tests.push_str(&generator.generate_fuzz_tests()?);
            Ok(tests)
        }
        _ => Err(StylusError::TestGenError("Invalid test type".into())),
    }
}
