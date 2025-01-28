use std::sync::{Arc, RwLock};
use log::{info,warn, debug};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
pub enum DoorState {
    Open,
    Opening,
    Closing,
    Closed,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElevatorState {
    Idle,
    Moving,
    Stopped,
}

#[derive(Debug)]
pub struct Door {
    pub state: DoorState,
}

#[derive(Debug)]
pub struct Fahrkabine {
    pub id: i32,
    pub current_floor: i32,
    pub door: Door,
    pub passengers: Vec<i32>,
    pub max_passengers: usize,
    pub state: ElevatorState,
}

impl Fahrkabine {
    pub fn new(id: i32, current_floor: i32, max_passengers: usize) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Fahrkabine {
            id,
            current_floor,
            door: Door {
                state: DoorState::Closed,
            },
            passengers: vec![],
            max_passengers,
            state: ElevatorState::Idle,
        }))
    }

    pub fn move_to(&mut self, target_floor: i32) {
    if self.current_floor == target_floor {
        return;
    }
    
    self.state = ElevatorState::Moving;
    info!("Elevator {} moving from {} to {}", self.id, self.current_floor, target_floor);
    
    // Simulate actual movement time
    let steps = (self.current_floor - target_floor).abs();
    thread::sleep(Duration::from_millis(200 * steps as u64));
    
    self.current_floor = target_floor;
    self.state = ElevatorState::Stopped;
}

    pub fn open_door(&mut self) {
        self.door.state = DoorState::Opening;
        info!("Elevator {} opening door", self.id);
        // Simulate door opening
        self.door.state = DoorState::Open;
    }

    pub fn close_door(&mut self) {
        self.door.state = DoorState::Closing;
        info!("Elevator {} closing door", self.id);
        // Simulate door closing
        self.door.state = DoorState::Closed;
    }

    pub fn add_passenger(&mut self, passenger_id: i32) {
        debug!("[ELEVATOR {}] Attempting to add passenger {}", self.id, passenger_id);
        if self.passengers.len() < self.max_passengers {
            self.passengers.push(passenger_id);
            info!("[ELEVATOR {}] Passenger {} entered", self.id, passenger_id);
        } else {
            warn!("[ELEVATOR {}] Full! Passenger {} can't enter", self.id, passenger_id);
        }
    }

    pub fn remove_passenger(&mut self, passenger_id: i32) {
        debug!("[ELEVATOR {}] Attempting to remove passenger {}", self.id, passenger_id);
        let before = self.passengers.len();
        self.passengers.retain(|&id| id != passenger_id);
        if self.passengers.len() < before {
            info!("[ELEVATOR {}] Passenger {} exited", self.id, passenger_id);
        } else {
            warn!("[ELEVATOR {}] Passenger {} not found!", self.id, passenger_id);
        }
    }
}