use std::marker::PhantomData;

use super::StateOutput;

pub struct FixedStateOutput<E> {
    output: String,
    env_phantom: PhantomData<E>,
}
impl <E> FixedStateOutput<E> {
    pub fn new(output: &str) -> Self {
        Self {
            output: String::from(output),
            env_phantom: PhantomData,
        }
    }
}
impl <E> StateOutput<E> for FixedStateOutput<E> {
    fn generate_output(&self, _data: &str, _env: &mut E) -> Option<String> {
        Some(String::from(&self.output))
    }
}
