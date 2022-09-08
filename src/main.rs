#![allow(dead_code)]
use chatbot::init_chatbot_state_machine;
use state_machine::StateMachineErrors;
use std::io::{self, BufRead};

mod whatsapp_messages;
mod messages;
mod state_machine;
mod chatbot;
mod registration;

fn main() -> Result<(), StateMachineErrors> {
    let mut chatbot = init_chatbot_state_machine()?;    
    let stdin = io::stdin();    
    for line_result in stdin.lock().lines() {
        match line_result {
            Err(_e) => { return Err(StateMachineErrors::StateNotFound); },
            Ok(line) => { 
                let (transition_output, state_output) = chatbot.transition_state(&line)?;
                if let Some(s) = transition_output {
                    println!("{}", &s);
                }
                if let Some(s) = state_output {
                    println!("{}", &s);
                }
            }
        }        
    }

    Ok(())
}

