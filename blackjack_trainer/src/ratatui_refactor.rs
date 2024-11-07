use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, Paragraph}, Terminal};
use std::{error::Error, io, time::Duration};
mod blackjack; // Import your game logic module

struct BlackjackUI {
    game: blackjack::Blackjack,
}

impl BlackjackUI {
    fn new() -> Self {
        Self {
            game: blackjack::Blackjack::new(),
        }
    }

    fn draw<B: ratatui::backend::Backend>(&self, f: &mut ratatui::Frame<B>) {
        // Set up layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        // Player's hand display
        let player_hand = self
            .game
            .get_players_hand()
            .map(|hand| hand.to_string())
            .unwrap_or("No hand".to_string());
        let player_paragraph = Paragraph::new(player_hand)
            .block(Block::default().title("Player's Hand").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Blue));

        // Dealer's hand display
        let dealer_hand = self
            .game
            .get_dealers_hand()
            .map(|hand| hand.to_string())
            .unwrap_or("No hand".to_string());
        let dealer_paragraph = Paragraph::new(dealer_hand)
            .block(Block::default().title("Dealer's Hand").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Red));

        // Render the two blocks in chunks
        f.render_widget(player_paragraph, chunks[0]);
        f.render_widget(dealer_paragraph, chunks[1]);
    }

    fn run_app(&mut self) -> Result<(), Box<dyn Error>> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        loop {
            terminal.draw(|f| self.draw(f))?;

            if event::poll(Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => self.game.hit(),
                        KeyCode::Char('s') => {
                            self.game.play_dealers_hand();
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        terminal.clear()?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut ui = BlackjackUI::new();
    ui.run_app()
}
