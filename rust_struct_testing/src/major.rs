#[derive(Debug, PartialEq)]
pub enum Major {
    CS,
    EE,
    Undefined,
}

impl Major {
    pub fn classify(major: &str) -> Major {
        match major {
            "CS" => Major::CS,
            "EE" => Major::EE,
            _ => Major::Undefined,
        }
    }
}