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

pub struct FnTransitionRule<F>
where F: Fn(&str, &str) -> bool {
    rule: F,
}
impl <F> FnTransitionRule<F>
where F: Fn(&str, &str) -> bool {
    pub fn new(rule: F) -> Self {
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