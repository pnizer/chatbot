mod chat_state;
pub mod context;

use std::{sync::Arc, cell::RefCell, collections::HashMap};
use mockall::automock;
use crate::{telegram::{TelegramMessageArrived, TelegramListener, TelegramSender, SendTelegramMessage}, state_machine::StateMachine};

use self::chat_state::{States, ChatState};

#[automock]
pub trait StateMachineBuilder {
    fn build(&self, state_data: HashMap<String, String>) -> StateMachine;
}

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
    telegram_sender: Arc<dyn TelegramSender>,
    state_machine_builder: Box<dyn StateMachineBuilder>,
}
impl MessagesGateway {
    pub fn new(
            states: Arc<RefCell<dyn States>>,
            telegram_sender: Arc<dyn TelegramSender>,
            state_machine_builder: Box<dyn StateMachineBuilder>,
        ) -> Self {
        Self {
            states,
            telegram_sender,
            state_machine_builder,
        }
    }

    fn message_arrived(&self, message: Message) {
        let chat_id = message.chat_id();
        let state = self.states.borrow_mut().get(&chat_id);        
        
        let mut state_machine = if let Some(s) = state {            
            let mut state_machine = self.state_machine_builder.build(s.data.clone());
            state_machine.set_current_state(&s.current_state).unwrap();
            state_machine
        } else {
            let state_machine = self.state_machine_builder.build(HashMap::new());            
            state_machine
        };

        if let Ok(output) = state_machine.transition_state(&message.text()) {
            let (transition_output, state_output) = output;
            if let Some(text) = transition_output {
                self.answer_message(&message, &text);
            }
            if let Some(text) = state_output {
                self.answer_message(&message, &text);
            }

            self.states.borrow_mut().change_state(&chat_id, ChatState {                
                current_state: state_machine.get_current_state().unwrap(),
                data: state_machine.get_state_data().clone(),
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
    use std::{cell::RefCell, sync::Arc, collections::HashMap};
    use crate::{telegram::MockTelegramSender, state_machine::{State, transitions::{EqTransitionRule, DefaultTransitionRule, FixedTransitionOutput}, state_output::FixedStateOutput}};
    use super::{*, chat_state::MockStates};

    struct TestScope {
        mock_states: MockStates,
        mock_telegram_sender: MockTelegramSender,
        state_machine_builder: MockStateMachineBuilder,
    }
    impl TestScope {
        fn new() -> Self {
            Self {
                mock_states: MockStates::new(),
                mock_telegram_sender: MockTelegramSender::new(),
                state_machine_builder: MockStateMachineBuilder::new(),
            }
        }

        fn build_object(self) -> MessagesGateway {
            MessagesGateway::new(
                Arc::new(RefCell::new(self.mock_states)), 
                Arc::new(self.mock_telegram_sender),
                Box::new(self.state_machine_builder),
            )
        }
    }

    fn build_state_machine(state_data: HashMap<String, String>) -> StateMachine {
        let mut state_machine: StateMachine = StateMachine::new(state_data);
        let name_1 = "state-1";
        let name_2 = "state-2";
        let mut state = State::new(name_1);
        state.add_transition(name_2, EqTransitionRule::new("1"));
        state.add_transition_with_output(name_1, DefaultTransitionRule::new(), FixedTransitionOutput::new("invalid option"));
        state_machine.add_state(state);
        let mut state = State::new(name_2);
        state.set_output(FixedStateOutput::new("this is state 2!"));
        state.add_transition(name_1, EqTransitionRule::new("2"));
        state.add_transition_with_output(name_2, DefaultTransitionRule::new(), FixedTransitionOutput::new("invalid option"));
        state_machine.add_state(state);
        state_machine.set_initial_state_name(name_1).unwrap();
        
        state_machine
    }

    #[test]
    fn message_gateway_should_get_state_using_chat_id() {
        let mut scope = TestScope::new();
        scope.state_machine_builder.expect_build().return_once(build_state_machine);
        scope.mock_states.expect_get()            
            .withf(|chat_id| chat_id == "111000")
            .return_once(move |_| None);        
        let telegram_message = TelegramMessageArrived {
            from: Some("userName".to_string()),
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
        scope.state_machine_builder.expect_build().return_once(build_state_machine);
        scope.mock_states.expect_get().return_once(move |_| None);
        let telegram_message = TelegramMessageArrived {
            from: Some("userName".to_string()),
            message_id: 111000,
            chat_id: 111000,
            text: "1".to_string(),
        };
        scope.mock_telegram_sender.expect_send_message().return_const(());
        scope.mock_states.expect_change_state()
            .withf(|chat_id, state| chat_id == "111000" && state.current_state == "state-2")
            .return_const(());
        let message_gateway = scope.build_object();

        <dyn TelegramListener>::message_arrived(&message_gateway, telegram_message);
    }    

    #[test]
    fn message_gateway_should_send_telegram_message_with_state_output() {
        let mut scope = TestScope::new();
        scope.state_machine_builder.expect_build().return_once(build_state_machine);
        scope.mock_states.expect_get().return_once(move |_| None);        
        let telegram_message = TelegramMessageArrived {
            from: Some("userName".to_string()),
            message_id: 111000,
            chat_id: 111000,
            text: "1".to_string(),
        };
        scope.mock_telegram_sender.expect_send_message()
            .withf(|message| message.chat_id == 111000 && message.text == "this is state 2!")
            .return_const(());
        scope.mock_states.expect_change_state().return_const(());
        let message_gateway = scope.build_object();

        <dyn TelegramListener>::message_arrived(&message_gateway, telegram_message);
    }    

    #[test]
    fn message_gateway_should_send_telegram_message_with_state_and_transition_output() {
        let mut scope = TestScope::new();
        let chat_state = ChatState {
            current_state: String::from("state-2"),
            data: HashMap::new(),
        };
        scope.mock_states.expect_get().return_once(move |_| Some(chat_state));        
        scope.state_machine_builder.expect_build().return_once(build_state_machine);
        let telegram_message = TelegramMessageArrived {
            from: Some("userName".to_string()),
            message_id: 111000,
            chat_id: 111000,
            text: "1".to_string(),
        };
        scope.mock_telegram_sender.expect_send_message()
            .withf(|message| message.chat_id == 111000 && message.text == "invalid option")
            .return_const(());
        scope.mock_telegram_sender.expect_send_message()
            .withf(|message| message.chat_id == 111000 && message.text == "this is state 2!")
            .return_const(());
            scope.mock_states.expect_change_state().return_const(());
        let message_gateway = scope.build_object();

        <dyn TelegramListener>::message_arrived(&message_gateway, telegram_message);
    }    

    #[test]
    fn message_gateway_should_load_current_state_from_repository() {
        let mut scope = TestScope::new();
        let chat_state = ChatState {
            current_state: String::from("state-2"),
            data: HashMap::new(),
        };
        scope.state_machine_builder.expect_build().return_once(build_state_machine);
        scope.mock_states.expect_get()
            .withf(|chat_id| chat_id == "111000")
            .return_once(move |_| Some(chat_state));        
        let telegram_message = TelegramMessageArrived {
            from: Some("userName".to_string()),
            message_id: 111000,
            chat_id: 111000,
            text: "2".to_string(),
        };
        scope.mock_telegram_sender.expect_send_message().return_const(());
        scope.mock_states.expect_change_state()
            .withf(|_chat_id, state| state.current_state == "state-1")
            .return_const(());
        let message_gateway = scope.build_object();

        <dyn TelegramListener>::message_arrived(&message_gateway, telegram_message);
    }    
}
