use std::{sync::Arc, cell::RefCell};

use super::{RegistrationManager, Registrations, registrations::RegistrationsInMemory, RegistrationManagerImpl};

pub struct RegistrationContext {
    pub registration_manager: Arc<RefCell<dyn RegistrationManager>>,
    registrations: Arc<RefCell<dyn Registrations>>,
}
impl RegistrationContext {
    pub fn build() -> Self {
        let registrations = Arc::new(RefCell::new(Self::build_registrations()));
        let registration_manager = Arc::new(RefCell::new(Self::build_registration_manager(registrations.clone())));

        Self {
            registration_manager,
            registrations,
        }
    }

    fn build_registration_manager(registrations: Arc<RefCell<dyn Registrations>>) -> impl RegistrationManager {
        RegistrationManagerImpl::new(registrations)
    }

    fn build_registrations() -> impl Registrations {
        RegistrationsInMemory::new()
    }
}
