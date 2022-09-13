#![allow(dead_code)]
use chatbot::build_chatbot_state_machine;
use context::ApplicationContext;
use std::{io::{self, BufRead, Error}};

mod telegram;
mod state_machine;
mod chatbot;
mod registration;
mod context;
mod test;

fn main() -> Result<(), Error> {
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

