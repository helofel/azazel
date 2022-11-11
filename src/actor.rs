use std::collections::LinkedList;

use rand::{distributions::Alphanumeric, Rng};

pub static USERS: i32 = 1_000_000;

#[derive(Debug)]
pub struct Human<'a> {
    name: Option<Box<String>>,
    parents: Option<Box<String>>,
    pub line: &'a mut Option<Box<LinkedList<Moment<'a>>>> 
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

#[derive(Debug, Clone, Copy)]
pub struct Moment<'d> {
    id: &'d str,
    agents: &'d Option<Vec<Agent<'d>>>
}


#[derive(Debug)]
pub struct Stack<'e, T: Copy> {
  pub stack: &'e mut Vec<T>,
}

enum Errors {
    HumanInstantiationError,
    EntityInstantiationError,
    AgentInstantiationError
}

//impl<'e, T> Copy for Stack<'e, T> where T: Copy {}

impl<'e, T> Stack<'e, T> where T: Copy {
  pub fn new() -> Self {
    Stack { stack: &mut Vec::new() }
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

    pub fn new (name: String, mother: String, father: String) -> Option<Box<Self>> { 
        let m = [mother, father].concat();
        let s = Some(Vec::new());
        let mut moments = [
          Moment { id: "Life Insurance", agents: &s},
          Moment { id: "Nanny", agents: &Some(Vec::new())},
          Moment { id: "Pre-school", agents: &Some(Vec::new())},
          Moment { id: "Elementary", agents: &Some(Vec::new())},
          Moment { id: "Middle school", agents: &Some(Vec::new())},
          Moment { id: "High school", agents: &Some(Vec::new())},
          Moment { id: "Driving school", agents: &Some(Vec::new())},
          Moment { id: "Road test", agents: &Some(Vec::new())},
          Moment { id: "Car", agents: &Some(Vec::new())},
          Moment { id: "College", agents: &Some(Vec::new())},
          Moment { id: "Job", agents: &Some(Vec::new())}
        ];

        Some(Box::new(Self {
            name: Some(Box::new(name.clone())),
            parents: Some(Box::new(m.clone())),
            line: &mut Some(Box::new(LinkedList::from(moments)))
        }))
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