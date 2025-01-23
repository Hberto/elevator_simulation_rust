use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;
use std::thread;
use crate::cabin::{DoorState, Etage, Fahrkabine};
//use crate::logger::{Logger};
use log::{info};
use crate::passenger::{Passagier, PassengerState};


pub struct Controller {
    fahrkabinen: Vec<Arc<std::sync::mpsc::Sender<i32>>>, //TODO sender either in controller or in kabine not in both
    etagen: Vec<Etage>,
    pub(crate) all_passengers: Vec<Arc<RwLock<Passagier>>>,
}

impl Controller {
    fn controll_cabin(
        kabinen_requests: std::sync::mpsc::Receiver<i32>,
        kabine: Arc<RwLock<Fahrkabine>>,
    ) {
        while let Ok(next_level) = kabinen_requests.recv() {
            info!(
                "Controller received request for Fahrkabine {:?} to got to floor {:?}",
                kabine.read().unwrap().id,
                next_level
            );
            Controller::move_to(&kabine, next_level);
            Controller::open_door(&kabine);
            Controller::await_passengers(&kabine);
            Controller::close_door(&kabine);
        }
        info!(
            "No More Requests for Fahrkabine: {:?}",
            kabine.read().unwrap().id
        );
    }

    /* fn get_fahrkabinen_states(&self) -> Vec<(i32, i32, DoorState, Vec<i32>)> {
        self.fahrkabinen.iter().map(|kabine| {
            let kabine = kabine.read().umwrap();
            (kabine.id, kabine.etage, kabine.door.state.clone(), kabine.passengers.clone())
        }).collect()
    } */

    // Method to get the states of all Passagiere
    pub fn get_passenger_states(&self) -> Vec<(i32, PassengerState)> {
        self.all_passengers.iter().map(|p| {
            let passagier = p.read().unwrap();
            (passagier.id, passagier.state.clone())
        }).collect()
    }
    fn move_to(kabine: &Arc<RwLock<Fahrkabine>>, etage: i32) {
        let mut kabine = kabine.write().unwrap();
        info!("Fahrkabine {} moving to etage {}", kabine.id, etage);
        thread::sleep(std::time::Duration::from_secs(1));
        kabine.etage = etage;
    }

    fn open_door(kabine_lock: &Arc<RwLock<Fahrkabine>>) {
        let mut kabine = kabine_lock.write().unwrap();
        info!("Fahrkabine {} opened door", kabine.id);

        kabine.door.state = DoorState::Opening;
        drop(kabine);
        thread::sleep(std::time::Duration::from_millis(100));
        let mut kabine = kabine_lock.write().unwrap();
        kabine.door.state = DoorState::Open;
    }

    fn close_door(kabine_lock: &Arc<RwLock<Fahrkabine>>) {
        let mut kabine = kabine_lock.write().unwrap();
        info!("Fahrkabine {} closing door", kabine.id);
        kabine.door.state = DoorState::Closing;
        drop(kabine);
        thread::sleep(std::time::Duration::from_millis(100));
        let mut kabine = kabine_lock.write().unwrap();
        kabine.door.state = DoorState::Closed;
    }

    fn await_passengers(kabine_lock: &Arc<RwLock<Fahrkabine>>) {
        let kabine = kabine_lock.read().unwrap();
        info!("Fahrkabine {} waiting for passengers", kabine.id);
        drop(kabine);
        thread::sleep(std::time::Duration::from_micros(100));
        loop {
            let kabine = kabine_lock.read().unwrap();
            let num_passengers = kabine.passengers.len();
            if num_passengers <= kabine.max_passengers {
                break;
            }
        }
    }

    pub(crate) fn send_floor_request(&self, etage: i32, direction: i32) {
        println!();
        info!("Controller is sending request to Fahrkabine");
        //TODO inteligent selection of kabine
        let random_kabine = 1;
        self.fahrkabinen[random_kabine as usize].send(etage);
    }

    pub(crate) fn new(fahrkabinen: Vec<Arc<RwLock<Fahrkabine>>>, etagen: Vec<Etage>) -> Controller {
        let mut kabinen_senders = vec![];
        for kabine in fahrkabinen {
            let (tx, rx) = channel();
            let tx = Arc::new(tx);
            kabinen_senders.push(tx.clone());
            kabine.write().unwrap().level_sender = Some(tx.clone());
            thread::spawn(move || {
                Controller::controll_cabin(rx, kabine);
            });
        }

        Controller {
            etagen: etagen,
            fahrkabinen: kabinen_senders,
            all_passengers: vec![],
        }
    }
}
