use std::collections::HashMap;

use super::{TransitionOutput, TransitionRule};

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
    fn test(&self, _data: &mut HashMap<String, String>, action: &str) -> bool {
        action == &self.value
    }
}

pub struct DefaultTransitionRule;
impl DefaultTransitionRule {
    pub fn new() -> Self {
        Self {}
    }
}
impl TransitionRule for DefaultTransitionRule {
    fn test(&self, _data: &mut HashMap<String, String>, _action: &str) -> bool {
        true
    }
}

pub struct FnTransitionRule<F>
where F: Fn(&mut HashMap<String, String>, &str) -> bool {
    rule: F,    
}
impl <F> FnTransitionRule<F>
where F: Fn(&mut HashMap<String, String>, &str) -> bool {
    pub fn new(rule: F) -> Self {
        Self {
            rule,
        }
    }
}
impl <F> TransitionRule for FnTransitionRule<F>
where F: Fn(&mut HashMap<String, String>, &str) -> bool {
    fn test(&self, data: &mut HashMap<String, String>, action: &str) -> bool {
        (&self.rule)(data, action)
    }
}

pub struct EmptyTransitionOutput;
impl EmptyTransitionOutput {
    pub fn new() -> Self {
        Self {}
    }
}
impl TransitionOutput for EmptyTransitionOutput {
    fn generate_output(&self, _data: &mut HashMap<String, String>, _action: &str) -> Option<String> {
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
    fn generate_output(&self, _data: &mut HashMap<String, String>, _action: &str) -> Option<String> {
        Some(String::from(&self.output))
    }
}

pub struct FnTransitionOutput<F>
where F: Fn(&mut HashMap<String, String>, &str) -> Option<String> {
    rule: F,
}
impl <F> FnTransitionOutput<F>
where F: Fn(&mut HashMap<String, String>, &str) -> Option<String> {
    pub fn new(rule: F) -> Self {
        Self {
            rule,
        }
    }
}
impl <F> TransitionOutput for FnTransitionOutput<F>
where F: Fn(&mut HashMap<String, String>, &str) -> Option<String> {
    fn generate_output(&self, data: &mut HashMap<String, String>, action: &str) -> Option<String> {
        (&self.rule)(data, action)
    }
}
