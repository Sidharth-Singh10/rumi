use audio::Controls;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::io;
mod audio;
mod errors;
mod tui;
use color_eyre::{eyre::WrapErr, Result};

// #[derive(Default)]
pub struct App {
    current: usize,
    exit: bool,
    controls: Controls,
}
impl App {
    pub fn new(controls: Controls) -> App {
        App {
            current: 0,
            exit: false,
            controls,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        // todo!()
        frame.render_widget(self, frame.size());
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

/// drawing the UI of rumi /// boilerplate
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        let title_block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default());
        let title_block2 = Block::default()
            .borders(Borders::LEFT)
            .style(Style::default());

        Paragraph::new(" Counter App Tutorial ".bold())
            .block(title_block)
            .render(chunks[0], buf);

            let mut state = ListState::default();

           let list = List::new(self.controls.songlist.clone())
            .block(Block::default().title("List").borders(Borders::ALL))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true).block(title_block2);

            ratatui::widgets::StatefulWidget::render(list, chunks[1], buf, &mut state);
           


        // Paragraph::new(" Laude ".bold())
        //     .block(title_block2)
        //     .render(chunks[1], buf);

        // let block = Block::default()
        //     .title(title.alignment(Alignment::Center))
        //     .title(
        //         instructions
        //             .alignment(Alignment::Center)
        //             .position(Position::Bottom),
        //     )
        //     .borders(Borders::ALL)
        //     .border_set(border::THICK);

        // let counter_text = Text::from(vec![Line::from(vec![
        //     "Value: ".into(),
        //     self.current.to_string().yellow(),
        // ])]);

        // Paragraph::new(counter_text)
        //     .centered()
        //     .block(block)
        //     .render(area, buf);

    }
}

fn main() -> Result<()> {
    errors::install_hooks()?;
    let controls = Controls::new();
    let mut terminal = tui::init()?;
    let app_result = App::new(controls).run(&mut terminal);
    tui::restore()?;
    app_result
}
