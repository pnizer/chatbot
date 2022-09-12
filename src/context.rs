use super::registration::context::RegistrationContext;

pub struct ApplicationContext {
    pub registration_context: RegistrationContext,
}
impl ApplicationContext {
    pub fn build() -> Self {
        let registration_context = RegistrationContext::build();

        Self {
            registration_context,
        }
    }
}
