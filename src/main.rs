use elevator::*;


fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    const NUM_ELEVATORS: usize = 3;
    const NUM_FLOORS: i32 = 3;
    const NUM_PASSENGERS: usize = 3;
    const MAX_PASSENGERS_PER_CABIN: usize = 2;

    run_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS, MAX_PASSENGERS_PER_CABIN);
}