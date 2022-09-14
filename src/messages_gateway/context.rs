use std::{sync::Arc, cell::RefCell};

use super::chat_state::*;

pub struct MessagesGatewayContext {    
    pub states: Arc<RefCell<dyn States>>,
}
impl MessagesGatewayContext {
    pub fn build() -> Self {
        let states = Arc::new(RefCell::new(Self::build_states()));

        Self {
            states,
        }
    }

    fn build_states() -> impl States {            
        StatesInMemory::new()
    }
}
