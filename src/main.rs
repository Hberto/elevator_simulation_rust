use elevator::*;
use std::sync::RwLock;
use std::sync::Arc;
use log::{info,warn};
use std::time::{Duration, Instant};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    const TIMEOUT: Duration = Duration::from_secs(5);
    let start_time = Instant::now();
    const NUM_ELEVATORS: usize = 1;
    const NUM_FLOORS: i32 = 2;
    const NUM_PASSENGERS: usize = 1;
    const MAX_PASSENGERS_PER_CABIN: usize = 2;

    let elevators: Vec<_> = (0..NUM_ELEVATORS)
        .map(|i| Fahrkabine::new(i as i32, 0, MAX_PASSENGERS_PER_CABIN))
        .collect();

    let controller = Controller::new(elevators.clone());

    // Create passengers
    for i in 0..NUM_PASSENGERS {
        let passenger = Passagier::new(i as i32, 0, 1, controller.clone());
        passenger.write().unwrap().request_elevator();
        controller.write().unwrap().add_passenger(passenger);
    }

    // Simulation loop
    loop {
        let all_exited = controller.read().unwrap().passengers.iter().all(|p| {
            p.read().unwrap().state == PassengerState::Exiting
        });
        if all_exited {
            break;
        }
        // Add small sleep to prevent tight loop
        std::thread::sleep(Duration::from_millis(100));
    }
    info!("Simulation complete.");
}