use chrono::{Utc, DateTime};
use uuid::Uuid;

mod registrations;
pub mod config;

pub struct Registration {
    id: String,
    name: String,
    phone: String,
    created_on: DateTime<Utc>,
}
impl Registration {
    pub fn new(name: &str, phone: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(), 
            name: String::from(name),
            phone: String::from(phone),
            created_on: Utc::now(),
        }
    }
}

#[derive(Debug)]
pub enum RegistrationManagerError {
    DuplicatedRegistration,    
}

pub trait RegistrationManager {
    fn add(&mut self, name: &str, phone: &str) -> Result<(), RegistrationManagerError>;
    fn get_all_registrations(&self) -> &Vec<Registration>;
}

struct RegistrationManagerImpl {
    registrations: Box<dyn Registrations>,
}
impl RegistrationManagerImpl {
    fn new<R: Registrations + 'static>(registrations: R) -> Self {
        Self {
            registrations: Box::from(registrations),
        }
    }
}
impl RegistrationManager for RegistrationManagerImpl {
    fn add(&mut self, name: &str, phone: &str) -> Result<(), RegistrationManagerError> {
        let registration = Registration::new(name, phone);        
        self.registrations.add(registration);
        Ok(())
    }

    fn get_all_registrations(&self) -> &Vec<Registration> {
        self.registrations.all_registrations()
    }
}

trait Registrations {
    fn all_registrations(&self) -> &Vec<Registration>;
    fn add(&mut self, registration: Registration);    
}

#[cfg(test)]
mod registration_tests {
    use super::*;
    use super::registrations::*;

    #[test]
    fn registration_should_have_name_and_creation_date() {
        let name = "Fulano de Tal";
        let phone = "+5541123";

        let registration = Registration::new(name, phone);
        
        assert_eq!(name, &registration.name);
        assert_eq!(phone, &registration.phone);
        assert!(registration.created_on <= Utc::now());        
    }

    #[test]
    fn registration_manager_impl_should_add_new_register() -> Result<(), RegistrationManagerError> {
        let mut registration_manager = RegistrationManagerImpl::new(RegistrationsInMemory::new());
        let name = "Fulano de Tal";
        let phone = "+5541123";
        
        registration_manager.add(name, phone)?;

        let all_registrations: &Vec<Registration> = registration_manager.get_all_registrations();
        assert_eq!(1, all_registrations.len());
        Ok(())
    }
}
