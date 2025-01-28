// lib.rs
mod cabin;
mod passenger;
mod controller;

pub use cabin::{DoorState, ElevatorState, Fahrkabine};
pub use passenger::{Passagier, PassengerState};
pub use controller::Controller;

use log::info;
use std::sync::{Arc, RwLock};

pub fn run_simulation(
    num_elevators: usize,
    num_floors: i32,
    num_passengers: usize,
    max_passengers_per_cabin: usize,
    current_floors: Option<Vec<i32>>,
    destination_floors: Option<Vec<i32>>,
) {
    env_logger::init();

    let elevators: Vec<_> = (0..num_elevators)
        .map(|i| Fahrkabine::new(i as i32, 0, max_passengers_per_cabin))
        .collect();

    let controller = Controller::new(elevators.clone());

    // Create passengers
    for i in 0..num_passengers {
        let current_floor = current_floors.as_ref().map_or(0, |v| v[i]);
        let destination_floor = destination_floors.as_ref().map_or(1, |v| v[i]);
        let passenger = Passagier::new(
            i as i32,
            current_floor,
            destination_floor,
            controller.clone(),
        );
        passenger.write().unwrap().request_elevator();
        controller.write().unwrap().add_passenger(passenger);
    }

    // Simulation loop
    loop {
        if controller.read().unwrap().passengers.iter().all(|p| {
            p.read().unwrap().state == PassengerState::Exiting
        }) {
            break;
        }
    }

    info!("Simulation complete.");
}