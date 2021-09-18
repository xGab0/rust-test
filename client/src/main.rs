mod gender;
mod person;

use chrono::{DateTime, Duration, Utc};
use crate::person::Person;
use crate::gender::Gender;

fn main() {
    println!("Starting the client...");

    let person = Person {
        name: "Mike",
        gender: Gender::Woman,
        birthday: Utc::now()
    };

    person.talk();
}
