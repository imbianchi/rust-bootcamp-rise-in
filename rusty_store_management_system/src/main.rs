mod helpers;
mod screens;

use screens::{Screen, Screens};

fn main() {
    Screen::load(Screens::Home);
}
