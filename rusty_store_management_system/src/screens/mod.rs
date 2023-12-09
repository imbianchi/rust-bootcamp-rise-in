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
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Widget},
    Terminal,
};

use crate::helpers::Helpers;

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
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub fn run_screen(&mut self, screen: Screens) {
        match screen {
            Screens::Welcome => &Self::welcome_screen(self).unwrap(),
            Screens::Login => &Self::login_screen(self).unwrap(),
            Screens::Dashboard => &Self::dashboard_screen(self).unwrap(),
            _ => unimplemented!(),
        };
    }

    fn wireframe_screen(&mut self, text: &str) {
        let _ = &mut self.terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(text)
                .borders(Borders::ALL);
            f.render_widget(block, Rect {
                height: size.height - 5,
                width: size.width,
                x: size.x,
                y: size.y,
            });
        }).expect("Error loading wireframe.");
    }

    fn welcome_screen(&mut self) -> Result<(), Error> {
        self.wireframe_screen("# JP CLI Store Management #");

        thread::sleep(Duration::from_millis(5000));

        self.terminal.clear().unwrap();

        let (helpers, config_exists) = Helpers::config_file_exists();

        if config_exists {
            if let Some(value) = helpers.get_credentials() {
                return Ok(self.run_screen(Screens::Dashboard));
            }

            Ok(self.run_screen(Screens::Login))
        } else {
            helpers.create_config_file();
            Ok(self.run_screen(Screens::Login))
        }
    }

    fn dashboard_screen(&mut self) -> Result<(), Error> {
        Ok(self.wireframe_screen("DASHBOARD"))
    }

    fn login_screen(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn close_terminal(&mut self) -> Result<(), Error> {
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
