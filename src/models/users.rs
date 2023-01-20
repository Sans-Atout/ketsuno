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

#[derive(Serialize, Deserialize)]
pub struct UserCreation {
    pub pseudo: String,
    pub email: String,
    pub password: String,
    pub repeat_password: String,
    pub accept_cgu: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::web_user)]

pub struct NewUser<'a> {
    pub pseudo: &'a String,
    pub email: &'a String,
    pub avatar: &'a String,
    pub password: &'a String,
    pub is_activated: &'a bool,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
}

impl UserCreation {
    pub fn validate(&self) -> Vec<Error> {
        let mut errors: Vec<Error> = Vec::new();
        if !validate_email(&self.email) {
            errors.push(get_form_errors(
                "email".to_string(),
                FormErrors::NotAnEmailAdress,
            ));
        }

        let valid_user = validate_username(&self.pseudo);
        let valid_password = validate_password(&self.password, &self.repeat_password);
        if valid_user == 1 {
            errors.push(get_form_errors(
                "pseudo".to_string(),
                FormErrors::StringToShort,
            ))
        }
        if valid_user == 2 {
            errors.push(get_form_errors(
                "pseudo".to_string(),
                FormErrors::StringToLong,
            ))
        }
        if valid_password == 1 {
            errors.push(get_form_errors(
                "email".to_string(),
                FormErrors::NotAnEmailAdress,
            ))
        }
        if valid_password == 2 {
            errors.push(get_form_errors(
                "email".to_string(),
                FormErrors::NotAnEmailAdress,
            ))
        }
        return errors;
    }
}
