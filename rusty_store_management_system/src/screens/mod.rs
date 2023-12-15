use std::io::Error;


pub enum Screens {
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

pub struct Screen {}

impl Screen {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn run_screen(&mut self, screen: Screens) {
        match screen {
            Screens::Login => &Self::login_screen(self).unwrap(),
            Screens::Dashboard => &Self::dashboard_screen(self).unwrap(),
            _ => unimplemented!(),
        };
    }

    fn dashboard_screen(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn login_screen(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn close_terminal(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn error_loading_screen(&self) -> Result<(), Error> {
        Ok(())
    }
}
