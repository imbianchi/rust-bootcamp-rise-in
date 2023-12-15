mod screens;
use screens::{Screen, Screens};

mod helpers;

fn main() {
    Screen::new().unwrap().run_screen(Screens::Login);
}
