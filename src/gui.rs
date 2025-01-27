use crate::cabin::DoorState;
use crate::passenger::PassengerState;
use crossbeam_channel::Receiver;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use std::{io, time::Duration};

pub struct UIState {
    pub elevators: Vec<(i32, i32, DoorState, Vec<i32>)>,
    pub passengers: Vec<(i32, PassengerState)>,
}

pub fn run_ui(receiver: Receiver<UIState>) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        // Handle CTRL+C
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('c')
                    && key.modifiers == crossterm::event::KeyModifiers::CONTROL
                {
                    break;
                }
            }
        }

        // Draw UI
        if let Ok(state) = receiver.try_recv() {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(f.size());

                // Elevators Panel
                let elevator_items: Vec<ListItem> = state
                    .elevators
                    .iter()
                    .map(|(id, floor, door_state, passengers)| {
                        let door_icon = match door_state {
                            DoorState::Open => "🟢 OPEN",
                            DoorState::Opening => "🟡 OPENING",
                            DoorState::Closing => "🟠 CLOSING",
                            DoorState::Closed => "🔴 CLOSED",
                        };
                        let passengers_display = if passengers.is_empty() {
                            "Empty".to_string()
                        } else {
                            format!("Passengers: {:?}", passengers)
                        };
                        ListItem::new(format!(
                            "Elevator {} │ Floor {} │ {} │ {}",
                            id, floor, door_icon, passengers_display
                        ))
                    })
                    .collect();

                let elevators = List::new(elevator_items)
                    .block(Block::default().title("Elevators").borders(Borders::ALL));
                f.render_widget(elevators, chunks[0]);

                // Passengers Panel
                let passenger_items: Vec<ListItem> = state
                    .passengers
                    .iter()
                    .map(|(id, state)| {
                        let status = match state {
                            PassengerState::WaitingOnFloor(floor) => format!("🕒 Floor {}", floor),
                            PassengerState::EnteringElevator => "🚪 Entering".to_string(),
                            PassengerState::ChoosingLevel => "📶 Choosing".to_string(),
                            PassengerState::InElevator => "🚶‍♂️ In Elevator".to_string(),
                            PassengerState::Exiting => "🚪 Exiting".to_string(),
                        };
                        ListItem::new(format!("Passenger {}: {}", id, status))
                    })
                    .collect();

                let passengers = List::new(passenger_items)
                    .block(Block::default().title("Passengers").borders(Borders::ALL));
                f.render_widget(passengers, chunks[1]);
            })?;
        }
    }

    // Cleanup
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;

    Ok(())
}
