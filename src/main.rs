use elevator::*;

fn main() {
    env_logger::init();
    // Define constants for the simulation
    const NUM_ELEVATORS: usize = 1;
    const NUM_FLOORS: i32 = 2;
    const NUM_PASSENGERS: usize = 1;

    const MAX_PASSENGERS_PER_CABIN: usize = 2;

    // Run the simulation with the defined constants
    run_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS, MAX_PASSENGERS_PER_CABIN, Option::from(vec![0, 1]), Option::from(vec![1, 0]));
    // run_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS, None, None);
}
