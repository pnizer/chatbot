use std::collections::HashMap;

use mockall::automock;

#[derive(Debug, Clone)]
pub struct ChatState {
    pub data: HashMap<String, String>,
    pub current_state: String,
}

#[automock]
pub trait States {
    fn get(&self, chat_id: &str) -> Option<ChatState>;
    fn change_state(&mut self, chat_id: &str, state: ChatState);
}

pub struct StatesInMemory {
    states: HashMap<String, ChatState>,
}
impl StatesInMemory {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }
}
impl States for StatesInMemory {
    fn get(&self, chat_id: &str) -> Option<ChatState> {
        match self.states.get(chat_id) {
            Some(state) => Some(state.clone()),
            None => None,
        }
    }

    fn change_state(&mut self, chat_id: &str, state: ChatState) {
        self.states.insert(chat_id.to_string(), state);
    }
}
