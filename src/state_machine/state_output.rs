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
    fn generate_output(&self, _data: &str) -> Option<String> {
        Some(String::from(&self.output))
    }
}
