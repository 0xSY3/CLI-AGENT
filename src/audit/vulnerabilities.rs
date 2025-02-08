#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct Vulnerability {
    pub name: String,
    pub severity: Severity,
    pub risk_description: String,
    pub recommendation: String,
}