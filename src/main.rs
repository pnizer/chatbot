#![allow(dead_code)]
use chatbot::build_chatbot_state_machine;
use context::ApplicationContext;
use messages_gateway::MessagesGateway;
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
    let application_context = ApplicationContext::build();

    let message_gateway = Arc::new(MessagesGateway::new(
        application_context.chatbot_context.states.clone(),
        application_context.registration_context.registration_manager.clone(),
        application_context.telegram_context.telegram_sender.clone(),        
    ));

    let mut receiver = application_context.telegram_context.new_telegram_receiver();
    receiver.add_message_arrived_listener(message_gateway.clone());
    receiver.start_receive();
}

fn run_terminal_bot() -> Result<(), Error> {
    let application_context = ApplicationContext::build();
    let mut chatbot = build_chatbot_state_machine(&application_context);
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