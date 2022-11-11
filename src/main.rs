mod actor;
use actor::*;
use im::vector;
use rayon::prelude::*;
use std::collections::*;

fn main() {
    //let h: Human = Human::new(String::from("Moses Mwaura"), String::from("Frashiah Njenga"), String::from("David Mwaura")).unwrap();
    let mut forest: Vec<Box<Human>> = Vec::new();
    /* 
    let mut moments = [
        Moment { id: "Life Insurance", agents: &Some(Vec::new())},
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
    */
    
    for _i in 0..USERS {
        match Human::new(generate_random_string().unwrap(), generate_random_string().unwrap(), generate_random_string().unwrap()) {
            Some(mut h) => {
                forest.push(Box::new(*h))
            },
            None => eprint!("Error creating Human")
        }
    }

    for u in forest.iter() {
        println!("{:?}", *u); 
    }
}

/*
nanny
pre-school
elementary
middle school
high-school
driving school
road-test
car
college
job
*/