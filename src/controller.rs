use std::sync::{Arc, RwLock};
use std::collections::{HashMap,VecDeque};
use std::time::Duration;
use log::info;
use super::{Fahrkabine, Direction, Passagier, PassengerState, ElevatorState};

pub struct Controller {
    elevators: Vec<Arc<RwLock<Fahrkabine>>>,
    floor_requests: HashMap<i32, VecDeque<Direction>>,
    passengers: Vec<Arc<RwLock<Passagier>>>,
}

impl Controller {
    pub fn new(num_elevators: usize, num_floors: i32, max_passengers: usize) -> Arc<RwLock<Self>> {
        let elevators = (0..num_elevators)
            .map(|i| Fahrkabine::new(i as i32, max_passengers))
            .collect();
        
        Arc::new(RwLock::new(Self {
            elevators,
            floor_requests: HashMap::with_capacity(num_floors as usize),
            passengers: vec![],
        }))
    }

    pub fn start_simulation(controller: Arc<RwLock<Self>>, passengers: Vec<Arc<RwLock<Passagier>>>) {
        controller.write().unwrap().passengers = passengers.clone();
        let controller_clone = controller.clone();
        std::thread::spawn(move || {
            let mut last_update = std::time::Instant::now();
            loop {
                if last_update.elapsed() >= Duration::from_millis(100) {
                    controller_clone.write().unwrap().update();
                    last_update = std::time::Instant::now();
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        });
        
        for passenger in passengers {
            let p_clone = passenger.clone();
            let controller = controller.clone();
            std::thread::spawn(move || {
                loop {
                    if p_clone.read().unwrap().state == PassengerState::Exiting {
                        break;
                    }
                    p_clone.write().unwrap().update();
                    controller.write().unwrap().assign_elevators();
                    std::thread::sleep(Duration::from_secs(1));
                }
            });
        }
    }

    fn update(&mut self) {
        for elevator in &self.elevators {
            elevator.write().unwrap().update();
        }
    }

    pub fn add_floor_request(&mut self, floor: i32, direction: Direction) {
        self.floor_requests
            .entry(floor)
            .or_insert_with(VecDeque::new)
            .push_back(direction);
    }

    fn assign_elevators(&mut self) {
    info!("Controller assigning elevators to floor requests: {:?}", self.floor_requests);
    // Take current requests and replace with an empty HashMap
    let mut current_requests = std::mem::take(&mut self.floor_requests);
    
    // Process each floor and direction in current_requests
    for (floor, directions) in current_requests.iter_mut() {
        while let Some(direction) = directions.pop_front() {
            if let Some(elevator) = self.find_best_elevator(*floor, &direction) {
                elevator.write().unwrap().add_target_floor(*floor);
            }
        }
    }
    
    // Merge any new requests that were added during processing
    for (floor, mut new_directions) in std::mem::take(&mut self.floor_requests) {
        current_requests.entry(floor)
            .or_insert_with(VecDeque::new)
            .append(&mut new_directions);
    }
    
    // Restore the merged requests back into self.floor_requests
    self.floor_requests = current_requests;
}

//    fn find_best_elevator(&self, floor: i32, direction: &Direction) -> Option<Arc<RwLock<Fahrkabine>>> {
//    info!("Controller searching for best elevator for floor {} (direction: {:?})", floor, direction);
//    self.elevators.iter().min_by_key(|e| {
//        let e = e.read().unwrap();
//        let distance = (e.current_floor - floor).abs();
//        // Prefer elevators moving towards the floor or idle
//        match e.state {
//            ElevatorState::MovingUp if e.current_floor <= floor => distance,
//            ElevatorState::MovingDown if e.current_floor >= floor => distance,
//            ElevatorState::Idle => distance,
//            _ => i32::MAX, // Ignore others
//            }
//        }).cloned()
//    }
//
//fn find_best_elevator(&self, floor: i32, direction: &Direction) -> Option<Arc<RwLock<Fahrkabine>>> {
//    self.elevators.iter().min_by_key(|e| {
//        let e = e.read().unwrap();
//        let distance = (e.current_floor - floor).abs();
//        match (&e.state, direction) {
//            (ElevatorState::MovingUp, Direction::Up) => distance,
//            (ElevatorState::MovingDown, Direction::Down) => distance,
//            (ElevatorState::Idle, _) => distance,
//            _ => i32::MAX,
//        }
//    }).cloned()
//}

fn find_best_elevator(&self, floor: i32, _direction: &Direction) -> Option<Arc<RwLock<Fahrkabine>>> {
    self.elevators.iter().min_by_key(|e| {
        let e = e.read().unwrap();
        (e.current_floor - floor).abs()
    }).cloned()
}

    pub fn get_available_elevator(
        &self,
        floor: i32,
        direction: &Direction,
    ) -> Option<Arc<RwLock<Fahrkabine>>> {
        self.elevators
            .iter()
            .filter(|e| {
                let e = e.read().unwrap();
                e.current_floor == floor && e.can_accept_passenger(direction)
            })
            .next()
            .cloned()
    }

    pub fn get_elevator_with_passenger(&self, passenger_id: i32) -> Option<Arc<RwLock<Fahrkabine>>> {
        self.elevators
            .iter()
            .find(|e| e.read().unwrap().passengers.contains(&passenger_id))
            .cloned()
    }

}