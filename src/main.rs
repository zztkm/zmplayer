use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::*,
    widgets::Paragraph,
};
use std::io;

// Define a type alias for the terminal backend
type TerminalBackend = CrosstermBackend<io::Stdout>;

fn main() -> Result<()> {
    // Initialize color-eyre for better error reporting
    color_eyre::install()?;

    // Initialize the terminal
    let mut terminal = init_terminal()?;

    // Run the application logic
    let app_result = run_app(&mut terminal);

    // Restore the terminal to its original state
    restore_terminal()?;

    // Return the application result
    app_result
}

// Initialize the terminal
fn init_terminal() -> Result<Terminal<TerminalBackend>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

// Restore the terminal
fn restore_terminal() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

// Main application logic
fn run_app(terminal: &mut Terminal<TerminalBackend>) -> Result<()> {
    loop {
        // Draw the UI
        terminal.draw(ui)?;

        // Check for user input
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Exit if 'q' is pressed
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}

// UI rendering logic
fn ui(frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new("Hello World from Ratatui!\nPress 'q' to quit.")
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true }),
        main_layout[0],
    );
}
