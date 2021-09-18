use chrono::{DateTime, Duration, Utc};
use crate::gender::Gender;

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