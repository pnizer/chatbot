pub mod context;

use std::{sync::{mpsc, Arc}, thread};

use mockall::automock;

#[derive(Debug, Clone)]
pub struct TelegramMessageArrived {
    pub from: Option<String>,
    pub message_id: i64,
    pub chat_id: i64,
    pub text: String,
}

pub struct SendTelegramMessage {
    pub chat_id: i64,
    pub text: String,
}

pub trait TelegramListener {
    fn message_arrived(&self, message: TelegramMessageArrived);
}

pub trait TelegramReceiver {
    fn add_message_arrived_listener(&mut self, listener: Arc<dyn TelegramListener>);
    fn start_receive(&self);
}

struct LongPollingTelegramReceiver {
    token: String,
    listeners: Vec<Arc<dyn TelegramListener>>,
}
impl LongPollingTelegramReceiver {
    fn new(token: &str) -> Self {
        Self {
            token: String::from(token),
            listeners: Vec::new(),
        }
    }

    fn poll_messages(&self) {
        let (tx, rx) = mpsc::channel();
        let token = String::from(&self.token);
        thread::spawn(move || {
            let mut last_offset: Option<i64> = None;
            loop {
                println!("GetUpdate...");
                let timeout = 15;
                let offset = match last_offset {
                    Some(n) => (n + 1).to_string(),
                    None => "".to_string()
                };
                let url = format!("https://api.telegram.org/bot{}/getUpdates?timeout={}&offset={}", &token, timeout, &offset);
                let resp: serde_json::Value = reqwest::blocking::get(url).unwrap()
                    .json().unwrap();

                println!("{:?}", resp);
                for result in resp["result"].as_array().unwrap() {
                    last_offset = Some(result["update_id"].as_i64().unwrap());
                    
                    if result["message"].is_object() {
                        let username = match result["message"]["from"]["username"].as_str() {
                            Some(name) => Some(String::from(name)),
                            None => None
                        };
                        let message = TelegramMessageArrived {
                            from: username,
                            message_id: result["message"]["message_id"].as_i64().unwrap(),
                            chat_id: result["message"]["chat"]["id"].as_i64().unwrap(),
                            text: String::from(result["message"]["text"].as_str().unwrap()),
                        };
                        tx.send(message).unwrap();
                    }                    

                }
            }    
        });
                    
        for message in rx {
            for listener in &self.listeners {
                listener.message_arrived(message.clone());
            }
            println!("{:?}", message);
        }
    }
}
impl TelegramReceiver for LongPollingTelegramReceiver {
    fn add_message_arrived_listener(&mut self, listener: Arc<dyn TelegramListener>) {
        self.listeners.push(listener);
    }

    fn start_receive(&self) {
        self.poll_messages();
    }
}

#[automock]
pub trait TelegramSender {
    fn send_message(&self, message: SendTelegramMessage);
}
struct TelegramSenderImpl {
    token: String,
}
impl TelegramSenderImpl {
    fn new(token: &str) -> Self {
        Self {
            token: String::from(token),
        }
    }
}
impl TelegramSender for TelegramSenderImpl {
    fn send_message(&self, message: SendTelegramMessage) {
        let chat_id = message.chat_id;        
        let text = &message.text;
        let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
            &self.token, 
            chat_id, 
            urlencoding::encode(text),
        );

        let result = reqwest::blocking::get(url).unwrap();
        let status = result.status();
        println!("{}", status);
        if !status.is_success() {            
            if let Ok(json) = result.json::<serde_json::Value>() {
                println!("{:?}", json);
            }    
        }        
    }
}
