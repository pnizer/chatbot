use std::cell::RefCell;
use std::sync::Arc;

use crate::messages_gateway::StateMachineBuilder;

use super::{registration::RegistrationManager};
use super::{state_machine::*, state_machine::transitions::*, state_machine::state_output::*};

const INITIAL_STATE_NAME: &str = "start";
const MENU_STATE_NAME: &str = "menu";
const REGISTE_STATE_NAME: &str = "register";
const MENU_MESSAGE: &str = "1: Novo registro\n2: Lista de registros";
const REGISTER_NAME_QUESTION: &str = "Qual o nome do registro?";
const INVALID_MENU_MESSAGE: &str = "Menu inválido!";

pub struct ChatbotBuilder {
    registration_manager: Arc<RefCell<dyn RegistrationManager>>,
}
impl StateMachineBuilder for ChatbotBuilder {
    fn build(&self) -> StateMachine {
        let mut state_machine = StateMachine::new("chat-bot");
        self.build_initial_state(&mut state_machine);
        self.build_menu_state(&mut state_machine);
        self.build_register_state(&mut state_machine);
        state_machine
    }
}
impl ChatbotBuilder {
    pub fn new(registration_manager: Arc<RefCell<dyn RegistrationManager>>) -> Self {
        Self {
            registration_manager,
        }
    }

    fn build_initial_state(&self, state_machine: &mut StateMachine) {
        let mut initial_state = State::new(INITIAL_STATE_NAME);
        initial_state.add_transition(MENU_STATE_NAME, DefaultTransitionRule::new());
        state_machine.add_state(initial_state);
        state_machine.set_initial_state_name(INITIAL_STATE_NAME).unwrap();
    }

    fn build_menu_state(&self, state_machine: &mut StateMachine) {
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
        state_machine.add_state(menu_state);
    }

    fn build_register_state(&self, state_machine: &mut StateMachine) {
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
        state_machine.add_state(register_state);
    }
}

#[cfg(test)]
mod chatbot_tests {
    use crate::registration::{MockRegistrationManager, Registration};

    use super::*;

    #[test]
    fn chatbot_should_have_initial_message() -> Result<(), StateMachineErrors> {
        let registration_manager = MockRegistrationManager::new();
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build();        

        let response = chatbot.transition_state("1")?;

        assert_eq!(MENU_MESSAGE, response.1.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_ask_register_name() -> Result<(), StateMachineErrors> {        
        let registration_manager = MockRegistrationManager::new();
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build();        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("1")?;

        assert_eq!(REGISTER_NAME_QUESTION, response.1.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_back_to_menu_after_name_registered() -> Result<(), StateMachineErrors> {
        let mut registration_manager = MockRegistrationManager::new();
        registration_manager.expect_add()
            .withf(|name, _phone| name == "José Ricardo")
            .return_once(|_,_| Ok(()));
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build();        
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        let response = chatbot.transition_state("José Ricardo")?;

        assert_eq!(MENU_MESSAGE, response.1.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_register_list_after_back_to_menu() -> Result<(), StateMachineErrors> {
        let mut registration_manager = MockRegistrationManager::new();
        registration_manager.expect_add()
            .withf(|name, _phone| name == "Fulano")
            .return_once(|_,_| Ok(()));        
        registration_manager.expect_get_all_registrations()            
            .return_once(move || Vec::from([Registration::new("Fulano", "+5541123")]));
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build();        
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        chatbot.transition_state("Fulano")?;
        let response = chatbot.transition_state("2")?;

        assert_eq!("Fulano", response.0.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_empty_register_list() -> Result<(), StateMachineErrors> {
        let mut registration_manager = MockRegistrationManager::new();
        registration_manager.expect_get_all_registrations()            
            .return_once(move || Vec::new());
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build();        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("2")?;

        assert_eq!("", response.0.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_filled_register_list() -> Result<(), StateMachineErrors> {
        let mut registration_manager = MockRegistrationManager::new();
        registration_manager.expect_get_all_registrations()            
            .return_once(move || Vec::from([
                Registration::new("Fulano", "+5541123"),
                Registration::new("Beltrano", "+5542223"),
            ]));
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build();        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("2")?;

        assert_eq!("Fulano\nBeltrano", response.0.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_menu_on_invalid_command() -> Result<(), StateMachineErrors> {
        let registration_manager = MockRegistrationManager::new();
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build();        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("olá")?;         

        assert_eq!(MENU_MESSAGE, response.1.unwrap());
        Ok(())
    }
}
