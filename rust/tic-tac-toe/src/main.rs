use bitvec::prelude::*;
use std::io;
use tic_tac_toe::TicTacToe;

mod tic_tac_toe;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    game: TicTacToe,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('0') => self.game.play(0, 0),
            KeyCode::Char('1') => self.game.play(0, 1),
            KeyCode::Char('2') => self.game.play(0, 2),
            KeyCode::Char('3') => self.game.play(1, 0),
            KeyCode::Char('4') => self.game.play(1, 1),
            KeyCode::Char('5') => self.game.play(1, 2),
            KeyCode::Char('6') => self.game.play(2, 0),
            KeyCode::Char('7') => self.game.play(2, 1),
            KeyCode::Char('8') => self.game.play(2, 2),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Tic Tac Toe ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        let game = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.clone().right_aligned())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let state = Block::bordered().title("State");

        Paragraph::new(self.game.to_string())
            .centered()
            .block(game)
            .render(layout[1], buf);

        Paragraph::new(self.game.state().to_string())
            .centered()
            .block(state)
            .render(layout[0], buf);
    }
}
