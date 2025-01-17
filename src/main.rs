use elevator::*;

fn main() {
    // Define constants for the simulation
    const NUM_ELEVATORS: usize = 3;
    const NUM_FLOORS: i32 = 3;
    const NUM_PASSENGERS: usize = 8;

    // Run the simulation with the defined constants
    run_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS);
}
