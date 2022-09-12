use std::{rc::Rc, cell::RefCell};

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

trait Command {
    fn execute(&self);
}

struct PrintCommand {
    my_service: Rc<RefCell<MyService>>,
}
impl PrintCommand {
    fn new(my_service: Rc<RefCell<MyService>>) -> Self {
        Self {
            my_service,
        }
    }
}
impl Command for PrintCommand {
    fn execute(&self) {
        let mut my_service = self.my_service.borrow_mut();
        my_service.general_function();
    }
}

struct FooCommand {
    my_service: Rc<RefCell<MyService>>,
}
impl FooCommand {
    fn new(my_service: Rc<RefCell<MyService>>) -> Self {
        Self {
            my_service,
        }
    }
}
impl Command for FooCommand {
    fn execute(&self) {
        let mut my_service = self.my_service.borrow_mut();
        my_service.foo_function();
    }
}

struct CommandsSequence {
    commands: Vec<Box<dyn Command>>,    
}
impl CommandsSequence {
    fn new() -> Self {
        Self {
            commands: Vec::new(),    
        }
    }

    fn add_command<C: Command + 'static>(&mut self, command: C) {
        self.commands.push(Box::new(command));
    }

    fn execute_all(&self) {
        for cmd in &self.commands {
            cmd.execute();
        }
    }
}

#[cfg(test)]
mod test_test {
    use super::*;

    #[test]
    fn my_test() {
        let service = Rc::new(RefCell::new(MyService::new("foo service")));        

        let mut seq = CommandsSequence::new();  
        seq.add_command(PrintCommand::new(service.clone()));
        seq.add_command(PrintCommand::new(service.clone()));
        seq.add_command(FooCommand::new(service.clone()));
        seq.add_command(PrintCommand::new(service.clone()));

        seq.execute_all();

        service.borrow().print_output();
    }
}
