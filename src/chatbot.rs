use super::{state_machine::*, state_machine::transitions::*};

const INITIAL_STATE_NAME: &str = "start";
const MENU_STATE_NAME: &str = "menu";
const REGISTE_STATE_NAME: &str = "register";
const MENU_MESSAGE: &str = "1: Novo registro\n2: Lista de registros";
const REGISTER_NAME_QUESTION: &str = "Qual o nome do registro?";
const REGISTER_LIST: &str = "João Silva\nLucas Neto\nAvestruz de Oliveira";

pub fn init_chatbot_state_machine() -> Result<StateMachine, StateMachineErrors>  {
    let mut state_machine = StateMachine::new("");

    let mut initial_state = State::new(INITIAL_STATE_NAME);
    initial_state.add_transition_with_output(MENU_STATE_NAME, Box::new(DefaultTransitionRule::new()), Box::new(FixedTransitionOutput::new(MENU_MESSAGE)));

    let mut menu_state = State::new(MENU_STATE_NAME);
    menu_state.add_transition_with_output(REGISTE_STATE_NAME, Box::new(EqTransitionRule::new("1")), Box::new(FixedTransitionOutput::new(REGISTER_NAME_QUESTION)));
    menu_state.add_transition_with_output(MENU_STATE_NAME, Box::new(EqTransitionRule::new("2")), Box::new(FixedTransitionOutput::new(REGISTER_LIST)));
    menu_state.add_transition_with_output(MENU_STATE_NAME, Box::new(DefaultTransitionRule::new()), Box::new(FixedTransitionOutput::new(MENU_MESSAGE)));

    let mut register_state = State::new(REGISTE_STATE_NAME);
    register_state.add_transition_with_output(MENU_STATE_NAME, Box::new(DefaultTransitionRule::new()), Box::new(FixedTransitionOutput::new(MENU_MESSAGE)));


    state_machine.add_state(initial_state);
    state_machine.set_initial_state_name(INITIAL_STATE_NAME)?;
    state_machine.add_state(menu_state);
    state_machine.add_state(register_state);

    Ok(state_machine)
}

#[cfg(test)]
mod chatbot_tests {
    use super::*;

    #[test]
    fn chatbot_should_have_initial_message() -> Result<(), StateMachineErrors> {
        let mut chatbot = init_chatbot_state_machine()?;
        
        let response = chatbot.transition_state("1")?;

        assert_eq!(MENU_MESSAGE, response.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_ask_register_name() -> Result<(), StateMachineErrors> {
        let mut chatbot = init_chatbot_state_machine()?;
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("1")?;

        assert_eq!(REGISTER_NAME_QUESTION, response.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_back_to_menu_after_name_registered() -> Result<(), StateMachineErrors> {
        let mut chatbot = init_chatbot_state_machine()?;
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        let response = chatbot.transition_state("José Ricardo")?;

        assert_eq!(MENU_MESSAGE, response.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_register_list_after_back_to_menu() -> Result<(), StateMachineErrors> {
        let mut chatbot = init_chatbot_state_machine()?;
        
        chatbot.transition_state("olá")?;
        chatbot.transition_state("1")?;
        chatbot.transition_state("José Ricardo")?;
        let response = chatbot.transition_state("2")?;

        assert_eq!(REGISTER_LIST, response.unwrap());
        Ok(())
    }

    #[test]
    fn chatbot_should_show_menu_on_invalid_command() -> Result<(), StateMachineErrors> {
        let mut chatbot = init_chatbot_state_machine()?;
        
        chatbot.transition_state("olá")?;
        let response = chatbot.transition_state("olá")?;         

        assert_eq!(MENU_MESSAGE, response.unwrap());
        Ok(())
    }

}
