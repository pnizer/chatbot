use std::marker::PhantomData;

struct MyService {
    name: String,
    output: Vec<String>,
}
impl MyService {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            output: Vec::new(),
        }
    }

    fn general_function(&mut self) {
        self.output.push(format!("general_function for {}", self.name));
    }
    fn foo_function(&mut self) {
        self.output.push(format!("foo_function for {}", self.name));
    }

    fn print_output(&self) {
        for o in &self.output {
            println!("{}", o);
        }
    }
}

trait Command<S> {
    fn execute(&self, env: &mut S);
}

struct PrintCommand {}
impl PrintCommand {
    fn new() -> Self {
        Self {}
    }
}
impl Command<Environment> for PrintCommand {
    fn execute(&self, env: &mut Environment) {
        env.my_service.general_function();
    }
}

struct FooCommand {}
impl FooCommand {
    fn new() -> Self {
        Self {}
    }
}
impl Command<Environment> for FooCommand {
    fn execute(&self, env: &mut Environment) {
        env.my_service.foo_function();
    }
}

struct CommandsSequence<S> {
    commands: Vec<Box<dyn Command<S>>>,
    phantom: PhantomData<S>,
}
impl<S> CommandsSequence<S> {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
            phantom: PhantomData,
        }
    }

    fn add_command<C: Command<S> + 'static>(&mut self, command: C) {
        self.commands.push(Box::new(command));
    }

    fn execute_all(&self, my_service: &mut S) {
        for cmd in &self.commands {
            cmd.execute(my_service);
        }
    }
}

struct Environment {
    my_service: MyService,
}
impl Environment {
    fn new(my_service: MyService) -> Self {
        Self {
            my_service,
        }
    }
}

#[cfg(test)]
mod test_test {
    use super::*;

    #[test]
    fn my_test() {
        let service = MyService::new("foo service");
        let mut env = Environment::new(service);

        let mut seq = CommandsSequence::new();  
        seq.add_command(PrintCommand::new());
        seq.add_command(PrintCommand::new());
        seq.add_command(FooCommand::new());
        seq.add_command(PrintCommand::new());

        seq.execute_all(&mut env);

        env.my_service.print_output();        
    }
}
