use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

mod app;
mod tui;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
pub fn next(&mut self) {
    let i = match self.state.selected() {
        Some(i) => {
            if i >= self.items.len() - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };
    self.state.select(Some(i));
}

pub fn previous(&mut self) {
    let i = match self.state.selected() {
        Some(i) => {
            if i == 0 {
                self.items.len() - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    self.state.select(Some(i));
    }
}

//boilerplate
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('w') {
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('s') {
            }

        }
    }
    Ok(false)
}
//boilerplate END

///todo? pass ui framework to "App"
fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
        //defnine and render MAIN border(s)
        .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("APERTURE ESCAPE"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Controls |  Exit Tab \"Alt + C\" | View Files \"Alt + D\" | "),
        main_layout[2],
    );

    //define main menu components/objects
    let mut state = ListState::default().with_selected(Some(0));
    let items = ["Continue", "New Game", "Exit"];
    let main_menu = List::new(items)
        .block(Block::bordered().title("Main Menu"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol("->")
        .repeat_highlight_symbol(true);

    *state.offset_mut() = 0;
    state.select(Some(0));

    let menupar = Paragraph::new(Text::from("Welcome to Aperture Escape.\nNavigate the Menu using \"W, A, S, D\" or \"Up, Down, Left, Right\""))
        .block(Block::bordered().title("Welcome"))
        .wrap(Wrap { trim: true })
        .scroll((1, 1));

    //split MAIN box into two blocks with constraints
    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(20), Constraint::Percentage(80)],
    )
        //call the split function to MAIN box, render the boxes with components (widgets) added
        .split(main_layout[1]);
    frame.render_stateful_widget(main_menu, inner_layout[0], &mut state);
    frame.render_widget(menupar, inner_layout[1]);
}
