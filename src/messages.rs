use std::collections::HashMap;
use async_trait::async_trait;

struct MessagesProcessor<'a> {
    chat_state_storage: &'a mut dyn ChatStateStorage,
    // state_machine: &'a mut StateMachine<S, A, T>,
}

impl <'a> MessagesProcessor<'a> {
    fn new(chat_state_storage: &'a mut dyn ChatStateStorage) -> Self {
        Self {
            chat_state_storage,
        }
    }

    async fn message_arrived(&mut self, event: MessageArrived) {        
        let current_state = self.chat_state_storage.retrive_chat_state_for(&event.source, event.message_type).await;
        let new_state = match current_state {
            Some(state) => self.process_next_state_and_response(state, &event.content),
            None => ChatState { message_count: 1 }
        };

        self.chat_state_storage.push_chat_state_for(&event.source, event.message_type, new_state).await;
    }

    fn process_next_state_and_response(&self, current_state: &ChatState, _message_content: &str) -> ChatState {
        ChatState { 
            message_count: current_state.message_count + 1,
        }
    }
}

struct ChatState {
    message_count: i32,
}

#[async_trait]
trait ChatStateStorage {
    async fn retrive_chat_state_for(&self, source: &str, message_type: MessageType) -> Option<&ChatState>;
    async fn push_chat_state_for(&mut self, source: &str, message_type: MessageType, chat_state: ChatState);
}

struct InMemoryChatStateStorage {
    chat_states: HashMap<(String, MessageType), ChatState>,
}
impl InMemoryChatStateStorage {
    fn new() -> Self {
        Self {
            chat_states: HashMap::new(),
        }
    }

    fn key_for(source: &str, message_type: MessageType) -> (String, MessageType) {
        (String::from(source), message_type)
    }
}
#[async_trait]
impl ChatStateStorage for InMemoryChatStateStorage {
    async fn retrive_chat_state_for(&self, source: &str, message_type: MessageType) -> Option<&ChatState> {        
        self.chat_states.get(&Self::key_for(source, message_type))
    }

    async fn push_chat_state_for(&mut self, source: &str, message_type: MessageType, chat_state: ChatState) {
        self.chat_states.insert(Self::key_for(source, message_type), chat_state);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MessageType {
    Whatsapp,
    Telegram,
    Sms,
}

#[derive(Debug)]
pub struct MessageArrived {    
    pub message_type: MessageType,
    pub source: String,
    pub content: String,
}

#[cfg(test)]
mod messages_test {
    use super::*;

    #[actix_rt::test]
    async fn in_memory_chat_state_storage_should_retrive_pushed_state() {
        let source = String::from("+5541123");
        let mut state_storage = InMemoryChatStateStorage::new();
        let chat_state = ChatState { message_count: 1 };
        state_storage.push_chat_state_for(&source, MessageType::Whatsapp, chat_state).await;
        
        let chat_state = state_storage.retrive_chat_state_for(&source, MessageType::Whatsapp).await;

        assert_eq!(true, chat_state.is_some());
    }

    #[actix_rt::test]
    async fn processor_should_modify_chat_state() {
        let mut state_storage = InMemoryChatStateStorage::new();
        let mut processor = MessagesProcessor::new(&mut state_storage);        
        let message_arrived = MessageArrived { 
            message_type: MessageType::Whatsapp, 
            source: String::from("+5541123"), 
            content: String::from("hello") 
        };

        processor.message_arrived(message_arrived).await;

        let state = state_storage.retrive_chat_state_for( "+5541123", MessageType::Whatsapp).await;
        assert!(state.is_some());
        assert_eq!(1, state.unwrap().message_count);
    }

    #[actix_rt::test]
    async fn processor_should_increment_state_count() {
        let initial_message_count = 4;
        let mut state_storage = InMemoryChatStateStorage::new();        
        state_storage.push_chat_state_for("+5541123", MessageType::Whatsapp, ChatState { message_count: initial_message_count }).await;
        let mut processor = MessagesProcessor::new(&mut state_storage);                
        let message_arrived = MessageArrived { 
            message_type: MessageType::Whatsapp, 
            source: String::from("+5541123"), 
            content: String::from("hello") 
        };

        processor.message_arrived(message_arrived).await;

        let state = state_storage.retrive_chat_state_for( "+5541123", MessageType::Whatsapp).await;
        assert!(state.is_some());
        assert_eq!(initial_message_count + 1, state.unwrap().message_count);
    }

    #[actix_rt::test]
    async fn processor_should_send_message() {

    }

}