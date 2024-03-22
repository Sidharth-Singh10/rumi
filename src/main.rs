use audio::Controls;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use std::io;
mod audio;
mod errors;
mod tui;
use color_eyre::{eyre::WrapErr, Result};

// #[derive(Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct App {
    current: usize,
    exit: bool,
    controls: Controls,
    items: StatefulList<String>,
}

impl App {
    pub fn new(controls: Controls) -> App {
        let songlist = controls.songlist.clone();
        App {
            current: 0,
            exit: false,
            controls,
            items: StatefulList::with_items(songlist),
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui(frame, self))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
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

    // control audio through here
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(), //
            KeyCode::Char('p') => self.play_pause(),
            KeyCode::Char('n') => self.next(),
            KeyCode::Char('m') => self.previous(),
            KeyCode::Char('s') => self.start(),
            // KeyCode::Left | KeyCode::Char('h') => self.app_list.items.unselect(),
            KeyCode::Down | KeyCode::Char('j') => self.items.next(),
            // KeyCode::Up | KeyCode::Char('k') => self.app_list.items.previous(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn play_pause(&self) {
        if self.controls.sink.is_paused() {
            self.controls.sink.play();
        } else {
            self.controls.sink.pause();
        }
    }
    fn next(&mut self) {
        self.current += 1;
        if self.current >= self.controls.playlistz.len() {
            self.current = 0;
        }
        self.start();
    }
    fn previous(&mut self) {
        if self.current == 0 {
            self.current = self.controls.playlistz.len() - 1;
        } else {
            self.current -= 1;
        }

        self.start();
    }

    fn start(&self) {
        let path = self.controls.playlistz[self.current].clone();
        let source = Controls::get_source(path);
        self.controls.sink.clear();
        self.controls.sink.append(source);
        self.controls.sink.play();
    }
    
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let title = Paragraph::new(" RUMI: A rusty music player  ".bold().red()).block(title_block);

    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app
        .items
        .items
        .clone()
        .into_iter()
        .map(|i| ListItem::new(i).style(Style::default().fg(Color::Black).bg(Color::White)))
        .collect();

    let items_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items_list, chunks[1], &mut app.items.state)
}

fn main() -> Result<()> {
    errors::install_hooks()?;
    let controls = Controls::new();
    // let app_list = list::App_list::new(controls.songlist.clone());
    let mut terminal = tui::init()?;
    let app_result = App::new(controls).run(&mut terminal);
    tui::restore()?;
    app_result
}
