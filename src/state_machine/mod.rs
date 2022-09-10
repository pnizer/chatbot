use std::{collections::HashMap, marker::PhantomData};

use self::transitions::EmptyTransitionOutput;

pub mod transitions;
pub mod state_output;
mod state_machine_tests;

pub trait TransitionRule<E> {
    fn test(&self, data: &str, action: &str, enviroment: &mut E) -> bool;
}

pub trait TransitionOutput<E> {
    fn generate_output(&self, data: &str, action: &str, enviroment: &mut E) -> Option<String>;
}

pub trait StateOutput<E> {
    fn generate_output(&self, data: &str, enviroment: &mut E) -> Option<String>;
}

#[derive(Debug)]
pub enum StateMachineErrors {
    StateNotFound,
    InitialStateNotSet,
    WrongTransition,
}

pub struct State<E> {
    pub name: String,    
    transitions: Vec<(String, Box<dyn TransitionRule<E>>, Box<dyn TransitionOutput<E>>)>,
    output: Option<Box<dyn StateOutput<E>>>,
    environment_phantom: PhantomData<E>,
}
impl <E: 'static> State<E> {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            transitions: Vec::new(),
            output: None,
            environment_phantom: PhantomData,
         }
    }
    
    pub fn add_transition<TR> (&mut self, target: &str, rule: TR)
    where TR: TransitionRule<E> + 'static {
        self.add_transition_with_output(target, rule, EmptyTransitionOutput::new());
    }

    pub fn add_transition_with_output<TR, TO>(&mut self, target: &str, rule: TR, output: TO)
    where TR: TransitionRule<E> + 'static, TO: TransitionOutput<E> + 'static {
        self.transitions.push((String::from(target), Box::new(rule), Box::new(output)));
    }

    pub fn set_output<O>(&mut self, output: O)
    where O: StateOutput<E> + 'static {
        self.output = Some(Box::new(output));
    }

    pub fn generate_output(&self, data: &str, environment: &mut E) -> Option<String> {
        match &self.output {
            None => None,
            Some(state_output) => state_output.generate_output(data, environment)
        }
    }

    pub fn transition(&self, data: &str, action: &str, enviroment: &mut E) -> Option<(String, Option<String>)> {        
        for (target, rule, output) in &self.transitions {
            if rule.test(data, action, enviroment) {
                return Some((String::from(target), output.generate_output(data, action, enviroment)));
            }
        }
        None
    }
}

pub struct StateMachine<E>
{
    states: HashMap<String, State<E>>,    
    initial_state_name: Option<String>,
    current_state: Option<String>,
    state_data: String,
    environment: PhantomData<E>,
}
impl<E: 'static> StateMachine<E>
{
    pub fn new(state_data: &str) -> Self {
        Self { 
            states: HashMap::new(),
            initial_state_name: None,
            current_state: None,
            state_data: String::from(state_data),
            environment: PhantomData,
        }
    }

    pub fn add_state(&mut self, state: State<E>) {
        self.states.insert(state.name.clone(), state);
    }

    fn get_states(&self) -> &HashMap<String, State<E>> {
        &self.states
    }

    fn get_state(&self, name: &str) -> Option<&State<E>> {
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

    fn get_initial_state(&self) -> Option<&State<E>> {
        match &self.initial_state_name {
            Some(name) => self.states.get(name),
            None => None
        }
    }

    pub fn transition_state(&mut self, action: &str, environment: &mut E) -> Result<(Option<String>, Option<String>), StateMachineErrors> {    
        let current_state_name = match &self.current_state {
            Some(s) => self.states.get(s),
            None => return Err(StateMachineErrors::InitialStateNotSet),
        };

        let current_state = match current_state_name {
            Some(s) => s,
            None => return Err(StateMachineErrors::InitialStateNotSet),
        };
        
        let new_state_name = current_state.transition(&self.state_data, action, environment);
        
        if let Some((n, transition_output)) = new_state_name {
            if !self.states.contains_key(&n) {
                return Err(StateMachineErrors::StateNotFound)
            }
            let state_output = self.states.get(&n).unwrap().generate_output(&self.state_data, environment);
            self.current_state = Some(n);            
            return Ok((transition_output, state_output));
        };
        
        Err(StateMachineErrors::WrongTransition)
    }
}
