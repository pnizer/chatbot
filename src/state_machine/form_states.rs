use crate::state_machine::transitions::FnTransitionRule;

use super::{State, StateMachine, state_output::FixedStateOutput};


pub enum FieldType {
    String,
    Number,
}
pub enum FieldOption {
    Optional,
    Required,
}

struct Field(String, String, FieldType, FieldOption);

pub struct FormStates {
    states_prefix: String,
    fields: Vec<Field>,
}
impl FormStates {
    pub fn new(states_prefix: &str) -> Self {
        Self {
            states_prefix: states_prefix.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field_name: &str, label: &str, field_type: FieldType, field_option: FieldOption) {
        self.fields.push(Field(field_name.to_string(), label.to_string(), field_type, field_option));
    }

    pub fn apply_states(self, success_state_name: &str, state_machine: &mut StateMachine) {        
        self.apply_form_field_state(
            self.fields.last().unwrap(),          // last field
            success_state_name,                   // should transition to success state
            &self.fields[..self.fields.len()-1],  // and do the same, droping last element
            state_machine
        );
    }

    fn apply_form_field_state(&self, field: &Field, next_state: &str, previous_fields: &[Field], state_machine: &mut StateMachine) {
        let Field(field_name, label, field_type, field_option) = field;
        let state_name = self.states_prefix.to_owned() + &field_name;
        let mut state = State::new(&state_name);
        state.set_output(FixedStateOutput::new(&label));
        let field_data_key = state_name.to_string();
        state.add_transition(next_state, FnTransitionRule::new(move |data, action| {
            data.insert(field_data_key.to_string(), action.to_string());
            true
        }));
        state_machine.add_state(state);

        if !previous_fields.is_empty() {
            self.apply_form_field_state(
                previous_fields.last().unwrap(),                // last field on slice
                &state_name,                                    // should transition to current
                &previous_fields[..previous_fields.len()-1],    // and do the same, droping last element
                state_machine
            );
        }        

        println!("add {} -> {} ({})", &state_name, next_state, previous_fields.len());
    }
}

#[cfg(test)]
mod form_states_test {
    use std::collections::HashMap;

    use crate::state_machine::{transitions::{EqTransitionRule, DefaultTransitionRule}, state_output::{FixedStateOutput}};

    use super::*;

    fn build_basic_state_machine(form_state: &str) -> StateMachine {
        let mut state_machine = StateMachine::new(HashMap::new());        
        let mut state = State::new("initial");
        state.add_transition(form_state, EqTransitionRule::new("1"));
        state_machine.add_state(state);
        state_machine.set_initial_state_name("initial").unwrap();
        let mut state = State::new("register-created-state");
        state.set_output(FixedStateOutput::new("finished!"));
        state.add_transition("initial", DefaultTransitionRule::new());
        state_machine.add_state(state);
        state_machine
    }

    #[test]
    fn form_states_should_be_configured_with_fields_and_types() {
        let mut state_machine = build_basic_state_machine("register-name");
        let mut form_states = FormStates::new("register-");
        form_states.add_field("name", "Name? (required)", FieldType::String, FieldOption::Required);
        form_states.add_field("age", "Age?", FieldType::Number, FieldOption::Optional);

        form_states.apply_states("register-created-state", &mut state_machine);
    }

    #[test]
    fn form_states_should_set_state_machine_to_read_fields_with_one_field() {
        let mut state_machine = build_basic_state_machine("register-name");
        let mut form_states = FormStates::new("register-");
        form_states.add_field("name", "Name? (required)", FieldType::String, FieldOption::Required);
        form_states.apply_states("register-created-state", &mut state_machine);

        let (_, state_output_01) = state_machine.transition_state("1").unwrap();
        let (_, state_output_02) = state_machine.transition_state("John John").unwrap();

        assert_eq!("Name? (required)", state_output_01.unwrap());        
        assert_eq!("finished!", state_output_02.unwrap());
    }

    #[test]
    fn form_states_should_set_state_machine_to_read_fields_with_many_fields() {
        let mut state_machine = build_basic_state_machine("register-first-name");
        let mut form_states = FormStates::new("register-");
        form_states.add_field("first-name", "First name? (required)", FieldType::String, FieldOption::Required);
        form_states.add_field("last-name", "Last name? (required)", FieldType::String, FieldOption::Required);
        form_states.add_field("age", "Age?", FieldType::Number, FieldOption::Optional);
        form_states.apply_states("register-created-state", &mut state_machine);

        let (_, state_output_01) = state_machine.transition_state("1").unwrap();
        let (_, state_output_02) = state_machine.transition_state("John").unwrap();
        let (_, state_output_03) = state_machine.transition_state("Smith").unwrap();
        let (_, state_output_04) = state_machine.transition_state("30").unwrap();

        assert_eq!("First name? (required)", state_output_01.unwrap());        
        assert_eq!("Last name? (required)", state_output_02.unwrap());     
        assert_eq!("Age?", state_output_03.unwrap());     
        assert_eq!("finished!", state_output_04.unwrap());
    }

    #[test]
    fn form_states_should_set_state_machine_data_change_when_filling_the_fields() {
        let mut state_machine = build_basic_state_machine("register-first-name");
        let mut form_states = FormStates::new("register-");
        form_states.add_field("first-name", "First name? (required)", FieldType::String, FieldOption::Required);
        form_states.add_field("last-name", "Last name? (required)", FieldType::String, FieldOption::Required);
        form_states.add_field("age", "Age?", FieldType::Number, FieldOption::Optional);
        form_states.apply_states("register-created-state", &mut state_machine);

        state_machine.transition_state("1").unwrap();
        state_machine.transition_state("John").unwrap();
        state_machine.transition_state("Smith").unwrap();
        state_machine.transition_state("30").unwrap();
        let data = state_machine.get_state_data();

        assert_eq!("John", data.get("register-first-name").unwrap());
        assert_eq!("Smith", data.get("register-last-name").unwrap());
        assert_eq!("30", data.get("register-age").unwrap());
    }
}
