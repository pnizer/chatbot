use crate::telegram::context::TelegramContext;

use super::chatbot::context::ChatbotContext;
use super::registration::context::RegistrationContext;

pub struct ApplicationContext {
    pub registration_context: RegistrationContext,
    pub chatbot_context: ChatbotContext,
    pub telegram_context: TelegramContext,
}
impl ApplicationContext {
    pub fn build() -> Self {
        let registration_context = RegistrationContext::build();
        let chatbot_context = ChatbotContext::build();
        let telegram_context = TelegramContext::build();

        Self {
            registration_context,
            chatbot_context,
            telegram_context,
        }
    }
}
