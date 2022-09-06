use self::transitions::EmptyTransitionOutput;
use std::{collections::HashMap};

pub mod transitions;
mod state_machine_tests;

pub trait TransitionRule {
    fn test(&self, data: &str, action: &str) -> bool;
}

pub trait TransitionOutput {
    fn generate_output(&self, data: &str, action: &str) -> Option<String>;
}

#[derive(Debug)]
pub enum StateMachineErrors {
    StateNotFound,
    InitialStateNotSet,
    WrongTransition,
}

pub struct State {
    pub name: String,    
    transitions: Vec<(String, Box<dyn TransitionRule>, Box<dyn TransitionOutput>)>,
}
impl State {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            transitions: Vec::new(),
         }
    }
    
    pub fn add_transition<TR> (&mut self, target: &str, rule: TR)
    where TR: TransitionRule + 'static {
        self.transitions.push((String::from(target), Box::new(rule), Box::new(EmptyTransitionOutput::new())));
    }

    pub fn add_transition_with_output<TR, TO>(&mut self, target: &str, rule: TR, output: TO)
    where TR: TransitionRule + 'static, TO: TransitionOutput + 'static {
        self.transitions.push((String::from(target), Box::new(rule), Box::new(output)));
    }

    pub fn transition(&self, data: &str, action: &str) -> Option<(String, Option<String>)> {
        for (target, rule, output) in &self.transitions {
            if rule.test(data, action) {
                return Some((String::from(target), output.generate_output(data, action)));
            }
        }
        None
    }
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
