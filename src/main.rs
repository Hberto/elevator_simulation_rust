use elevator::*;
use log::info;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // Define constants for the simulation
    const NUM_ELEVATORS: usize = 3;
    const NUM_FLOORS: i32 = 3;
    const NUM_PASSENGERS: usize = 2;

    const MAX_PASSENGERS_PER_CABIN: usize = 2;

    // Run the simulation with the defined constants
    //run_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS, MAX_PASSENGERS_PER_CABIN, Option::from(vec![0, 1]), Option::from(vec![1, 0]));
     info!(
        "Starting simulation: {} elevator, {} floors, {} passenger",
        NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS
    );
    run_simulation(
        NUM_ELEVATORS,   // num_elevators
        NUM_FLOORS,   // num_floors (0-3)
        NUM_PASSENGERS,  // num_passengers
        MAX_PASSENGERS_PER_CABIN,   // max_passengers_per_cabin
        None, // current_floors
        None, // destination_floors
    );

    // Keep main thread alive to allow simulation to run
    std::thread::sleep(std::time::Duration::from_secs(100));
    // run_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS, None, None);
}
