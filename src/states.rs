use std::collections::HashMap;

use crate::state_machine::State;

#[derive(Debug)]
pub enum StateMachineErrors {
    StateNotFound,
    InitialStateNotSet,
    WrongTransition,
}

pub struct StateMachine
{
    states: HashMap<String, State>,    
    initial_state_name: Option<String>,
    current_state: Option<String>,
    state_data: String,    
}
impl StateMachine
{
    pub fn new(state_data: &str) -> Self {
        Self { 
            states: HashMap::new(),
            initial_state_name: None,
            current_state: None,
            state_data: String::from(state_data),
        }
    }

    pub fn add_state(&mut self, state: State) {
        self.states.insert(state.name.clone(), state);
    }

    fn get_states(&self) -> &HashMap<String, State> {
        &self.states
    }

    fn get_state(&self, name: &str) -> Option<&State> {
        self.states.get(name)
    }

    pub fn set_initial_state_name(&mut self, name: &str) -> Result<(), StateMachineErrors> {
        let state = self.states.get(name);        
        if state.is_some() {
            self.initial_state_name = Some(String::from(name));
            self.current_state = Some(String::from(name));
            Ok(())
        } else {
            Err(StateMachineErrors::StateNotFound)
        }
    }

    fn get_initial_state(&self) -> Option<&State> {
        match &self.initial_state_name {
            Some(name) => self.states.get(name),
            None => None
        }
    }

    pub fn transition_state(&mut self, action: &str) -> Result<Option<String>, StateMachineErrors>{        
        let current_state_name = match &self.current_state {
            Some(s) => self.states.get(s),
            None => return Err(StateMachineErrors::InitialStateNotSet),
        };

        let current_state = match current_state_name {
            Some(s) => s,
            None => return Err(StateMachineErrors::InitialStateNotSet),
        };
        
        let new_state_name = current_state.transition(&self.state_data, action);
        
        if let Some((n, output)) = new_state_name {
            if !self.states.contains_key(&n) {
                return Err(StateMachineErrors::StateNotFound)
            }
            self.current_state = Some(n);
            return Ok(output);
        };
        
        Err(StateMachineErrors::WrongTransition)
    }
}

#[cfg(test)]
mod states_test {
    use crate::state_machine::EqTransitionRule;

    use super::*;

    #[test]
    fn state_should_have_names() {
        let name = "state 1";

        let state = State::new(name);

        assert_eq!(name, state.name);
    }

    #[test]
    fn state_machine_should_receive_states() {
        let mut state_machine = StateMachine::new("");
        let name_1 = "state 1";
        let name_2 = "state 2";

        state_machine.add_state(State::new(name_1));
        state_machine.add_state(State::new(name_2));

        assert_eq!(2, state_machine.get_states().len());
    }

    #[test]
    fn state_machine_should_return_state_by_name() {
        let mut state_machine = StateMachine::new("");
        let name_1 = "state 1";
        let name_2 = "state 2";
        state_machine.add_state(State::new(name_1));
        state_machine.add_state(State::new(name_2));

        let state_1 = state_machine.get_state(name_1);
        let state_2 = state_machine.get_state(name_2);

        assert_eq!(name_1, &state_1.unwrap().name);
        assert_eq!(name_2, &state_2.unwrap().name);
    }

    #[test]
    fn state_machine_should_have_initial_state() -> Result<(), StateMachineErrors> {
        let mut state_machine = StateMachine::new("");
        let name_1 = "state 1";
        let name_2 = "state 2";
        state_machine.add_state(State::new(name_1));
        state_machine.add_state(State::new(name_2));

        state_machine.set_initial_state_name(name_2)?;

        assert_eq!(name_2, state_machine.get_initial_state().unwrap().name);
        Ok(())
    }

    #[test]
    fn state_machine_should_transition_state() -> Result<(), StateMachineErrors> {
        let mut state_machine = StateMachine::new("");
        let name_1 = "state 1";
        let name_2 = "state 2";
        let mut state_1 = State::new(name_1);
        state_1.add_transition(name_2, Box::new(EqTransitionRule::new("hi")));
        let mut state_2 = State::new(name_2);
        state_1.add_transition(name_1, Box::new(EqTransitionRule::new("hi")));
        state_machine.add_state(state_1);
        state_machine.add_state(state_2);
        state_machine.set_initial_state_name(name_1)?;
        
        state_machine.transition_state("hi")?;

        assert_eq!(name_2, state_machine.current_state.as_ref().unwrap());
        Ok(())
    }

    #[test]
    fn state_machine_should_transition_back_state() -> Result<(), StateMachineErrors> {
        let mut state_machine = StateMachine::new("");
        let name_1 = "state 1";
        let name_2 = "state 2";
        let mut state_1 = State::new(name_1);
        state_1.add_transition(name_2, Box::new(EqTransitionRule::new("hi")));
        let mut state_2 = State::new(name_2);
        state_2.add_transition(name_1, Box::new(EqTransitionRule::new("hi")));
        state_machine.add_state(state_1);
        state_machine.add_state(state_2);
        state_machine.set_initial_state_name(name_1)?;
        
        state_machine.transition_state("hi")?;
        state_machine.transition_state("hi")?;

        assert_eq!(name_1, state_machine.current_state.as_ref().unwrap());
        Ok(())
    }
}
