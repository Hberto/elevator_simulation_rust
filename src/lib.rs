use std::sync::Arc;
use std::sync::RwLock;
use crate::cabin::{Etage, Fahrkabine};
use crate::controller::Controller;
use crate::passenger::{Passagier, PassengerState};
use log::{info, warn};

// use elevator::*;
mod cabin;
pub mod passenger;
mod controller;

pub fn init_simulation(
    num_elevators: usize,
    num_floors: i32,
    num_passengers: usize,
    max_passengers_per_cabin: usize,
    current_floors: Option<Vec<i32>>,
    destination_floors: Option<Vec<i32>>,
) -> Arc<RwLock<Controller>> {
    // Create elevators
    let mut fahrkabinen = Vec::new();
    for i in 0..num_elevators {
        fahrkabinen.push(Fahrkabine::new(i as i32, 0, max_passengers_per_cabin));
    }

    // Create floors
    let mut etagen = Vec::new();
    for i in 0..num_floors {
        etagen.push(Etage { id: i });
    }

    // Create controller
    let controller = Arc::new(RwLock::new(Controller::new(
        fahrkabinen.clone(),
        etagen,
    )));

    // init current floors if not passed as parameter
    let current_floors = current_floors.unwrap_or_else(|| {
        info!("no current floors set, randomizing");
        (0..num_passengers)
            .map(|_| rand::random::<i32>().abs() % num_floors)
            .collect()
    });

    let destination_floors = destination_floors.unwrap_or_else(|| {
        info!("no destination floors set, randomizing");
        let mut floors = vec![];
        for &current_floor in &current_floors {
            let mut dest_floor = rand::random::<i32>().abs() % num_floors;
            while dest_floor == current_floor {
                dest_floor = rand::random::<i32>().abs() % num_floors;
            }
            floors.push(dest_floor);
        }
        floors
    });

    // ensure the lengths match
    assert_eq!(current_floors.len(), destination_floors.len(), "Current and destination floors must have the same length.");
    assert_eq!(current_floors.len() as i32, num_floors, "Current floors must match number of floors.");
    assert_eq!(destination_floors.len() as i32, num_floors, "Destination floors must match number of floors.");

    // assert that all current and destination floors are within valid bounds
    for &floor in &current_floors {
        assert!(floor >= 0 && floor < num_floors, "Current floor {} is out of bounds.", floor);
    }

    for &floor in &destination_floors {
        assert!(floor >= 0 && floor < num_floors, "Destination floor {} is out of bounds.", floor);
    }

    for (i, (&current_floor, &destination_floor)) in current_floors.iter().zip(destination_floors.iter()).enumerate() {
        let p = Passagier::new(
            i as i32,
            current_floor,
            destination_floor,
            fahrkabinen.clone(),
            controller.clone(),
        );
        controller.write().unwrap().all_passengers.push(p);
    }

    controller
}

pub fn run_simulation(
    controller: Arc<RwLock<Controller>>,
) {
    info!("State before");
    debug_passenger_states(&controller);

    // Wait for all passengers to exit
    // TODO: some way to distinctly go through the states step by step would be awesome to test the different states, right now its kinda doing its own thing
    while controller.read().unwrap().all_passengers.iter().any(|p| {
        let p = p.read().unwrap();
        p.state != PassengerState::Exiting
    }) {}

    info!("All passengers have exited");
    info!("State after");
    debug_passenger_states(&controller);
}

fn debug_passenger_states(controller: &Arc<RwLock<Controller>>) {
    controller.read().unwrap().all_passengers.iter().map(|p| {
        Passagier::get_state(p)
    }).for_each(|p| {
        info!("P{:?}: {:?}->{:?} ({:?})", p.0, p.1, p.2, p.3);
    });
}

