use std::{io, thread, time::Duration};

use audio::Controls;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use dummy::dummy;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
mod dummy;
mod audio;
mod tui;
mod errors;
use color_eyre::{
    eyre:: WrapErr,
    Result,
};

// #[derive(Default)]
pub struct App {
    counter: u8,
    exit: bool,
    controls: Controls
}
impl App {

    pub fn new(controls: Controls) -> App {
        App {
            counter: 0,
            exit: false,
            controls
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;        }
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
    fn handle_key_event(&mut self, key_event: KeyEvent)  {
        
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),  //next song, previous song, play, pause
            KeyCode::Right => self.increment_counter(), //
            KeyCode::Up => self.play(),
            KeyCode::Down => self.pause(),
            // KeyCode::Char('k') => self.start(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn play(&self) {
        self.controls.sink.play();
    }

    fn pause(&self) {
        self.controls.sink.pause();
    }

   
}





/// drawing the UI of rumi /// boilerplate
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            // " Decrement ".into(),
            // "<Left>".blue().bold(),
            // " Increment ".into(),
            // "<Right>".blue().bold(),
            // " Quit ".into(),
            // "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}


fn main() -> Result<()> {
    errors::install_hooks()?;
    let controls = Controls::new();
    let mut terminal = tui::init()?;
    let app_result = App::new(controls).run(&mut terminal);
    // audio::awedio();
    tui::restore()?;
    app_result


}

// fn main(){

//     let controls = Controls::new();
//     controls.sink.play();
//     thread::sleep(Duration::new(5, 0)); 
//     controls.sink.pause();
//     //
//    // dummy();

// }
