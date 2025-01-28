use crate::cabin:: Fahrkabine;
use crate::passenger::Passagier;
use log:: info;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use crate::PassengerState;
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};

//ToDo: Getter für Elevator schreiben
pub struct Controller {
    elevators: Vec<Arc<RwLock<Fahrkabine>>>,
    pub passengers: Vec<Arc<RwLock<Passagier>>>,
    senders: Vec<Arc<Sender<i32>>>,
    next_elevator: AtomicUsize,
}

impl Controller {
     pub fn new(elevators: Vec<Arc<RwLock<Fahrkabine>>>) -> Arc<RwLock<Self>> {
        let mut senders = vec![];
        let controller = Arc::new(RwLock::new(Controller {
            elevators: elevators.clone(),
            passengers: vec![],
            senders: vec![],
            next_elevator: AtomicUsize::new(0),
        }));

        for elevator in &elevators {
            let (tx, rx) = channel();
            let tx = Arc::new(tx);
            senders.push(tx.clone());
            let elevator_clone = elevator.clone();
            let controller_clone = controller.clone();
            thread::spawn(move || {
                Controller::control_elevator(rx, elevator_clone, controller_clone);
            });
        }

        let mut c = controller.write().unwrap();
        c.senders = senders;
        controller.clone()
    }


fn control_elevator(rx: mpsc::Receiver<i32>, elevator: Arc<RwLock<Fahrkabine>>, controller: Arc<RwLock<Controller>>) {
    while let Ok(target_floor) = rx.recv() {
        // Move elevator
        let mut elevator_guard = elevator.write().unwrap();
        elevator_guard.move_to(target_floor);
        elevator_guard.open_door();

        // Handle passengers using existing lock
        let current_floor = elevator_guard.current_floor;
        let  mut controller_guard = controller.write().unwrap();
        
        // Board passengers
        for passenger in &controller_guard.passengers {
            let mut passenger = passenger.write().unwrap();
            passenger.board_elevator(&mut elevator_guard);
        }
        
        // Exit passengers
        let passenger_ids: Vec<i32> = elevator_guard.passengers.clone();
        for pid in passenger_ids {
            if let Some(passenger) = controller_guard.passengers.iter().find(|p| p.read().unwrap().id == pid) {
                let mut passenger = passenger.write().unwrap();
                passenger.exit_elevator(&mut elevator_guard);
            }
        }

        controller_guard.passengers.retain(|p| {
            p.read().unwrap().state != PassengerState::Exiting
        });
        
        drop(controller_guard); // Explicitly release controller lock
        elevator_guard.close_door();
    }
}


    pub fn add_passenger(&mut self, passenger: Arc<RwLock<Passagier>>) {
        self.passengers.push(passenger);
    }


pub fn send_random_floor_request(&self, current_floor: i32, destination_floor: i32) {
    info!("[CONTROLLER] Received request from {} to {}", current_floor, destination_floor);
    
    // Send pickup floor first
    let mut rng = rand::rng();
    let elevator_idx = rng.random_range(0..self.senders.len());
    
    if let Some(sender) = self.senders.get(elevator_idx) {
        sender.send(current_floor).unwrap();
    }
    
    // Send destination floor after delay
    let senders = self.senders.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        if let Some(sender) = senders.first() {
            sender.send(destination_floor).unwrap();
        }
    });
}

pub fn send_floor_request(&self, current_floor: i32, destination_floor: i32) {
    info!("[CONTROLLER] Received request from {} to {}", current_floor, destination_floor);
    
    // Send pickup floor first
    if let Some(sender) = self.senders.first() {
        sender.send(current_floor).unwrap();
    }
    
    // Send destination floor after delay
    let senders = self.senders.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        if let Some(sender) = senders.first() {
            sender.send(destination_floor).unwrap();
        }
    });

}
pub fn send_rr_floor_request(&self, current_floor: i32, destination_floor: i32) {
        info!("[CONTROLLER] Received request from {} to {}", current_floor, destination_floor);
        
        // Get and increment elevator index atomically
        let idx = self.next_elevator.fetch_add(1, Ordering::Relaxed) % self.senders.len();
        let sender = &self.senders[idx];
        
        // Send pickup request
        sender.send(current_floor).unwrap();
        
        // Send destination to same elevator after delay
        let sender_clone = sender.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            sender_clone.send(destination_floor).unwrap();
        });
    }
}
