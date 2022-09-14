use crate::telegram::context::TelegramContext;

use super::messages_gateway::context::MessagesGatewayContext;
use super::registration::context::RegistrationContext;

pub struct ApplicationContext {
    pub registration_context: RegistrationContext,
    pub messages_gateway_context: MessagesGatewayContext,
    pub telegram_context: TelegramContext,
}
impl ApplicationContext {
    pub fn build() -> Self {
        let registration_context = RegistrationContext::build();
        let chatbot_context = MessagesGatewayContext::build();
        let telegram_context = TelegramContext::build();

        Self {
            registration_context,
            messages_gateway_context: chatbot_context,
            telegram_context,
        }
    }
}
