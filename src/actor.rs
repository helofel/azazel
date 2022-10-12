use rand::{distributions::Alphanumeric, Rng};

pub static USERS: i32 = 1_000_000;

#[derive(Debug)]
pub struct Human<'a> {
    name: Option<Box<String>>,
    parents: Option<Box<String>>,
    line: Option<Box<Stack<Moment<'a>>>> //Queue
}

#[derive(Debug)]
struct Entity<'b> {
    //signature: &'b Option<Box<usize>>,
    func: &'b Option<Box<std::vec::Vec<Moment<'b>>>>,
    schedule: &'b Option<Box<std::vec::Vec<Moment<'b>>>> //Queue
}

#[derive(Debug)]
struct Agent<'c> {
    signature: &'c Option<Box<usize>>,
    tail: &'c Option<Box<Human<'c>>>,
    head: &'c Option<Box<Entity<'c>>>,
    tasks: &'c Option<Box<std::vec::Vec<Moment<'c>>>> //Queue
}

#[derive(Debug)]
pub struct Moment<'d> {
    id: &'d str,
    agents: &'d Option<Vec<Agent<'d>>>
}


#[derive(Debug)]
pub struct Stack<T> {
  stack: Vec<T>,
}
enum Errors {
    HumanInstantiationError,
    EntityInstantiationError,
    AgentInstantiationError
}

impl<T> Stack<T> {
  pub fn new() -> Self {
    Stack { stack: Vec::new() }
  }

  pub fn length(&self) -> usize {
    self.stack.len()
  }

  pub fn pop(&mut self) -> Option<T> {
    self.stack.pop()
  }

  pub fn push(&mut self, item: T) {
    self.stack.push(item)
  }

  pub fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }

  pub fn peek(&self) -> Option<&T> {
    self.stack.last()
  }
}

impl<'a> Human<'a> {
    pub fn new (name: String, mother: String, father: String) -> Option<Human<'a>> { 
        let m = [mother, father].concat();

        Some(Human {
            name: Some(Box::new(name.clone())),
            parents: Some(Box::new(m.clone())),
            line: Some(Box::new(Stack::new())) 
        })
    }
}

pub fn generate_random_string() -> Option<String> {
    Some(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect())
}

/* 
impl<'b> Entity<'b> {
    fn new (&self, data: usize) -> &Option<Entity> {
        &Some(Entity {
            //signature: &Some(Box::new(data)),
            func: &Some(Box::new(std::vec::Vec::new())),
            schedule: &Some(Box::new(std::vec::Vec::new()))
        })
    }
}

impl<'c> Agent<'c> {
    
}*/