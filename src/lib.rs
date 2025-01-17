use std::ops::Deref;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
enum DoorState {
    Open,
    Opening,
    Closing,
    Closed,
}
#[derive(Debug, PartialEq, Clone)]
enum PassengerState {
    WaitingOnFloor(i32),
    EnteringElevator,
    ChoosingLevel,
    InElevator,
    Exiting,
}

#[derive(Debug)]
struct Door {
    state: DoorState,
}

#[derive(Debug)]
struct Fahrkabine {
    id: i32,
    etage: i32,
    door: Door,
    passengers: Vec<i32>,
    max_passengers: usize,

    level_sender: Option<Arc<std::sync::mpsc::Sender<i32>>>,
}
struct Etage {
    id: i32,
}

struct Controller {
    fahrkabinen: Vec<Arc<std::sync::mpsc::Sender<i32>>>, //TODO sender either in controller or in kabine not in both
    etagen: Vec<Etage>,
    all_passengers: Vec<Arc<RwLock<Passagier>>>,
}

#[derive(Debug)]
struct Passagier {
    id: i32,
    etage: i32,
    dest_etage: i32,
    state: PassengerState,
}

impl Door {
    fn is_open(&self) -> bool {
        self.state == DoorState::Open //TODO change to not closed aka Open or Opening
    }
}

impl Fahrkabine {
    fn new(id: i32, etage: i32) -> Arc<RwLock<Fahrkabine>> {
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

    fn press_level_button(&self, etage: i32) {
        self.level_sender.as_ref().unwrap().send(etage);
    }

    fn add_passenger(kabine: &Arc<RwLock<Fahrkabine>>, passenger_id: i32) {
        println!(
            "Adding passenger {} to kabine {}",
            passenger_id,
            kabine.read().unwrap().id
        );
        let mut kabine = kabine.write().unwrap();
        kabine.passengers.push(passenger_id);
    }

    fn remove_passenger(kabine: &Arc<RwLock<Fahrkabine>>, passenger_id: i32) {
        println!(
            "Removing passenger {} to kabine {}",
            passenger_id,
            kabine.read().unwrap().id
        );

        let mut kabine = kabine.write().unwrap();
        kabine.passengers.retain(|&x| x != passenger_id);
    }
}

impl Controller {
    fn controll_cabin(
        kabinen_requests: std::sync::mpsc::Receiver<i32>,
        kabine: Arc<RwLock<Fahrkabine>>,
    ) {
        while let Ok(next_level) = kabinen_requests.recv() {
            println!(
                "Controller received request for Fahrkabine {:?} to got to floor {:?}",
                kabine.read().unwrap().id,
                next_level
            );
            Controller::move_to(&kabine, next_level);
            Controller::open_door(&kabine);
            Controller::await_passengers(&kabine);
            Controller::close_door(&kabine);
        }
        println!(
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
        println!("Fahrkabine {} moving to etage {}", kabine.id, etage);
        thread::sleep(std::time::Duration::from_secs(1));
        kabine.etage = etage;
    }

    fn open_door(kabine_lock: &Arc<RwLock<Fahrkabine>>) {
        let mut kabine = kabine_lock.write().unwrap();
        println!("Fahrkabine {} opening door", kabine.id);
        kabine.door.state = DoorState::Opening;
        drop(kabine);
        thread::sleep(std::time::Duration::from_millis(100));
        let mut kabine = kabine_lock.write().unwrap();
        kabine.door.state = DoorState::Open;
    }

    fn close_door(kabine_lock: &Arc<RwLock<Fahrkabine>>) {
        let mut kabine = kabine_lock.write().unwrap();
        println!("Fahrkabine {} closing door", kabine.id);
        kabine.door.state = DoorState::Closing;
        drop(kabine);
        thread::sleep(std::time::Duration::from_millis(100));
        let mut kabine = kabine_lock.write().unwrap();
        kabine.door.state = DoorState::Closed;
    }

    fn await_passengers(kabine_lock: &Arc<RwLock<Fahrkabine>>) {
        let kabine = kabine_lock.read().unwrap();
        println!("Fahrkabine {} waiting for passengers", kabine.id);
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

    fn send_floor_request(&self, etage: i32, direction: i32) {
        println!("Controller is sending request to Fahrkabine");
        //TODO inteligent selection of kabine
        let random_kabine = 1;
        self.fahrkabinen[random_kabine as usize].send(etage);
    }

    fn new(fahrkabinen: Vec<Arc<RwLock<Fahrkabine>>>, etagen: Vec<Etage>) -> Controller {
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

impl Passagier {
    fn new(
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
                    println!("Elevator is full");
                    if rand::random::<i32>().abs() % 10 == 0 {
                        println!("Passenger {} will Exit and wait for the next!", passagier.read().unwrap().id);
                        Passagier::exit_elevator(&passagier, &arrived_elevator);
                        thread::sleep(Duration::from_secs(5)); //TODO if we dont wait this will end in a deadlock
                        continue 'outer_loop;
                    }
                    else{
                        println!("Passenger {} will hope for another passenger to leave!", passagier.read().unwrap().id);
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
        println!("Passenger {} is pressing up or down button", passagier.id);
        controller.read().unwrap().send_floor_request(passagier.etage, 1);
    }
    fn press_level_button(passagier: &Arc<RwLock<Passagier>>, kabine: &Arc<RwLock<Fahrkabine>>) {
        let mut passagier = passagier.write().unwrap();
        passagier.state = PassengerState::ChoosingLevel;
        println!("Passenger {} is pressing level button", passagier.id);

        kabine
            .read()
            .unwrap()
            .press_level_button(passagier.dest_etage);
    }
    fn wait_for_elevator(
        passagier: &Arc<RwLock<Passagier>>,
        fahrkabinen: &Vec<Arc<RwLock<Fahrkabine>>>,
    ) -> Arc<RwLock<Fahrkabine>> {
        //TODO switch to changevar
        let passagier = passagier.write().unwrap();

        println!(
            "Passenger {} is waiting for elevator in floor {}",
            passagier.id, passagier.etage
        );
        loop {
            for kabine_lock in fahrkabinen.iter() {
                let kabine = kabine_lock.read().unwrap();
                if kabine.etage == passagier.etage && kabine.door.is_open() {
                    return kabine_lock.clone();
                }
            }
        }
    }
    fn enter_elevator(passagier: &Arc<RwLock<Passagier>>, kabine: &Arc<RwLock<Fahrkabine>>) {
        let mut passagier = passagier.write().unwrap();
        Fahrkabine::add_passenger(kabine, passagier.id);
        passagier.state = PassengerState::EnteringElevator;
        println!("Passenger {} is entering elevator", passagier.id);
    }
    fn wait_for_exit(passagier: &Arc<RwLock<Passagier>>, kabine: &Arc<RwLock<Fahrkabine>>) {
        let mut passagier = passagier.write().unwrap();

        passagier.state = PassengerState::InElevator;
        println!("Passenger {} is waiting for exit", passagier.id);
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
        println!("Passenger {} is exiting elevator", passagier.id);
    }
}

pub  fn run_simulation(num_elevators: usize, num_floors: i32, num_passengers: usize) {
    // create elevators
    let mut fahrkabinen = Vec::new();
    for i in 0..num_elevators {
        fahrkabinen.push(Fahrkabine::new(i as i32, 0));
    }

    // create floors
    let mut etagen = Vec::new();
    for i in 0..num_floors {
        etagen.push(Etage { id: i });
    }

    // create controller
    let controller = Arc::new(RwLock::new(Controller::new(
        fahrkabinen.clone(),
        etagen,
    )));

    println!("Main loop");
    for i in 0..num_passengers {
        let random_floor = rand::random::<i32>().abs() % num_floors;
        let mut random_floor_2 = rand::random::<i32>().abs() % num_floors;
        while random_floor_2 == random_floor {
            random_floor_2 = rand::random::<i32>().abs() % num_floors;
        }

        let p = Passagier::new(
            i as i32,
            random_floor,
            random_floor_2,
            fahrkabinen.clone(),
            controller.clone(),
        );
        controller.write().unwrap().all_passengers.push(p);
    }
    println!("Created all Passengers!");
    while controller.read().unwrap().all_passengers.iter().any(|p| {
        let p = p.read().unwrap();
        p.state != PassengerState::Exiting
    }) {}
    println!("All passengers have exited");
    for fahrkabine in fahrkabinen {
        println!("{:?}", fahrkabine);
    }
}
