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

    pub fn add_transition(&mut self, target: &str, rule: Box<dyn TransitionRule>) {
        self.transitions.push((String::from(target), rule, Box::new(EmptyTransitionOutput::new())));
    }

    pub fn add_transition_with_output(&mut self, target: &str, rule: Box<dyn TransitionRule>, output: Box<dyn TransitionOutput>) {
        self.transitions.push((String::from(target), rule, output));
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

pub trait TransitionRule {
    fn test(&self, data: &str, action: &str) -> bool;
}

pub trait TransitionOutput {
    fn generate_output(&self, data: &str, action: &str) -> Option<String>;
}

pub struct EqTransitionRule {
    value: String,
}
impl EqTransitionRule {
    pub fn new(value: &str) -> Self {
        Self {
            value: String::from(value),
        }
    }
}
impl TransitionRule for EqTransitionRule {
    fn test(&self, _data: &str, action: &str) -> bool {
        action == &self.value
    }
}

pub struct DefaultTransitionRule {}
impl DefaultTransitionRule {
    pub fn new() -> Self {
        Self {}
    }
}
impl TransitionRule for DefaultTransitionRule {
    fn test(&self, _data: &str, _action: &str) -> bool {
        true
    }
}

struct FnTransitionRule<F>
where F: Fn(&str, &str) -> bool {
    rule: F,
}
impl <F> FnTransitionRule<F>
where F: Fn(&str, &str) -> bool {
    fn new(rule: F) -> Self {
        Self {
            rule,
        }
    }
    
    fn run_test(&self, data: &str, action: &str) -> bool {
        (&self.rule)(data, action)
    }
}
impl <F> TransitionRule for FnTransitionRule<F>
where F: Fn(&str, &str) -> bool {
    fn test(&self, data: &str, action: &str) -> bool {
        self.run_test(data, action)
    }
}

pub struct EmptyTransitionOutput {}
impl EmptyTransitionOutput {
    pub fn new() -> Self {
        Self {}
    }
}
impl TransitionOutput for EmptyTransitionOutput {
    fn generate_output(&self, _data: &str, _action: &str) -> Option<String> {
        None
    }
}

pub struct FixedTransitionOutput {
    output: String,
}
impl FixedTransitionOutput {
    pub fn new(output: &str) -> Self {
        Self {
            output: String::from(output),
        }
    }
}
impl TransitionOutput for FixedTransitionOutput {
    fn generate_output(&self, _data: &str, _action: &str) -> Option<String> {
        Some(String::from(&self.output))
    }
}

#[cfg(test)]
mod state_machine_test {
    use super::*;

    #[test]
    fn state_should_have_name() {
        let state_name = "one";

        let state = State::new(state_name);

        assert_eq!(state_name, &state.name);
    }

    fn state_should_transition_to_right_state() {
        let mut state = State::new("base");
        let transition_rule_1 = FnTransitionRule::new(|_data,action|action == "1");
        let transition_rule_2 = FnTransitionRule::new(|_data,action|action == "2");
        state.add_transition("one", Box::new(transition_rule_1));
        state.add_transition("two", Box::new(transition_rule_2));

        let new_state_1 = state.transition("data", "1");
        let new_state_2 = state.transition("data", "2");
        let new_state_3 = state.transition("data", "3");
        
        assert_eq!("one", new_state_1.as_ref().unwrap().0);
        assert_eq!("two", new_state_2.as_ref().unwrap().0);
        assert!(new_state_3.is_none());
    }

}
