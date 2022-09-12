use super::{Registrations, Registration};

pub struct RegistrationsInMemory {
    registrations: Vec<Registration>,
}
impl RegistrationsInMemory {
    pub fn new() -> Self {
        Self { registrations: Vec::new() }
    }
}
impl Registrations for RegistrationsInMemory {
    fn all_registrations(&self) -> Vec<Registration> {
        self.registrations.clone()
    }

    fn add(&mut self, registration: Registration) {
        self.registrations.push(registration);
    }
}

#[cfg(test)]
mod registrations_tests {
    use super::*;

    #[test]
    fn registration_in_memory_should_add_registrations() {
        let mut registrations: Box<dyn Registrations> = Box::new(RegistrationsInMemory::new());
        let registration_01 = Registration::new("Fulano One", "+55411");
        let registration_02 = Registration::new("Fulano Two", "+55412");
        let registration_03 = Registration::new("Fulano Three", "+55413");
        registrations.add(registration_01);
        registrations.add(registration_02);
        registrations.add(registration_03);

        let vec = registrations.all_registrations();

        assert_eq!(3, vec.len());
        assert_eq!("Fulano One", &vec[0].name);
        assert_eq!("Fulano Two", &vec[1].name);
        assert_eq!("Fulano Three", &vec[2].name);
    }
}
