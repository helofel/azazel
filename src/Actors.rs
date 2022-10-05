#[derive(Debug)]
struct Human<'a> {
    signature: &'a Option<Box<usize>>,
    line: &'a Option<Box<std::collections::LinkedList<Node<'a>>>> //Queue
}

struct Entity<'b> {
    signature: &'b Option<Box<usize>>,
    func: &'b Option<Box<std::vec::Vec<Node<'b>>>>,
    schedule: &'b Option<Box<std::vec::Vec<Node<'b>>>> //Queue
}

struct Agent<'c> {
    signature: &'c Option<Box<usize>>,
    tail: &'c Option<Box<Human<'c>>>,
    head: &'c Option<Box<Entity<'c>>>,
    tasks: &'c Option<Box<std::vec::Vec<Node<'c>>>> //Queue
}

#[derive(Debug)]
struct Node<'d> {
    val: &'d usize 
}
enum Errors {
    HumanInstantiationError,
    EntityInstantiationError,
    AgentInstantiationError
}

impl<'a> Human<'a> {
    fn new (&self, data: usize) -> &'a Option<Human> {
        
        &Some(Human {
            signature: &Some(Box::new(data.to_owned())),
            line: &Some(Box::new(std::collections::LinkedList::new())) 
        })
    }
}

impl<'b> Entity<'b> {
    fn new (&self) -> &'b Result<Entity, crate::Actors::Errors> {
        Ok(Entity {
            signature: &Some(Box::new("".to_owned())),
            func: &Some(Box::new(std::vec::Vec::new())),
            schedule: &Some(Box::new(std::vec::Vec::new()))
        })
    }
}

impl<'c> Agent<'c> {
    
}