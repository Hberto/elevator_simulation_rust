use std::sync::{Arc, RwLock};
use log::info;
// use e

#[derive(Debug, PartialEq, Clone)]
pub enum DoorState {
    Open,
    Opening,
    Closing,
    Closed,
}

#[derive(Debug)]
pub struct Door {
    pub(crate) state: DoorState,
}

#[derive(Debug)]
pub struct Fahrkabine {
    pub(crate) id: i32,
    pub(crate) etage: i32,
    pub(crate) door: Door,
    pub(crate) passengers: Vec<i32>,
    pub(crate) max_passengers: usize,

    pub(crate) level_sender: Option<Arc<std::sync::mpsc::Sender<i32>>>,
}
pub struct Etage {
    pub(crate) id: i32,
}

impl Door {
    pub(crate) fn is_open(&self) -> bool {
        self.state == DoorState::Open //TODO change to not closed aka Open or Opening
    }
}

impl Fahrkabine {
    pub fn new(id: i32, etage: i32) -> Arc<RwLock<Fahrkabine>> {
        Arc::new(RwLock::new(Fahrkabine {
            id: id,
            etage: etage,
            door: Door {
                state: DoorState::Closed,
            },
            passengers: vec![],
            max_passengers: 2,
            level_sender: None,
        }))
    }

    pub  fn get_state(kabine: &Arc<RwLock<Fahrkabine>>) -> (i32, i32, DoorState, Vec<i32>) {
        let kabine = kabine.read().unwrap();
        (kabine.id, kabine.etage, kabine.door.state.clone(), kabine.passengers.clone())
    }

    pub(crate) fn press_level_button(&self, etage: i32) {
        self.level_sender.as_ref().unwrap().send(etage);
    }

    pub(crate) fn add_passenger(kabine: &Arc<RwLock<Fahrkabine>>, passenger_id: i32) {
        info!(
            "Adding passenger {} to kabine {}",
            passenger_id,
            kabine.read().unwrap().id
        );
        let mut kabine = kabine.write().unwrap();
        kabine.passengers.push(passenger_id);
    }

    pub(crate) fn remove_passenger(kabine: &Arc<RwLock<Fahrkabine>>, passenger_id: i32) {
        info!(
            "Removing passenger {} to kabine {}",
            passenger_id,
            kabine.read().unwrap().id
        );

        let mut kabine = kabine.write().unwrap();
        kabine.passengers.retain(|&x| x != passenger_id);
    }
}