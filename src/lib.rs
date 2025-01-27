// lib.rs
use std::sync::{Arc, RwLock};
use crossbeam_channel::Sender;
use log::info;

pub mod cabin;
pub mod controller;
pub mod passenger;
pub mod gui;

pub use cabin::{DoorState, Fahrkabine};
pub use controller::Controller;
pub use passenger::{Passagier, PassengerState};

pub fn run_simulation(
    num_elevators: usize,
    num_floors: i32,
    num_passengers: usize,
    max_passengers_per_cabin: usize,
    current_floors: Option<Vec<i32>>,
    destination_floors: Option<Vec<i32>>,
    ui_sender: Sender<gui::UIState>,
) {
    // Create elevators
    let mut fahrkabinen = Vec::new();
    for i in 0..num_elevators {
        fahrkabinen.push(Fahrkabine::new(i as i32, 0, max_passengers_per_cabin));
    }

    // Create floors
    let mut etagen = Vec::new();
    for i in 0..num_floors {
        etagen.push(cabin::Etage { id: i });
    }

    // Create controller
    let controller = Arc::new(RwLock::new(Controller::new(
        fahrkabinen.clone(),
        etagen,
    )));

    info!("Main loop");

    // Init current floors if not passed as parameter
    let current_floors = current_floors.unwrap_or_else(|| {
        info!("no current floors set, randomizing");
        (0..num_passengers)
            .map(|_| rand::random::<i32>().abs() % num_floors)
            .collect()
    });

    let destination_floors = destination_floors.unwrap_or_else(|| {
        info!("no destination floors set, randomizing");
        let mut floors = vec![];
        for &current_floor in &current_floors {
            let mut dest_floor = rand::random::<i32>().abs() % num_floors;
            while dest_floor == current_floor {
                dest_floor = rand::random::<i32>().abs() % num_floors;
            }
            floors.push(dest_floor);
        }
        floors
    });

   //// Validate input lengths
   //assert_eq!(
   //    current_floors.len(), 
   //    destination_floors.len(), 
   //    "Current and destination floors must have the same length."
   //);
   //assert_eq!(
   //    current_floors.len(), 
   //    num_passengers,
   //    "Current floors count ({}) must match passenger count ({})",
   //    current_floors.len(),
   //    num_passengers
   //);
   //assert_eq!(
   //    destination_floors.len(), 
   //    num_passengers,
   //    "Destination floors count ({}) must match passenger count ({})",
   //    destination_floors.len(),
   //    num_passengers
   //);

    // Validate floor bounds
    for &floor in &current_floors {
        assert!(
            floor >= 0 && floor < num_floors, 
            "Current floor {} is out of bounds.", 
            floor
        );
    }

    for &floor in &destination_floors {
        assert!(
            floor >= 0 && floor < num_floors, 
            "Destination floor {} is out of bounds.", 
            floor
        );
    }

    // Create passengers
    for (i, (&current_floor, &destination_floor)) in current_floors.iter()
        .zip(destination_floors.iter()).enumerate() 
    {
        let p = Passagier::new(
            i as i32,
            current_floor,
            destination_floor,
            fahrkabinen.clone(),
            controller.clone(),
        );
        controller.write().unwrap_or_else(|e| e.into_inner()).all_passengers.push(p);
    }

    info!("Created all Passengers!");

    // Main simulation loop
    loop {
        // Collect elevator states
        let elevator_states: Vec<_> = fahrkabinen
            .iter()
            .map(|k| Fahrkabine::get_state(k))
            .collect();

        // Collect passenger states
        let passenger_states = controller.read()
            .unwrap_or_else(|e| e.into_inner())
            .get_passenger_states();

        // Send state to UI
        let _ = ui_sender.send(gui::UIState {
            elevators: elevator_states,
            passengers: passenger_states,
        });

        // Check exit condition with poison recovery
        let all_exited = {
            let controller_guard = controller.read()
                .unwrap_or_else(|e| e.into_inner());
            
            controller_guard.all_passengers.iter().all(|p| {
                p.read()
                    .unwrap_or_else(|e| e.into_inner())
                    .state == PassengerState::Exiting
            })
        };

        if all_exited {
            break;
        }

        // Slow down updates
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    info!("All passengers have exited");

    // Send final empty state to GUI
let elevator_states: Vec<_> = fahrkabinen
    .iter()
    .map(|k| Fahrkabine::get_state(k))
    .collect();

let passenger_states = controller.read()
    .unwrap_or_else(|e| e.into_inner())
    .get_passenger_states();

// Send final update
let _ = ui_sender.send(gui::UIState {
    elevators: elevator_states,
    passengers: passenger_states,
});

// Keep GUI visible for 2 more seconds
std::thread::sleep(std::time::Duration::from_secs(2));
}