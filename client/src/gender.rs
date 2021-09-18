pub enum Gender {
    Man {name: &'static str},
    Woman {name: &'static str},
    Other {name: &'static str}
}

impl Gender {
    pub fn name(&self) -> &str {
        return match self {
            Self::Man { name: "Man" } => "Man",
            Self::Woman { name: "Woman" } => "Woman",
            Self::Other { name: "Other" } => "Other",
            _ => { "Unknown" }
        }
    }
}