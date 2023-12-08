use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Error, Stdout},
    thread,
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

pub enum Screens {
    Welcome,
    Login,
    Dashboard,
    CreateUser,
    DeleteUser,
    CrateProduct,
    EditProduct,
    DeleteProduct,
    GetSalesHistory,
    GetPuschaseHistory,
    Report,
    Error,
}

pub struct Screen {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Screen {
    pub fn new() -> Result<Self, Error> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub fn run_screen(&mut self, screen: Screens) -> Result<(), Error> {
        match screen {
            Screens::Welcome => &Self::welcome_screen(self),
            Screens::Login => &Self::login_screen(self),
            _ => unimplemented!(),
        };
    }

    fn welcome_screen(&mut self) -> Result<(), Error> {
        let _ = &self.terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("AJ - Your CLI Store Management")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        self.run_screen(Screens::Login);

        Ok(())
    }

    fn login_screen(&mut self) {
        unimplemented!()
    }

    fn close_terminal(&mut self) -> Result<(), Error>{
        // restore terminal
        disable_raw_mode()?;
        execute!(
            &mut self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        let _ = &mut self.terminal.show_cursor()?;

        Ok(())
    }

    fn error_loading_screen(&self) -> Result<(), Error> {
        Ok(())
    }
}
