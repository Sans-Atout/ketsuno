use serde::{Serialize,Deserialize};

pub mod string;

#[derive(Serialize,Deserialize)]
pub struct Error{
    pub field : String,
    pub error_message : ErrorMessage
}

#[derive(Serialize,Deserialize)]
pub struct ErrorMessage{
    pub french : String,
    pub english : String
}