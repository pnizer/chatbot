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
mod messages_gateway_tests {
    use std::{cell::RefCell, sync::Arc};
    use crate::{chatbot::chat_state::{MockStates, ChatState}, registration::MockRegistrationManager, telegram::MockTelegramSender};
    use super::*;

    struct TestScope {
        mock_states: MockStates,
        mock_registration_manager: MockRegistrationManager,
        mock_telegram_sender: MockTelegramSender,
    }
    impl TestScope {
        fn new() -> Self {
            Self {
                mock_states: MockStates::new(),
                mock_registration_manager: MockRegistrationManager::new(),
                mock_telegram_sender: MockTelegramSender::new(),
            }
        }

        fn build_object(self) -> MessagesGateway {
            MessagesGateway::new(
                Arc::new(RefCell::new(self.mock_states)), 
                Arc::new(RefCell::new(self.mock_registration_manager)),
                Arc::new(self.mock_telegram_sender),
            )
        }
    }

    #[test]
    fn message_gateway_should_get_state_using_chat_id() {
        let mut scope = TestScope::new();
        let chat_state = ChatState {
            current_state: String::from("menu"),
        };
        scope.mock_states.expect_get()            
            .withf(|chat_id| chat_id == "111000")
            .return_once(move |_| Some(chat_state));        
        let telegram_message = TelegramMessageArrived {
            from: "userName".to_string(),
            message_id: 111000,
            chat_id: 111000,
            text: "1".to_string(),
        };
        scope.mock_telegram_sender.expect_send_message().return_const(());
        scope.mock_states.expect_change_state().return_const(());
        let message_gateway = scope.build_object();

        <dyn TelegramListener>::message_arrived(&message_gateway, telegram_message);
    }    

    #[test]
    fn message_gateway_should_save_new_state_using_chat_id() {
        let mut scope = TestScope::new();
        let chat_state = ChatState {
            current_state: String::from("menu"),
        };
        scope.mock_states.expect_get().return_once(move |_| Some(chat_state));        
        let telegram_message = TelegramMessageArrived {
            from: "userName".to_string(),
            message_id: 111000,
            chat_id: 111000,
            text: "1".to_string(),
        };
        scope.mock_telegram_sender.expect_send_message().return_const(());
        scope.mock_states.expect_change_state()
            .withf(|chat_id, state| chat_id == "111000" && state.current_state == "register")
            .return_const(());
        let message_gateway = scope.build_object();

        <dyn TelegramListener>::message_arrived(&message_gateway, telegram_message);
    }    

    #[test]
    fn message_gateway_should_send_telegram_message_with_response() {
        let mut scope = TestScope::new();
        let chat_state = ChatState {
            current_state: String::from("menu"),
        };
        scope.mock_states.expect_get().return_once(move |_| Some(chat_state));        
        let telegram_message = TelegramMessageArrived {
            from: "userName".to_string(),
            message_id: 111000,
            chat_id: 111000,
            text: "1".to_string(),
        };
        scope.mock_telegram_sender.expect_send_message()
            .withf(|message| message.chat_id == 111000 && message.text == "Qual o nome do registro?")
            .return_const(());
        scope.mock_states.expect_change_state().return_const(());
        let message_gateway = scope.build_object();

        <dyn TelegramListener>::message_arrived(&message_gateway, telegram_message);
    }    
}
