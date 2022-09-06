use super::messages::*;

struct WhatsappMessagesReceiver<'a, F>
where F: FnMut(MessageArrived)
{
    listener: &'a mut F,
}

impl <'a, F> WhatsappMessagesReceiver<'a, F> 
where F: FnMut(MessageArrived)
{
    pub fn new(listener: &'a mut F) -> Self 
        where F: FnMut(MessageArrived) {
        WhatsappMessagesReceiver { 
            listener,
         }
    }

    pub fn message_arrived(&mut self, message: WhatsappMessage) {
        let event = MessageArrived {
            message_type: MessageType::Whatsapp,
            source: message.phone_number,
            content: message.content,
        };
        (self.listener)(event);
    }
}

#[derive(Debug)]
struct WhatsappMessage {
    phone_number: String,
    content: String,
}

#[cfg(test)]
mod whatsapp_messages_test {
    use super::*;

    #[test]
    fn receiver_should_call_processor_when_receive_whatsapp_message() {
        let mut last_event: Option<MessageArrived> = Option::None;
        let mut listener = |event: MessageArrived| {            
            last_event = Option::Some(event);
        };
        let mut receiver = WhatsappMessagesReceiver::new(&mut listener);
        let message = WhatsappMessage {
            phone_number: String::from("+41123123"),
            content: String::from("Fizz buzz"),
        };

        receiver.message_arrived(message);

        assert_eq!(true, last_event.is_some());
        assert_eq!("+41123123", &last_event.as_ref().unwrap().source);
        assert_eq!("Fizz buzz", &last_event.as_ref().unwrap().content);
        assert!(matches!(last_event.as_ref().unwrap().message_type, MessageType::Whatsapp));
    }

}
