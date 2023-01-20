use std::time::SystemTime;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enumeration::errors::FormErrors;
use crate::errors::string::get_form_errors;
use crate::errors::Error;
use crate::validator::users::{validate_email, validate_password, validate_username};

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
    pub email: String,
    pub avatar: String,
    pub password: String,
    pub is_activated: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}