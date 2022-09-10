use std::marker::PhantomData;

use super::{TransitionOutput, TransitionRule};

pub struct EqTransitionRule<E> {
    value: String,
    env_phantom: PhantomData<E>,
}
impl <E> EqTransitionRule<E> {
    pub fn new(value: &str) -> Self {
        Self {
            value: String::from(value),
            env_phantom: PhantomData,
        }
    }
}
impl <E> TransitionRule<E> for EqTransitionRule<E> {
    fn test(&self, _data: &str, action: &str, _env: &mut E) -> bool {
        action == &self.value
    }
}

pub struct DefaultTransitionRule<E> {
    env_phantom: PhantomData<E>,
}
impl <E> DefaultTransitionRule<E> {
    pub fn new() -> Self {
        Self {
            env_phantom: PhantomData,
        }
    }
}
impl <E> TransitionRule<E> for DefaultTransitionRule<E> {
    fn test(&self, _data: &str, _action: &str, _env: &mut E) -> bool {
        true
    }
}

pub struct FnTransitionRule<F, E>
where F: Fn(&str, &str, &mut E) -> bool {
    rule: F,
    env_phantom: PhantomData<E>,
}
impl <F, E> FnTransitionRule<F, E>
where F: Fn(&str, &str, &mut E) -> bool {
    pub fn new(rule: F) -> Self {
        Self {
            rule,
            env_phantom: PhantomData,
        }
    }
}
impl <F, E> TransitionRule<E> for FnTransitionRule<F, E>
where F: Fn(&str, &str, &mut E) -> bool {
    fn test(&self, data: &str, action: &str, env: &mut E) -> bool {
        (&self.rule)(data, action, env)
    }
}

pub struct EmptyTransitionOutput<E> {
    env_phantom: PhantomData<E>,
}
impl <E> EmptyTransitionOutput<E> {
    pub fn new() -> Self {
        Self {
            env_phantom: PhantomData,
        }
    }
}
impl <E> TransitionOutput<E> for EmptyTransitionOutput<E> {
    fn generate_output(&self, _data: &str, _action: &str, _env: &mut E) -> Option<String> {
        None
    }
}

pub struct FixedTransitionOutput<E> {
    output: String,
    env_phantom: PhantomData<E>,
}
impl <E> FixedTransitionOutput<E> {
    pub fn new(output: &str) -> Self {
        Self {
            output: String::from(output),
            env_phantom: PhantomData,
        }
    }
}
impl <E> TransitionOutput<E> for FixedTransitionOutput<E> {
    fn generate_output(&self, _data: &str, _action: &str, _env: &mut E) -> Option<String> {
        Some(String::from(&self.output))
    }
}

pub struct FnTransitionOutput<F, E>
where F: Fn(&str, &str, &mut E) -> Option<String> {
    rule: F,
    env_phantom: PhantomData<E>,
}
impl <F, E> FnTransitionOutput<F, E>
where F: Fn(&str, &str, &mut E) -> Option<String> {
    pub fn new(rule: F) -> Self {
        Self {
            rule,
            env_phantom: PhantomData,
        }
    }
}
impl <F, E> TransitionOutput<E> for FnTransitionOutput<F, E>
where F: Fn(&str, &str, &mut E) -> Option<String> {
    fn generate_output(&self, data: &str, action: &str, env: &mut E) -> Option<String> {
        (&self.rule)(data, action, env)
    }
}
