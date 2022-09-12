use std::cell::RefCell;
use std::sync::Arc;

use super::{registration::RegistrationManager, context::ApplicationContext};
use super::{state_machine::*, state_machine::transitions::*, state_machine::state_output::*};

const INITIAL_STATE_NAME: &str = "start";
const MENU_STATE_NAME: &str = "menu";
const REGISTE_STATE_NAME: &str = "register";
const MENU_MESSAGE: &str = "1: Novo registro\n2: Lista de registros";
const REGISTER_NAME_QUESTION: &str = "Qual o nome do registro?";
const INVALID_MENU_MESSAGE: &str = "Menu inválido!";

pub fn build_chatbot_state_machine(application_context: &ApplicationContext) -> ChatbotStateMachine {        
    let mut chatbot = ChatbotStateMachine::new(application_context.registration_context.registration_manager.clone());
    chatbot.init();
    chatbot
}

pub struct ChatbotStateMachine {
    pub state_machine: StateMachine,
    registration_manager: Arc<RefCell<dyn RegistrationManager>>,
}
impl ChatbotStateMachine {
    pub fn new(registration_manager: Arc<RefCell<dyn RegistrationManager>>) -> Self {
        Self {
            state_machine: StateMachine::new(""),
            registration_manager,
        }
    }

    pub fn init(&mut self) {
        self.build_initial_state();
        self.build_menu_state();
        self.build_register_state();
    }
    
    fn build_initial_state(&mut self) {
        let mut initial_state = State::new(INITIAL_STATE_NAME);
        initial_state.add_transition(MENU_STATE_NAME, DefaultTransitionRule::new());
        self.state_machine.add_state(initial_state);
        self.state_machine.set_initial_state_name(INITIAL_STATE_NAME).unwrap();
    }

    fn build_menu_state(&mut self) {
        let mut menu_state = State::new(MENU_STATE_NAME);
        menu_state.set_output(FixedStateOutput::new(MENU_MESSAGE));
        menu_state.add_transition(REGISTE_STATE_NAME, EqTransitionRule::new("1"));
        let registration_manager_arc = self.registration_manager.clone();
        menu_state.add_transition_with_output(MENU_STATE_NAME, EqTransitionRule::new("2"), FnTransitionOutput::new(
            move |_data, _action| {
                let registration_manager = registration_manager_arc.borrow();
                let registers = registration_manager.get_all_registrations();                
                let mut names = Vec::new();
                for r in registers {
                    names.push(r.name.clone());
                }
                Some(names.join("\n"))
            }
        ));        
        menu_state.add_transition_with_output(MENU_STATE_NAME, DefaultTransitionRule::new(), FixedTransitionOutput::new(INVALID_MENU_MESSAGE));
        self.state_machine.add_state(menu_state);
    }

    fn build_register_state(&mut self) {
        let mut register_state = State::new(REGISTE_STATE_NAME);
        register_state.set_output(FixedStateOutput::new(REGISTER_NAME_QUESTION));        
        let registration_manager_arc = self.registration_manager.clone();
        register_state.add_transition(MENU_STATE_NAME, FnTransitionRule::new(
            move |_data, action| {
                let mut registration_manager = registration_manager_arc.borrow_mut();
                match registration_manager.add(action, "+5541123") {
                    Ok(_) => true,
                    Err(_) => false,
                }                
            }
        ));
        register_state.add_transition_with_output(MENU_STATE_NAME, DefaultTransitionRule::new(), FixedTransitionOutput::new("Erro ao cadastar!"));
        self.state_machine.add_state(register_state);
    }

    pub fn transition_state(&mut self, action: &str) -> Result<(Option<String>, Option<String>), StateMachineErrors> {
        self.state_machine.transition_state(action)
    }

}


#[cfg(test)]
mod chatbot_tests {
    use super::*;

    #[test]
    fn chatbot_should_have_initial_message() -> Result<(), StateMachineErrors> {
        let application_context = ApplicationContext::build();
        let mut chatbot = build_chatbot_state_machine(&application_context);        

        let response = chatbot.transition_state("1")?;

        assert_eq!(MENU_MESSAGE, response.1.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_ask_register_name() -> Result<(), StateMachineErrors> {        
        let application_context = ApplicationContext::build();
        let mut chatbot = build_chatbot_state_machine(&application_context);        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("1")?;

        assert_eq!(REGISTER_NAME_QUESTION, response.1.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_back_to_menu_after_name_registered() -> Result<(), StateMachineErrors> {
        let application_context = ApplicationContext::build();
        let mut chatbot = build_chatbot_state_machine(&application_context);        
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        let response = chatbot.transition_state("José Ricardo")?;

        assert_eq!(MENU_MESSAGE, response.1.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_register_list_after_back_to_menu() -> Result<(), StateMachineErrors> {
        let application_context = ApplicationContext::build();
        let mut chatbot = build_chatbot_state_machine(&application_context);        
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        chatbot.transition_state("Fulano")?;
        let response = chatbot.transition_state("2")?;

        assert_eq!("Fulano", response.0.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_empty_register_list() -> Result<(), StateMachineErrors> {
        let application_context = ApplicationContext::build();
        let mut chatbot = build_chatbot_state_machine(&application_context);        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("2")?;

        assert_eq!("", response.0.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_filled_register_list() -> Result<(), StateMachineErrors> {
        let application_context = ApplicationContext::build();
        let mut chatbot = build_chatbot_state_machine(&application_context);        
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        chatbot.transition_state("Fulano")?;
        chatbot.transition_state("1")?;
        chatbot.transition_state("Beltrano")?;
        let response = chatbot.transition_state("2")?;

        assert_eq!("Fulano\nBeltrano", response.0.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_menu_on_invalid_command() -> Result<(), StateMachineErrors> {
        let application_context = ApplicationContext::build();
        let mut chatbot = build_chatbot_state_machine(&application_context);        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("olá")?;         

        assert_eq!(MENU_MESSAGE, response.1.unwrap());
        Ok(())
    }
}
