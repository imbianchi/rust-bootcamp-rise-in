mod screens;
use screens::{Screen, Screens};
fn main() {
    Screen::new()
        .unwrap()
        .run_screen(Screens::Welcome);
}
