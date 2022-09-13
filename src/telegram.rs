use std::{rc::Rc, sync::mpsc, thread};

#[derive(Debug)]
struct TelegramMessageArrived {
    from: String,
    message_id: i64,
    chat_id: i64,
    text: String,
}

struct SendTelegramMessage {
    chat_id: i64,
    text: String,
}

trait TelegramListener {
    fn message_arrived(&self, message: &TelegramMessageArrived);
}

trait TelegramReceiver {
    fn add_message_arrived_listener(&mut self, listener: Rc<dyn TelegramListener>);
}

struct LongPollingTelegramReceiver {
    token: String,
    listeners: Vec<Rc<dyn TelegramListener>>,
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
                let timeout = 10;
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
                        let message = TelegramMessageArrived {
                            from: String::from(result["message"]["from"]["username"].as_str().unwrap()),
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
                listener.message_arrived(&message);
            }
            println!("{:?}", message);
        }
    }
}
impl TelegramReceiver for LongPollingTelegramReceiver {
    fn add_message_arrived_listener(&mut self, listener: Rc<dyn TelegramListener>) {
        self.listeners.push(listener);
    }
}

trait TelegramSender {
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
        let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}", &self.token, chat_id, text);
        let status = reqwest::blocking::get(url).unwrap().status();
        println!("{}", status);
    }
}

#[cfg(test)]
mod telegram_test {
    use std::env;

    use super::*;

    #[test]
    fn telegram_should_longpoll_get_updates() {
        let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
        // let receiver = LongPollingTelegramReceiver::new();
        // receiver.poll_messages();

        let sender = TelegramSenderImpl::new(&token);
        let message = SendTelegramMessage {
            chat_id: 851269483,
            text: "Hey Ho!".to_string(),
        };

        sender.send_message(message);
    }
}
