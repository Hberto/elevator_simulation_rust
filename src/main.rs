use elevator::*;

fn main() {
    env_logger::init();
    // Define constants for the simulation
    const NUM_ELEVATORS: usize = 3;
    const NUM_FLOORS: i32 = 3;
    const NUM_PASSENGERS: usize = 2;

    // Run the simulation with the defined constants
    // run_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS, Option::from(vec![1, 1]), Option::from(vec![2, 2]));
    run_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS, None, None);
}
