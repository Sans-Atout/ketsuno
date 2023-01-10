use serde::{Serialize,Deserialize};
use diesel::prelude::*;

#[derive(Serialize,Deserialize, Queryable)]
pub struct User{
    pub name : String, 
    pub email : String,

}