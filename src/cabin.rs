use std::sync::{Arc, RwLock, Weak};
use std::thread;
use std::time::Duration;
use log::{info, warn};
use super::Direction;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum DoorState {
    Open,
    Opening,
    Closing,
    Closed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElevatorState {
    Idle,
    MovingUp,
    MovingDown,
    Stopped,
}

#[derive(Debug)]
pub struct Door {
    pub state: DoorState,
    pub is_obstructed: bool,
}

#[derive(Debug)]
pub struct Fahrkabine {
    pub id: i32,
    pub current_floor: i32,
    pub door: Door,
    pub state: ElevatorState,
    pub passengers: Vec<i32>,
    pub max_passengers: usize,
    pub target_floors: VecDeque<i32>,
    weak_self: Weak<RwLock<Self>>,
}

impl Fahrkabine {
    pub fn new(id: i32, max_passengers: usize) -> Arc<RwLock<Self>> {
        let cabin = Arc::new(RwLock::new(Self {
        id,
        current_floor: 0,
        door: Door {
            state: DoorState::Closed,
            is_obstructed: false,
        },
        state: ElevatorState::Idle,
        passengers: vec![],
        max_passengers,
        target_floors: VecDeque::new(),
        weak_self: Weak::new(), // Temporary placeholder
    }));

    // Set weak_self to point to the Arc
    {
        let mut cabin_lock = cabin.write().unwrap();
        cabin_lock.weak_self = Arc::downgrade(&cabin);
    }

    // Start elevator thread
    let cabin_clone = cabin.clone();
    thread::spawn(move || {
        loop {
            let mut cabin = cabin_clone.write().unwrap();
            cabin.update();
            drop(cabin);
            thread::sleep(Duration::from_millis(100));
        }
    });

    cabin
    }

    pub fn add_target_floor(&mut self, floor: i32) {
        if floor != self.current_floor && !self.target_floors.contains(&floor) {
        self.target_floors.push_back(floor);
    }
    }

    pub fn update(&mut self) {
        match self.state {
            ElevatorState::Idle => self.process_next_target(),
            ElevatorState::MovingUp => self.move_to_next_floor(1),
            ElevatorState::MovingDown => self.move_to_next_floor(-1),
            ElevatorState::Stopped => self.handle_stopped_state(),
        }
    }

    pub fn process_next_target(&mut self) {
        if let Some(&target) = self.target_floors.front() {
            self.state = if target > self.current_floor {
                ElevatorState::MovingUp
            } else {
                ElevatorState::MovingDown
            };
        }
    }

    pub fn move_to_next_floor(&mut self, direction: i32) {
    let new_floor = self.current_floor + direction;
    if new_floor < 0 || new_floor >= 3 { // Replace NUM_FLOORS with your variable
        warn!("Invalid floor {}", new_floor);
        return;
    }

    info!("Elevator {} moving {}", self.id, if direction == 1 { "up" } else { "down" });
    thread::sleep(Duration::from_secs(1));
    self.current_floor = new_floor;

    if let Some(&target) = self.target_floors.front() {
        if self.current_floor == target {
            self.state = ElevatorState::Stopped;
            self.target_floors.pop_front();
            if let Some(arc_self) = self.weak_self.upgrade() {
                Fahrkabine::open_doors(arc_self);
            }
        }
    }
}

    pub fn open_doors(cabin: Arc<RwLock<Self>>) {
        // Set door state to Opening
    {
        let mut cabin_lock = cabin.write().unwrap();
        cabin_lock.door.state = DoorState::Opening;
    }

    info!("Elevator {} doors opening", cabin.read().unwrap().id);
    let weak_cabin = Arc::downgrade(&cabin);
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        if let Some(cabin_arc) = weak_cabin.upgrade() {
            let mut cabin = cabin_arc.write().unwrap();
            if cabin.door.state == DoorState::Opening {
                cabin.door.state = DoorState::Open;
                Fahrkabine::start_door_timer(cabin_arc.clone());
            }
        }
    });
    }

    fn start_door_timer(cabin: Arc<RwLock<Self>>) {
         let weak_cabin = Arc::downgrade(&cabin);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));
            if let Some(cabin_arc) = weak_cabin.upgrade() {
                let mut cabin = cabin_arc.write().unwrap();
                if cabin.door.state == DoorState::Open && !cabin.door.is_obstructed {
                    cabin.close_doors();
                }
            }
        });
    }

    pub fn close_doors(&mut self) {
        info!("Elevator {} doors closing", self.id);
        if self.passengers.len() > self.max_passengers {
            warn!("Elevator {} over capacity! Doors remain open.", self.id);
            return;
        }

        self.door.state = DoorState::Closing;
        info!("Elevator {} doors closing", self.id);
    }

    pub fn can_accept_passenger(&self, direction: &Direction) -> bool {
        self.door.state == DoorState::Open &&
        self.passengers.len() < self.max_passengers &&
        match self.state {
            ElevatorState::Idle => true,
            ElevatorState::MovingUp => *direction == Direction::Up,
            ElevatorState::MovingDown => *direction == Direction::Down,
            _ => false,
        }
    }

    pub fn handle_stopped_state(&mut self) {
        info!("Elevator {} stopped at floor {}", self.id, self.current_floor);
        self.state = ElevatorState::Idle;
    }
}