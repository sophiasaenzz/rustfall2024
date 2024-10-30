use crate::major::Major;

#[derive(Debug)]
#[warn(dead_code)]
pub struct Student {
    name: String,
    major: Major,
}

impl Student {
    pub fn new(name:&str, major:&str) -> Student {
        Student {
            name: name.to_string(),
            major: Major::classify(major),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_undefined_creation() {
        let s = Student::new("AAA", "Chemistry");
        assert_eq!(s.major, Major::Undefined);
    }
}