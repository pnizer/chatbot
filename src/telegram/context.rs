use std::{sync::Arc, env};

use super::*;

pub struct TelegramContext {    
    pub telegram_sender: Arc<dyn TelegramSender>,
}
impl TelegramContext {
    pub fn build() -> Self {
        let telegram_sender = Arc::new(Self::build_telegram_sender());

        Self {
            telegram_sender,
        }
    }

    fn build_telegram_sender() -> impl TelegramSender {                    
        let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
        TelegramSenderImpl::new(&token)
    }

    pub fn new_telegram_receiver(&self) -> impl TelegramReceiver {
        let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
        LongPollingTelegramReceiver::new(&token)
    }
}
