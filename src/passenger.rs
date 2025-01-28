// passenger.rs
use std::sync::{Arc, RwLock};
use log::info;
use crate::cabin::Fahrkabine;
use crate::controller::Controller;
use std::thread;

#[derive(Debug, PartialEq, Clone)]
pub enum PassengerState {
    Waiting,
    Boarding,
    Riding,
    Exiting,
}

pub struct Passagier {
    pub id: i32,
    pub current_floor: i32,
    pub destination_floor: i32,
    pub state: PassengerState,
    controller: Arc<RwLock<Controller>>,
}

impl Passagier {
    pub fn new(
        id: i32,
        current_floor: i32,
        destination_floor: i32,
        controller: Arc<RwLock<Controller>>,
    ) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Passagier {
            id,
            current_floor,
            destination_floor,
            state: PassengerState::Waiting,
            controller,
        }))
    }

    
pub fn request_elevator(&mut self) {
    info!("Passenger {} requesting elevator", self.id);
    let controller = self.controller.clone();
    let current = self.current_floor;
    let destination = self.destination_floor;
    
    thread::spawn(move || {
        controller.write().unwrap().send_floor_request(current, destination);
    });
}

pub fn board_elevator(&mut self, elevator: &mut Fahrkabine) {
    if elevator.current_floor == self.current_floor 
        && elevator.passengers.len() < elevator.max_passengers 
    {
        info!("Passenger {} boarding elevator {}", self.id, elevator.id);
        elevator.passengers.push(self.id);
        self.state = PassengerState::Riding;
        self.current_floor = -1; // Mark as in transit
    }
}

pub fn exit_elevator(&mut self, elevator: &mut Fahrkabine) {
    if elevator.current_floor == self.destination_floor 
        && elevator.passengers.contains(&self.id) 
    {
        info!("Passenger {} exiting elevator {}", self.id, elevator.id);
        elevator.passengers.retain(|&id| id != self.id);
        self.state = PassengerState::Exiting;
        self.current_floor = self.destination_floor;
    }
}
}