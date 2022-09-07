use chrono::{Utc, DateTime};
use uuid::Uuid;

pub mod registrations;

pub struct Registration {
    id: String,
    name: String,
    created_on: DateTime<Utc>,
}
impl Registration {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(), 
            name: String::from(name),
            created_on: Utc::now(),
        }
    }
}

pub trait Registrations {
    fn all_registrations(&self) -> &Vec<Registration>;
    fn add(&mut self, registration: Registration);    
}

#[cfg(test)]
mod registration_tests {
    use super::*;

    #[test]
    fn registration_should_have_name_and_creation_date() {
        let name = "Fulano de Tal";

        let registration = Registration::new(name);
        
        assert_eq!(name, &registration.name);
        assert!(registration.created_on <= Utc::now());        
    }
}