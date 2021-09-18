pub enum Gender {
    Man,
    Woman,
    Other
}

impl Gender {
    pub fn name(&self) -> &str {
        return match self {
            Self::Man => "Man",
            Self::Woman => "Woman",
            Self::Other => "Other",
            _ => { "Unknown" }
        }
    }
}











pub enum OldGender {
    Man {name: &'static str},
    Woman {name: &'static str},
    Other {name: &'static str}
}

impl OldGender {
    pub fn name(&self) -> &str {
        return match self {
            Self::Man { name: "Man" } => "Man",
            Self::Woman { name: "Woman" } => "Woman",
            Self::Other { name: "Other" } => "Other",
            _ => { "Unknown" }
        }
    }
}