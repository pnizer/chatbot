use std::{sync::Arc, cell::RefCell};

use crate::{telegram::{TelegramMessageArrived, TelegramListener, TelegramSender, SendTelegramMessage}, chatbot::{chat_state::{States, ChatState}, ChatbotStateMachine}, registration::RegistrationManager};

enum Message {
    Telegram(TelegramMessageArrived),
}
impl Message {
    fn text(&self) -> String {
        match self {
            Self::Telegram(message) => message.text.to_string(),
        }
    }

    fn chat_id(&self) -> String {
        match self {
            Self::Telegram(message) => message.chat_id.to_string(),
        }
    }
}

pub struct MessagesGateway {
    states: Arc<RefCell<dyn States>>,
    registration_manager: Arc<RefCell<dyn RegistrationManager>>,
    telegram_sender: Arc<dyn TelegramSender>,
}
impl MessagesGateway {
    pub fn new(states: Arc<RefCell<dyn States>>, registration_manager: Arc<RefCell<dyn RegistrationManager>>, telegram_sender: Arc<dyn TelegramSender>) -> Self {
        Self {
            states,
            registration_manager,
            telegram_sender,
        }
    }

    fn message_arrived(&self, message: Message) {
        let chat_id = message.chat_id();
        let state = self.states.borrow_mut().get(&chat_id);
        let mut chatbot = ChatbotStateMachine::new(self.registration_manager.clone());
        chatbot.init();
        if let Some(s) = state {
            chatbot.set_current_state(&s.current_state).unwrap();
        }
        if let Ok(output) = chatbot.transition_state(&message.text()) {
            let (transition_output, state_output) = output;
            if let Some(text) = transition_output {
                self.answer_message(&message, &text);
            }
            if let Some(text) = state_output {
                self.answer_message(&message, &text);
            }

            self.states.borrow_mut().change_state(&chat_id, ChatState {
                current_state: chatbot.get_current_state().unwrap(),
            })
        }
    }

    fn answer_message(&self, arrived_message: &Message, text: &str) {
        match arrived_message {
            Message::Telegram(telegram_arrived_message) => {
                let new_message = SendTelegramMessage {
                    chat_id: telegram_arrived_message.chat_id,
                    text: text.to_string(),
                };
                self.telegram_sender.send_message(new_message);
            }
        }
    }
}
impl TelegramListener for MessagesGateway {
    fn message_arrived(&self, message: TelegramMessageArrived) {
        MessagesGateway::message_arrived(self, Message::Telegram(message));
    }
}

#[cfg(test)]
mod messages_gateway_tests {}

