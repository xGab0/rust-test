use chrono::{DateTime, Duration, Utc};

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

pub struct Person {
    pub name: &'static str,
    pub gender: Gender,
    pub birthday: DateTime<Utc>
}

impl Person {
    pub fn talk(&self) {
        println!("Hi, I'm {}", self.name);
        println!("My gender is {}.", self.gender.name());
        println!("My birthday is {}.", self.birthday.to_string());
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}