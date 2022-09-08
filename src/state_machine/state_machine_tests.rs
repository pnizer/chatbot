#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::transitions::*;
    use super::super::state_output::*;

    #[test]
    fn state_should_have_name() {
        let state_name = "one";

        let state = State::new(state_name);

        assert_eq!(state_name, &state.name);
    }

    #[test]
    fn state_should_transition_to_right_state() {
        let mut state = State::new("base");
        let transition_rule_1 = FnTransitionRule::new(|_data,action|action == "1");
        let transition_rule_2 = FnTransitionRule::new(|_data,action|action == "2");
        state.add_transition("one", transition_rule_1);
        state.add_transition("two", transition_rule_2);

        let new_state_1 = state.transition("data", "1");
        let new_state_2 = state.transition("data", "2");
        let new_state_3 = state.transition("data", "3");
        
        assert_eq!("one", new_state_1.as_ref().unwrap().0);
        assert_eq!("two", new_state_2.as_ref().unwrap().0);
        assert!(new_state_3.is_none());
    }

    #[test]
    fn state_should_have_names() {
        let name = "state 1";

        let state = State::new(name);

        assert_eq!(name, state.name);
    }

    #[test]
    fn state_should_have_optional_output() {
        let name = "state 1";
        let mut state = State::new(name);
        let state_output = FixedStateOutput::new("hello there!");
        state.set_output(state_output);
        
        let output: Option<String> = state.generate_output("data");

        assert_eq!(true, output.is_some());
        assert_eq!("hello there!", output.as_ref().unwrap());
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
        state_1.add_transition(name_2, EqTransitionRule::new("hi"));
        let mut state_2 = State::new(name_2);
        state_2.add_transition(name_1, EqTransitionRule::new("hi"));
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
        state_1.add_transition(name_2, EqTransitionRule::new("hi"));
        let mut state_2 = State::new(name_2);
        state_2.add_transition(name_1, EqTransitionRule::new("hi"));
        state_machine.add_state(state_1);
        state_machine.add_state(state_2);
        state_machine.set_initial_state_name(name_1)?;
        
        state_machine.transition_state("hi")?;
        state_machine.transition_state("hi")?;

        assert_eq!(name_1, state_machine.current_state.as_ref().unwrap());
        Ok(())
    }

    #[test]
    fn state_machine_should_transition_with_state_output() -> Result<(), StateMachineErrors> {
        let mut state_machine = StateMachine::new("");
        let name_1 = "state 1";
        let name_2 = "state 2";
        let mut state_1 = State::new(name_1);
        state_1.add_transition(name_2, EqTransitionRule::new("hi"));
        let mut state_2 = State::new(name_2);
        state_2.set_output(FixedStateOutput::new("fixed value"));
        state_2.add_transition(name_1, EqTransitionRule::new("hi"));        
        state_machine.add_state(state_1);
        state_machine.add_state(state_2);
        state_machine.set_initial_state_name(name_1)?;
        
        let (_transition_output_01, state_output_01) = state_machine.transition_state("hi")?;
        let (_transition_output_02, state_output_02) = state_machine.transition_state("hi")?;

        assert_eq!(true, state_output_01.is_some());
        assert_eq!("fixed value", state_output_01.unwrap());
        assert_eq!(true, state_output_02.is_none());
        Ok(())
    }
}
