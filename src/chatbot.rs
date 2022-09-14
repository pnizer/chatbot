use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use crate::messages_gateway::StateMachineBuilder;
use crate::state_machine::form_states::*;

use super::{registration::RegistrationManager};
use super::{state_machine::*, state_machine::transitions::*, state_machine::state_output::*};

const INITIAL_STATE_NAME: &str = "start";
const MENU_STATE_NAME: &str = "menu";
const REGISTER_FIELD_PREFIX: &str = "register-";
const REGISTER_FIELD_INITIAL_STATE: &str = "register-name";
const REGISTER_FIELD_FINISHED_STATE: &str = "register-finished";
const MENU_MESSAGE: &str = "1: Novo registro\n2: Lista de registros";
const REGISTER_NAME_QUESTION: &str = "Qual o nome?";
const REGISTER_PHONE_QUESTION: &str = "Qual o telefone?";
const INVALID_MENU_MESSAGE: &str = "Menu inválido!";

pub struct ChatbotBuilder {
    registration_manager: Arc<RefCell<dyn RegistrationManager>>,
}
impl StateMachineBuilder for ChatbotBuilder {
    fn build(&self, state_data: HashMap<String, String>) -> StateMachine {
        let mut state_machine = StateMachine::new(state_data);
        self.build_initial_state(&mut state_machine);
        self.build_menu_state(&mut state_machine);
        self.build_register_form(&mut state_machine);
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
        menu_state.add_transition(REGISTER_FIELD_INITIAL_STATE, EqTransitionRule::new("1"));
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

    fn build_register_form(&self, state_machine: &mut StateMachine) {
        let mut form_states = FormStates::new("register-");
        form_states.add_field("name", REGISTER_NAME_QUESTION, FieldType::String, FieldOption::Required);
        form_states.add_field("phone", REGISTER_PHONE_QUESTION, FieldType::String, FieldOption::Required);        
        form_states.apply_states(REGISTER_FIELD_FINISHED_STATE, state_machine);

        let mut register_finished = State::new(REGISTER_FIELD_FINISHED_STATE);
        register_finished.set_output(FnStateOutput::new(|data| {
            let mut output = String::new();
            output.push_str("Nome: "); 
            output.push_str(&data.get("register-name").unwrap());
            output.push_str("\n");
            output.push_str("Telefone: ");
            output.push_str(&data.get("register-phone").unwrap());
            output.push_str("\n\n");
            output.push_str("Confirmar? (sim, não ou cancelar)");
            Some(output)
        }));
        register_finished.add_transition(MENU_STATE_NAME, EqTransitionRule::new("cancelar"));
        register_finished.add_transition(REGISTER_FIELD_INITIAL_STATE, EqTransitionRule::new("não"));
        let registration_manager_arc = self.registration_manager.clone();
        register_finished.add_transition(MENU_STATE_NAME, FnTransitionRule::new(
            move |data, action| {
                if "sim" == action {
                    let mut registration_manager = registration_manager_arc.borrow_mut();
                    registration_manager.add(&data.get("register-name").unwrap(), &data.get("register-phone").unwrap()).unwrap();
                    true            
                } else {
                    false
                }
            }
        ));
        state_machine.add_state(register_finished);
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
        let mut chatbot = chatbot_builder.build(HashMap::new());        

        let response = chatbot.transition_state("1")?;

        assert_eq!(MENU_MESSAGE, response.1.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_ask_register_name() -> Result<(), StateMachineErrors> {        
        let registration_manager = MockRegistrationManager::new();
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build(HashMap::new());        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("1")?;

        assert_eq!(REGISTER_NAME_QUESTION, response.1.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_back_to_menu_after_name_registered() -> Result<(), StateMachineErrors> {
        let mut registration_manager = MockRegistrationManager::new();
        registration_manager.expect_add()
            .withf(|name, phone| name == "José Ricardo" && phone == "123321")
            .return_once(|_,_| Ok(()));
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build(HashMap::new());        
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        chatbot.transition_state("José Ricardo")?;
        chatbot.transition_state("123321")?;
        chatbot.transition_state("sim")?;
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
        let mut chatbot = chatbot_builder.build(HashMap::new());        
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        chatbot.transition_state("Fulano")?;
        chatbot.transition_state("123123")?;
        chatbot.transition_state("sim")?;
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
        let mut chatbot = chatbot_builder.build(HashMap::new());        
        
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
        let mut chatbot = chatbot_builder.build(HashMap::new());        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("2")?;

        assert_eq!("Fulano\nBeltrano", response.0.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_menu_on_invalid_command() -> Result<(), StateMachineErrors> {
        let registration_manager = MockRegistrationManager::new();
        let chatbot_builder = ChatbotBuilder::new(Arc::new(RefCell::new(registration_manager)));
        let mut chatbot = chatbot_builder.build(HashMap::new());        
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("olá")?;         

        assert_eq!(INVALID_MENU_MESSAGE, response.0.unwrap());
        assert_eq!(MENU_MESSAGE, response.1.unwrap());
        Ok(())
    }
}
