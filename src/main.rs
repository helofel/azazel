mod actor;
use actor::*;
use im::vector;
use rayon::prelude::*;

fn main() {
    let h: Human = Human::new(String::from("Moses Mwaura"), String::from("Frashiah Njenga"), String::from("David Mwaura")).unwrap();
    let mut forest: Vec<Box<Human>> = Vec::new();
    let mut moments: Vec<&str> = vec![
        "Life Insurance",
        "Nanny",
        "Pre-school",
        "Elementary",
        "Middle school",
        "High school",
        "Driving school",
        "Road test",
        "Car",
        "College",
        "Job"
    ];

    for _i in 0..USERS {
        match Human::new(generate_random_string().unwrap(), generate_random_string().unwrap(), generate_random_string().unwrap()) {
            Some(h) => {  
                forest.push(Box::new(h))
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