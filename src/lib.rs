// lib.rs
mod cabin;
mod passenger;
mod controller;

pub use cabin::{DoorState, ElevatorState, Fahrkabine};
pub use passenger::{Passagier, PassengerState};
pub use controller::Controller;

use log::info;
use std::sync::{Arc, RwLock};
use rand::Rng;
use std::time::Duration;

pub fn run_simulation(
    num_elevators: usize,
    num_floors: i32,
    num_passengers: usize,
    max_passengers_per_cabin: usize,
) {
    let elevators: Vec<_> = (0..num_elevators)
        .map(|i| Fahrkabine::new(i as i32, 0, max_passengers_per_cabin))
        .collect();

    let controller = Controller::new(elevators.clone());

    create_random_passengers(num_passengers, num_floors, &controller);

    // Simulation loop
    loop {
        let all_exited = controller.read().unwrap().passengers.iter().all(|p| {
            p.read().unwrap().state == PassengerState::Exiting
        });
        if all_exited {
            break;
        }
        // Add small sleep to prevent tight loop
        std::thread::sleep(Duration::from_millis(100));
    }
    info!("------ELEVATOR:STATUS------");
    for fahrkabine in elevators {
        info!("{:#?}", fahrkabine);
    }
    info!("Simulation complete.");
}


pub fn create_fixed_passengers(
    num_passengers: i32,
    num_floors: i32,
    current_floor: i32,
    destination_floor: i32,
    controller: &Arc<RwLock<Controller>>,
){
    for i in 0..num_passengers {
        if current_floor < num_floors || current_floor > num_floors || destination_floor < num_floors || destination_floor > num_floors {
            panic!("Invalid floor number");
        }
        let passenger = Passagier::new(i as i32, current_floor, destination_floor, controller.clone());
        passenger.write().unwrap().request_elevator();
        controller.write().unwrap().add_passenger(passenger);
        
    }
}

fn create_random_passengers(
    num_passengers: usize,
    num_floors: i32,
    controller: &Arc<RwLock<Controller>>,
){
    for i in 0..num_passengers {
        let mut rng = rand::rng();
        let current_floor = rng.random_range(0..num_floors);
        let mut destination_floor = rng.random_range(0..num_floors);

        // Ensure current and destination floors are not the same
        while destination_floor == current_floor {
            destination_floor = rng.random_range(0..num_floors);
        }
        info!("current_floor: {}", current_floor);
        info!("destination_floor: {}", destination_floor);

        let passenger = Passagier::new(i as i32, current_floor, destination_floor, controller.clone());
        passenger.write().unwrap().request_elevator();
        controller.write().unwrap().add_passenger(passenger);
    }
}