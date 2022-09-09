#![allow(dead_code)]
use chatbot::build_chatbot_state_machine;
use std::{io::{self, BufRead, Error}};

mod whatsapp_messages;
mod messages;
mod state_machine;
mod chatbot;
mod registration;

fn main() -> Result<(), Error> {
    let mut chatbot = build_chatbot_state_machine();
    let stdin = io::stdin();    
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        let (transition_output, state_output) = chatbot.state_machine.transition_state(&line).unwrap();
        if let Some(s) = transition_output {
            println!("{}", &s);
        }
        if let Some(s) = state_output {
            println!("{}", &s);
        }
    }
    Ok(())
}

