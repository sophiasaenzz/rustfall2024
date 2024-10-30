#[derive(Debug)]
pub struct Student {
    name: String,
    major: String,
}

impl Student {
    pun fn new() -> Self {
        Student {
            name: "Neo".to_string(),
            major: "CS".to_string(),
        }
    }
}