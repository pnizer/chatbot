use std::{collections::HashMap};

use self::transitions::EmptyTransitionOutput;

pub mod transitions;
pub mod state_output;
pub mod form_states;
mod state_machine_tests;

pub trait TransitionRule {
    fn test(&self, data: &mut HashMap<String, String>, action: &str) -> bool;
}

pub trait TransitionOutput {
    fn generate_output(&self, data: &mut HashMap<String, String>, action: &str) -> Option<String>;
}

pub trait StateOutput {
    fn generate_output(&self, data: &mut HashMap<String, String>) -> Option<String>;
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
    output: Option<Box<dyn StateOutput>>,    
}
impl State {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            transitions: Vec::new(),
            output: None,
         }
    }
    
    pub fn add_transition<TR> (&mut self, target: &str, rule: TR)
    where TR: TransitionRule + 'static {
        self.add_transition_with_output(target, rule, EmptyTransitionOutput::new());
    }

    pub fn add_transition_with_output<TR, TO>(&mut self, target: &str, rule: TR, output: TO)
    where TR: TransitionRule + 'static, TO: TransitionOutput + 'static {
        self.transitions.push((String::from(target), Box::new(rule), Box::new(output)));
    }

    pub fn set_output<O>(&mut self, output: O)
    where O: StateOutput + 'static {
        self.output = Some(Box::new(output));
    }

    pub fn generate_output(&self, data: &mut HashMap<String, String>) -> Option<String> {
        match &self.output {
            None => None,
            Some(state_output) => state_output.generate_output(data)
        }
    }

    pub fn transition(&self, data: &mut HashMap<String, String>, action: &str) -> Option<(String, Option<String>)> {        
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
    state_data: HashMap<String, String>,
}
impl StateMachine
{
    pub fn new(state_data: HashMap<String, String>) -> Self {
        Self { 
            states: HashMap::new(),
            initial_state_name: None,
            current_state: None,
            state_data: state_data,
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

    pub fn set_initial_state_name(&mut self, state_name: &str) -> Result<(), StateMachineErrors> {
        self.states.get(state_name).ok_or(StateMachineErrors::StateNotFound)?;
        self.initial_state_name = Some(String::from(state_name));
        self.current_state = Some(String::from(state_name));
        Ok(())
    }

    fn get_initial_state(&self) -> Option<&State> {
        match &self.initial_state_name {
            Some(name) => self.states.get(name),
            None => None
        }
    }

    pub fn set_current_state(&mut self, state_name: &str) -> Result<(), StateMachineErrors> {
        self.states.get(state_name).ok_or(StateMachineErrors::StateNotFound)?;
        self.current_state = Some(state_name.to_string());
        Ok(())
    }

    pub fn get_current_state(&self) -> Option<String> {
        self.current_state.clone()
    }

    pub fn transition_state(&mut self, action: &str) -> Result<(Option<String>, Option<String>), StateMachineErrors> {    
        let current_state_name = match &self.current_state {
            Some(s) => self.states.get(s),
            None => return Err(StateMachineErrors::InitialStateNotSet),
        };

        let current_state = match current_state_name {
            Some(s) => s,
            None => return Err(StateMachineErrors::InitialStateNotSet),
        };
        
        let new_state_name = current_state.transition(&mut self.state_data, action);
        
        if let Some((n, transition_output)) = new_state_name {
            if !self.states.contains_key(&n) {
                return Err(StateMachineErrors::StateNotFound)
            }
            let state_output = self.states.get(&n).unwrap().generate_output(&mut self.state_data);
            self.current_state = Some(n);            
            return Ok((transition_output, state_output));
        };
        
        Err(StateMachineErrors::WrongTransition)
    }

    pub fn get_state_data(&self) -> &HashMap<String, String> {
        return &self.state_data;
    }
}
