use std::sync::Arc;
use std::sync::RwLock;
use crate::cabin::{Etage, Fahrkabine};
use crate::controller::Controller;
use crate::passenger::{Passagier, PassengerState};

// use elevator::*;
mod cabin;
mod passenger;
mod controller;

pub  fn run_simulation(num_elevators: usize, num_floors: i32, num_passengers: usize) {
    // create elevators
    let mut fahrkabinen = Vec::new();
    for i in 0..num_elevators {
        fahrkabinen.push(Fahrkabine::new(i as i32, 0));
    }

    // create floors
    let mut etagen = Vec::new();
    for i in 0..num_floors {
        etagen.push(Etage { id: i });
    }

    // create controller
    let controller = Arc::new(RwLock::new(Controller::new(
        fahrkabinen.clone(),
        etagen,
    )));

    println!("Main loop");
    for i in 0..num_passengers {
        let random_floor = rand::random::<i32>().abs() % num_floors;
        let mut random_floor_2 = rand::random::<i32>().abs() % num_floors;
        while random_floor_2 == random_floor {
            random_floor_2 = rand::random::<i32>().abs() % num_floors;
        }

        let p = Passagier::new(
            i as i32,
            random_floor,
            random_floor_2,
            fahrkabinen.clone(),
            controller.clone(),
        );
        controller.write().unwrap().all_passengers.push(p);
    }
    println!("Created all Passengers!");
    while controller.read().unwrap().all_passengers.iter().any(|p| {
        let p = p.read().unwrap();
        p.state != PassengerState::Exiting
    }) {}
    println!("All passengers have exited");
    for fahrkabine in fahrkabinen {
        println!("{:?}", fahrkabine);
    }
}
