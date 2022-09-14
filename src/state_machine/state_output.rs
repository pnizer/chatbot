use std::collections::HashMap;

use super::StateOutput;

pub struct FixedStateOutput {
    output: String,
}
impl FixedStateOutput {
    pub fn new(output: &str) -> Self {
        Self {
            output: String::from(output),
        }
    }
}
impl StateOutput for FixedStateOutput {
    fn generate_output(&self, _data: &mut HashMap<String, String>) -> Option<String> {
        Some(String::from(&self.output))
    }
}

pub struct FnStateOutput<F>
where F: Fn(&mut HashMap<String, String>) -> Option<String> {
    rule: F,
}
impl <F> FnStateOutput<F>
where F: Fn(&mut HashMap<String, String>) -> Option<String> {
    pub fn new(rule: F) -> Self {
        Self {
            rule,
        }
    }
}
impl <F> StateOutput for FnStateOutput<F>
where F: Fn(&mut HashMap<String, String>) -> Option<String> {
    fn generate_output(&self, data: &mut HashMap<String, String>) -> Option<String> {
        (&self.rule)(data)
    }
}
