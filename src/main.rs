use audio::Controls;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// use image::{codecs::png::FilterType, imageops::crop};
use pictures::{display_image, extract_image, get_artist_and_album};

use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use std::io;
// use std::fs;
use ratatui_image::{protocol::StatefulProtocol, StatefulImage};
mod audio;
mod errors;
mod pictures;
mod tui;
use color_eyre::{eyre::WrapErr, Result};

// #[derive(Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    current: usize,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            current: 0,
        }
    }

    pub fn select_item(&mut self) {
        self.state.select(Some(self.current));
    }

    // pub fn unselect(&mut self) {
    //     self.state.select(None);
    // }
}

pub struct App {
    exit: bool,
    controls: Controls,
    items: StatefulList<String>,
    dis: Box<dyn StatefulProtocol>,
    artist: String,
    album: String,
}

impl App {
    pub fn new(controls: Controls) -> App {
        let songlist = controls.songlist.clone();
        App {
            exit: false,
            controls,
            items: StatefulList::with_items(songlist),
            dis: display_image("img/Titli.png".to_string()),
            artist: "Unknown".to_string(),
            album: "Unknown".to_string(),
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
            KeyCode::Char('n') => {
                self.next();
                self.items.select_item()
            }
            KeyCode::Char('m') => {
                self.previous();
                self.items.select_item()
            }
            KeyCode::Char('s') => {
                self.previous();
                self.items.select_item()
            }
            // KeyCode::Left | KeyCode::Char('h') => self.app_list.items.unselect(),
            // KeyCode::Down | KeyCode::Char('j') => self.items.next(),
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
        self.items.current += 1;
        if self.items.current >= self.controls.playlistz.len() {
            self.items.current = 0;
        }
        self.start();
    }
    fn previous(&mut self) {
        if self.items.current == 0 {
            self.items.current = self.controls.playlistz.len() - 1;
        } else {
            self.items.current -= 1;
        }

        self.start();
    }

    fn start(&mut self) {
        let path = self.controls.playlistz[self.items.current].clone();
        let source = Controls::get_source(path);
        self.controls.sink.clear();
        self.controls.sink.append(source);
        self.controls.sink.play();
        self.metadata();
    }

    fn metadata(&mut self) {
        //display image ka nautanki
        let input_path = format!(
            "music/{}",
            self.controls.songlist[self.items.current].clone()
        );
        let output_path = format!("img/{}", self.controls.songlist[self.items.current].clone());

        let out_to_image = output_path.replace(".mp3", ".png");

        let _ = extract_image(input_path.clone(), out_to_image.clone());

        self.dis = display_image(out_to_image);






        let (artist, album) = get_artist_and_album(input_path).unwrap();

        if let Some(artist) = artist {
            self.artist = artist;
        } else {
            self.artist = "Unknown".to_string();
        }

        if let Some(album) = album {
            self.album = album;
        } else {
            self.album = "Unknown".to_string();
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(f.size());

    let left_block = Block::default()
        .borders(Borders::RIGHT)
        .style(Style::default());

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let minichunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(chunks[0]);

    let title = Paragraph::new("RUMI: A rusty music player".bold().red())
        .block(title_block)
        .alignment(Alignment::Center);

    let image = StatefulImage::new(None);

    f.render_widget(left_block, chunks[0]);
    f.render_widget(title, minichunks[0]);

    let info_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(minichunks[1]);

    let image_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(info_chunk[0]);

    let author_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    // let author = Paragraph::new("Artist: Bulbul".bold().white()).block(author_block).alignment(Alignment::Left);

    let authorf = format!("Author: {}", app.artist);
    let albumf = format!("Album: {}", app.album);
    let text = vec![authorf.white().bold().into(), albumf.white().bold().into()];

    let meta = Paragraph::new(text)
        .block(author_block)
        .alignment(Alignment::Left).wrap(Wrap { trim: true });

    f.render_widget(meta, image_chunk[1]);

    f.render_stateful_widget(image, image_chunk[0], &mut app.dis);

    let items: Vec<ListItem> = app
        .items
        .items
        .clone()
        .into_iter()
        .map(|i| ListItem::new(i).style(Style::default().fg(Color::Black).bg(Color::White)))
        .collect();

    let items_list = List::new(items)
        .block(Block::default().borders(Borders::TOP).title("PlayList"))
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
