#[cfg(test)]
mod whatsapp_messages;
mod messages;
mod states;
mod state_machine;
mod chatbot;

use std::collections::HashMap;

struct Funcionario;
struct Aluno;
trait Pessoa {
    fn foo(&self);
}

impl Pessoa for Funcionario {
    fn foo(&self) {
        println!("foo");
    }
}

impl Pessoa for Aluno {
    fn foo(&self) {
        println!("bar");
    }
}

fn main() {
    let aluno = Aluno{};
    let funcionario = Funcionario{};
    let aluno2 = create_aluno();
    call_foo(&aluno);
    call_foo(&funcionario);
    call_foo(create_pessoa(1).as_ref());
    call_foo(&aluno2);

    let result = count_chars("abccde");
    for e in result {
        println!("{}: {}", e.0, e.1);
    }
}

fn create_aluno() -> impl Pessoa {
    Aluno{}
}

fn create_pessoa(val: i32) -> Box<dyn Pessoa> {
    if val == 1 {
        Box::new(Aluno{})
    } else {
        Box::new(Funcionario{})
    }
}

fn call_foo(pessoa: &dyn Pessoa) {
    pessoa.foo();
}

fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();
    for ch in s.chars() {
        let c = result.get(&ch).unwrap_or(&0) + 1;
        result.insert(ch, c);
    }
    result
}
