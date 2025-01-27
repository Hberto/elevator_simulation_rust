use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use crate::cabin::Fahrkabine;
use crate::controller::Controller;
use log::info;

#[derive(Debug, PartialEq, Clone)]
pub enum PassengerState {
    WaitingOnFloor(i32),
    EnteringElevator,
    ChoosingLevel,
    InElevator,
    Exiting,
}

#[derive(Debug)]
pub struct Passagier {
    pub(crate) id: i32,
    etage: i32,
    dest_etage: i32,
    pub(crate) state: PassengerState,
}

impl Passagier {
    pub(crate) fn new(
        id: i32,
        etage: i32,
        dest_etage: i32,
        fahrkabinen: Vec<Arc<RwLock<Fahrkabine>>>,
        controller: Arc<RwLock<Controller>>,
    ) -> Arc<RwLock<Passagier>> {
        let p = Passagier {
            id: id,
            etage: etage,
            dest_etage: dest_etage,
            state: PassengerState::WaitingOnFloor(etage),
        };

        let l = Arc::new(RwLock::new(p));
        let f = l.clone();
        thread::spawn(move || Passagier::lifecycle(l, fahrkabinen, controller));
        f
    }

    pub fn get_state(passagier: &Arc<RwLock<Passagier>>) -> (i32, i32, i32, PassengerState) {
        let passagier = passagier.read().unwrap();
        (passagier.id, passagier.etage, passagier.dest_etage, passagier.state.clone())
    }

    fn lifecycle(passagier: Arc<RwLock<Passagier>>, fahrkabinen: Vec<Arc<RwLock<Fahrkabine>>>, controller: Arc<RwLock<Controller>>) {
        'outer_loop: loop {
            Passagier::press_up_or_down_button(&passagier, &controller);
            let arrived_elevator = Passagier::wait_for_elevator(&passagier, &fahrkabinen);
            Passagier::enter_elevator(&passagier, &arrived_elevator);
            'inner_loop: loop {

                if arrived_elevator.read().unwrap().passengers.len()
                    > arrived_elevator.read().unwrap().max_passengers
                {
                    info!("Elevator is full");
                    if rand::random::<i32>().abs() % 10 == 0 {
                        info!("Passenger {} will Exit and wait for the next!", passagier.read().unwrap().id);
                        Passagier::exit_elevator(&passagier, &arrived_elevator);
                        thread::sleep(Duration::from_secs(5)); //TODO if we dont wait this will end in a deadlock
                        continue 'outer_loop;
                    }
                    else{
                        info!("Passenger {} will hope for another passenger to leave!", passagier.read().unwrap().id);
                        thread::sleep(Duration::from_micros(rand::random::<u64>() % 200));

                    }
                }
                else {
                    break 'inner_loop;
                }

            }
            Passagier::press_level_button(&passagier, &arrived_elevator);
            Passagier::wait_for_exit(&passagier, &arrived_elevator);
            Passagier::exit_elevator(&passagier, &arrived_elevator);
            break;
        }
    }
    fn press_up_or_down_button(passagier: &Arc<RwLock<Passagier>>, controller: &Arc<RwLock<Controller>>) {
        let mut passagier = passagier.write().unwrap();
        passagier.state = PassengerState::WaitingOnFloor(passagier.etage);
        info!("Passenger {} is pressing up or down button", passagier.id);
        controller.read().unwrap().send_floor_request(passagier.etage, 1);
    }
    fn press_level_button(passagier: &Arc<RwLock<Passagier>>, kabine: &Arc<RwLock<Fahrkabine>>) {
        let mut passagier = passagier.write().unwrap();
        passagier.state = PassengerState::ChoosingLevel;
        info!("Passenger {} is pressing level button", passagier.id);

        kabine
            .read()
            .unwrap()
            .press_level_button(passagier.dest_etage);
    }
    // passenger.rs
fn wait_for_elevator(
    passagier: &Arc<RwLock<Passagier>>,
    fahrkabinen: &Vec<Arc<RwLock<Fahrkabine>>>,
) -> Arc<RwLock<Fahrkabine>> {
    let passagier_guard = passagier.read().unwrap();
    let current_floor = passagier_guard.etage;
    drop(passagier_guard);

    loop {
        for kabine_lock in fahrkabinen {
            let kabine = kabine_lock.read().unwrap();
            if kabine.etage == current_floor && kabine.door.is_open() {
                return kabine_lock.clone();
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
    fn enter_elevator(passagier: &Arc<RwLock<Passagier>>, kabine: &Arc<RwLock<Fahrkabine>>) {
        let mut passagier = passagier.write().unwrap();
        Fahrkabine::add_passenger(kabine, passagier.id);
        passagier.state = PassengerState::EnteringElevator;
        info!("Passenger {} is entering elevator", passagier.id);
    }
    fn wait_for_exit(passagier: &Arc<RwLock<Passagier>>, kabine: &Arc<RwLock<Fahrkabine>>) {
        let mut passagier = passagier.write().unwrap();

        passagier.state = PassengerState::InElevator;
        info!("Passenger {} is waiting for exit", passagier.id);
        //TODO switch to changevar
        loop {
            let kabine = kabine.read().unwrap();
            if kabine.etage == passagier.dest_etage && kabine.door.is_open() {
                break;
            }
        }
    }
    fn exit_elevator(passagier: &Arc<RwLock<Passagier>>, kabine: &Arc<RwLock<Fahrkabine>>) {
        let mut passagier = passagier.write().unwrap();
        Fahrkabine::remove_passenger(kabine, passagier.id);
        passagier.state = PassengerState::Exiting;
        info!("Passenger {} is exiting elevator", passagier.id);
    }
}
