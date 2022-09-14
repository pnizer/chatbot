#![allow(dead_code)]
use chatbot::ChatbotBuilder;
use context::ApplicationContext;
use messages_gateway::{MessagesGateway, StateMachineBuilder};
use telegram::TelegramReceiver;
use std::{io::{self, BufRead, Error}, sync::Arc};

mod telegram;
mod state_machine;
mod chatbot;
mod registration;
mod context;
mod messages_gateway;
mod test;

fn main() {
    run_telegram_bot();
}

fn run_telegram_bot() {
    let application_context = ApplicationContext::build();

    let message_gateway = Arc::new(MessagesGateway::new(
        application_context.messages_gateway_context.states.clone(),
        application_context.telegram_context.telegram_sender.clone(),
        Box::new(ChatbotBuilder::new(
            application_context.registration_context.registration_manager.clone())
        ),
    ));

    let mut receiver = application_context.telegram_context.new_telegram_receiver();
    receiver.add_message_arrived_listener(message_gateway.clone());
    receiver.start_receive();
}

fn run_terminal_bot() -> Result<(), Error> {
    let application_context = ApplicationContext::build();
    let chatbot_builder = ChatbotBuilder::new(
        application_context.registration_context.registration_manager.clone()
    );
    let mut chatbot = chatbot_builder.build();
    let stdin = io::stdin();    
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        let (transition_output, state_output) = chatbot.transition_state(&line).unwrap();
        if let Some(s) = transition_output {
            println!("{}", &s);
        }
        if let Some(s) = state_output {
            println!("{}", &s);
        }
    }
    Ok(())
}
