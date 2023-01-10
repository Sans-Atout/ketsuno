use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct App {
    name: String,
    version : String,
    status : String
}

impl App {
    pub fn new(name : &str, version : &str, stat: &str) -> App{
        return App{
            name : name.to_owned().to_string(),
            version : version.to_owned().to_string(),
            status : stat.to_owned().to_string()
        };
    }
}