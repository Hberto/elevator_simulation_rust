use elevator::{gui, run_simulation};
use crossbeam_channel::unbounded;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger to file instead of stdout
    let log_file = std::fs::File::create("elevator.log")?;
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    let (tx, rx) = unbounded();
    let (shutdown_tx, shutdown_rx) = unbounded();

    let ui_handle = std::thread::spawn(move || {
        gui::run_ui(rx).expect("UI should run");
        let _ = shutdown_tx.send(());
    });

    const NUM_ELEVATORS: usize = 3;
    const NUM_FLOORS: i32 = 3;
    const NUM_PASSENGERS: usize = 10;
    const MAX_PASSENGERS_PER_CABIN: usize = 2;

    let sim_handle = std::thread::spawn(move || {
        run_simulation(
            NUM_ELEVATORS,
            NUM_FLOORS,
            NUM_PASSENGERS,
            MAX_PASSENGERS_PER_CABIN,
            Some(vec![0, 1]),
            Some(vec![1, 0]),
            tx.clone(),
        );
    });

    shutdown_rx.recv()?;
    sim_handle.join().unwrap();
    ui_handle.join().unwrap();
    Ok(())
}