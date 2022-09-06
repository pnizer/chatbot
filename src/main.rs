use chatbot::init_chatbot_state_machine;
use state_machine::StateMachineErrors;
use std::io::{self, BufRead};

mod whatsapp_messages;
mod messages;
mod state_machine;
mod chatbot;

fn main() -> Result<(), StateMachineErrors> {
    let mut chatbot = init_chatbot_state_machine()?;    
    let stdin = io::stdin();    
    for line_result in stdin.lock().lines() {
        match line_result {
            Err(_e) => { return Err(StateMachineErrors::StateNotFound); },
            Ok(line) => { 
                let output = chatbot.transition_state(&line)?;
                if let Some(s) = output {
                    println!("{}", &s);
                }
            }
        }        
    }

    Ok(())
}

