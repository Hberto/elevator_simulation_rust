use elevator::passenger::{Passagier, PassengerState};
use elevator::*;

// example test.
// if we want to test in a meaningful way we need to be able to check state in between
// which the current implementation makes pretty difficult
// so that would be a possible TODO
#[test]
fn test_all_passengers_exited() {
    const NUM_ELEVATORS: usize = 1;
    const NUM_FLOORS: i32 = 2;
    const NUM_PASSENGERS: usize = 1;
    const MAX_PASSENGERS_PER_CABIN: usize = 2;

    let sim_controller = init_simulation(NUM_ELEVATORS, NUM_FLOORS, NUM_PASSENGERS, MAX_PASSENGERS_PER_CABIN, Option::from(vec![0, 1]), Option::from(vec![1, 0]));
    run_simulation(sim_controller.clone());

    // all passenger should be exited after running simulation
    // TODO better state getter would be nice
    sim_controller.read().unwrap().all_passengers.iter().map(|p| {
        Passagier::get_state(p)
    }).for_each(|p| {
        assert_eq!(p.3, PassengerState::Exiting, "passenger not exited");
    });

}