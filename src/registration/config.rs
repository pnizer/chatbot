use super::{RegistrationManagerImpl, Registrations, registrations::RegistrationsInMemory, RegistrationManager};

pub fn build_registration_manager() -> impl RegistrationManager {
    RegistrationManagerImpl::new(build_registrations())
}

fn build_registrations() -> impl Registrations {
    RegistrationsInMemory::new()
}
