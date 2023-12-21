mod home;


pub enum Screens {
    Home,
}

pub struct Screen {}

impl Screen {
    pub fn load(screen: Screens) {
        match screen {
            Screens::Home => home::run(),
        };
    }
}
