mod cabin;
mod passenger;
mod controller;


pub use cabin::{DoorState, ElevatorState, Fahrkabine};
pub use passenger::{Direction, Passagier, PassengerState};
pub use controller::Controller;

//use crate::cabin::{Etage, Fahrkabine};
//use crate::controller::Controller;
//use crate::passenger::{Passagier, PassengerState};


pub fn run_simulation(
    num_elevators: usize,
    num_floors: i32,
    num_passengers: usize,
    max_passengers_per_cabin: usize,
    current_floors: Option<Vec<i32>>,
    destination_floors: Option<Vec<i32>>,
) {
    let controller = Controller::new(num_elevators, num_floors, max_passengers_per_cabin);
    
    let passengers = Passagier::create_passengers(
        num_passengers,
        num_floors,
        current_floors,
        destination_floors,
        controller.clone(),
    );

    Controller::start_simulation(controller, passengers);
}

