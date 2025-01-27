use std::sync::{Arc, RwLock};
use log::info;
use rand::Rng;
use super::controller::Controller;
use super::DoorState;
//mod cabin;
//pub use cabin::DoorState;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PassengerState {
    Waiting,
    Boarding,
    Riding,
    Exiting,
}

pub struct Passagier {
    pub id: i32,
    pub current_floor: i32,
    pub target_floor: i32,
    pub direction: Direction,
    pub state: PassengerState,
    pub controller: Arc<RwLock<Controller>>,
}

impl Passagier {
    pub fn new(
        id: i32,
        current_floor: i32,
        target_floor: i32,
        controller: Arc<RwLock<Controller>>,
    ) -> Arc<RwLock<Self>> {
        let direction = if target_floor > current_floor {
            Direction::Up
        } else {
            Direction::Down
        };
        
        Arc::new(RwLock::new(Self {
            id,
            current_floor,
            target_floor,
            direction,
            state: PassengerState::Waiting,
            controller,
        }))
    }

    pub fn create_passengers(
        num: usize,
        num_floors: i32,
        current_floors: Option<Vec<i32>>,
        target_floors: Option<Vec<i32>>,
        controller: Arc<RwLock<Controller>>,
    ) -> Vec<Arc<RwLock<Self>>> {
        let mut rng = rand::rng();
        
        (0..num)
            .map(|i| {
                let current = current_floors.as_ref().map_or_else(
                    || rng.random_range(0..num_floors),
                    |v| v[i % v.len()],
                );
                
                let mut target = target_floors.as_ref().map_or_else(
                    || rng.random_range(0..num_floors),
                    |v| v[i % v.len()],
                );
                
                while target == current {
                    target = rng.random_range(0..num_floors);
                }
                
                Self::new(i as i32, current, target, controller.clone())
            })
            .collect()
    }

    pub fn update(&mut self) {
        match self.state {
        PassengerState::Waiting => {
            self.request_elevator();
            self.check_for_available_elevator(); // Add this line
        },
        PassengerState::Boarding => self.board_elevator(),
        PassengerState::Riding => self.check_arrival(),
        PassengerState::Exiting => (),
    }
}

    fn check_for_available_elevator(&mut self) {
    if let Some(_elevator) = self.controller.read().unwrap().get_available_elevator(
        self.current_floor,
        &self.direction,
    ) {
        self.state = PassengerState::Boarding;
    }
    }

    fn request_elevator(&mut self) {
    info!("Passenger {} requesting elevator on floor {}", self.id, self.current_floor);
    // Add the floor request to the controller
    self.controller
        .write()
        .unwrap()
        .add_floor_request(self.current_floor, self.direction.clone());

    // Check if there's an available elevator now
    if let Some(_elevator) = self.controller.read().unwrap().get_available_elevator(
        self.current_floor,
        &self.direction,
    ) {
        // Transition to Boarding if an elevator is available
        self.state = PassengerState::Boarding;
        }
    }

    fn board_elevator(&mut self) {
        info!("Passenger {} attempting to board elevator...", self.id);
        if let Some(elevator) = self.controller.read().unwrap().get_available_elevator(
            self.current_floor,
            &self.direction,
        ) { 
            let mut elevator = elevator.write().unwrap();
            elevator.passengers.push(self.id);
            info!("Passenger {} boarding elevator {}", self.id, elevator.id);
            self.state = PassengerState::Riding;
            elevator.add_target_floor(self.target_floor);
        }
    }

    fn check_arrival(&mut self) {
    info!("Passenger {} checking arrival...", self.id);
    let controller = self.controller.read().unwrap();
    if let Some(elevator) = controller.get_elevator_with_passenger(self.id) {
        let elevator_guard = elevator.read().unwrap();
        let elevator_floor = elevator_guard.current_floor;
        let door_open = elevator_guard.door.state == DoorState::Open;
        drop(elevator_guard); // Release the lock early

        if elevator_floor == self.target_floor && door_open {
            self.state = PassengerState::Exiting;
            info!("Passenger {} exiting elevator {}", self.id, elevator.read().unwrap().id);
            elevator.write().unwrap().passengers.retain(|&id| id != self.id);
        }
    }
}
}