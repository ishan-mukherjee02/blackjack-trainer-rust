use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, Paragraph}, Terminal};
use std::{error::Error, io, time::Duration};
mod blackjack; // Import game logic module
use ratatui::prelude::Backend;
use ratatui::Frame;

struct BlackjackUI {
    bj: blackjack::Blackjack,
}

impl BlackjackUI {
    fn new() -> Self {
        Self {
            bj: blackjack::Blackjack::new(),
        }
    }

    // Plays a single hand of blackjack
    pub fn play_hand(&mut self) {
        self.bj.deal_cards();
        self.play_players_hand();
        self.bj.play_dealers_hand();
        self.display_result();
    }

    // Plays blackjack hands until the user chooses to quit
    pub fn play_hands_until_quit(&mut self) {
        let mut input = String::new();

        loop {
            self.play_hand();
            println!("Keep playing? (yes/no): ");
            // io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() != "yes" {
                break;
            }
        }

        println!("Thanks for playing");
    }

    // Allows the player to hit until it is no longer possible or until the player chooses to stand
    fn play_players_hand(&mut self) {
        println!("You have: {}", self.bj.get_players_hand().expect("nothing").to_string());
        println!("Dealer has: {}", self.bj.get_dealers_hand().expect("nothing").to_string());

        let mut response = String::new();

        let bj = &mut self.bj;

        while bj.can_hit() {
            println!("Do you want to hit or stand?");
            // io::stdout().flush().unwrap();
            response.clear();
            io::stdin().read_line(&mut response).unwrap();
            
            match response.trim().to_lowercase().as_str() {
                "hit" => {
                    bj.hit();
                    if bj.get_players_hand().expect("nothing").get_value() > 21 {
                        println!("You now have: {}", bj.get_players_hand().expect("nothing").to_string());
                        println!("You are bust.");
                        continue;
                    }
                }
                "stand" => break,
                _ => println!("Invalid option, please type 'hit', 'stand', or 'double'."),
            }
    
            println!(
                "You have: {}",
                bj.get_players_hand().expect("nothing").to_string()
            );
        }
    }

    // Displays the result of the hand (push, player win, player blackjack, or loss)
    fn display_result(&mut self) {
        if self.bj.get_dealers_hand().expect("Empty").is_blackjack() && self.bj.get_players_hand().expect("Empty").is_blackjack() {
            println!("Y'all both got blackjack, it's a push.");
        } else if self.bj.get_players_hand().expect("Empty").is_blackjack() {
            println!("YOU GOT BLACKJACK!");
        } else if self.bj.is_player_win() {
            println!("Player win.");
        } else if self.bj.is_push() {
            println!("Push.");
        } else {
            println!("Player loss.");
        }
    }

    fn draw<B: Backend>(&self, f: &mut Frame) {
        // Set up layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.area());

        // Player's hand display
        let player_hand = self
            .bj
            .get_players_hand()
            .map(|hand| hand.to_string())
            .unwrap_or("No hand".to_string());
        let player_paragraph = Paragraph::new(player_hand)
            .block(Block::default().title("Player's Hand").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Blue));
    
        // Dealer's hand display
        let dealer_hand = self
            .bj
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
            terminal.draw(|f| self.draw::<ratatui::backend::CrosstermBackend<io::Stdout>>(f))?;


            if event::poll(Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => self.bj.hit(),
                        KeyCode::Char('s') => {
                            self.bj.play_dealers_hand();
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
    // Initialize terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ui = BlackjackUI::new();

    // Draw the UI
    terminal.draw(|f| ui.draw::<CrosstermBackend<io::Stdout>>(f))?;
    ui.run_app();
    Ok(())
}
