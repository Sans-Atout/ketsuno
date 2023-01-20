use super::Error;
use super::ErrorMessage;

use crate::enumeration::errors::FormErrors;

pub fn get_form_errors(fields: String, errors: FormErrors) -> Error {
    let mut message: ErrorMessage;
    match errors {
        FormErrors::NotAnEmailAdress => {
            message = ErrorMessage {
                french: "L'email donné n'est pas valide".to_string(),
                english: "The given email is not valid".to_string(),
            };
        }
        FormErrors::EmailDidntExist => {
            message = ErrorMessage {
                french: "L'email donné n'existe pas".to_string(),
                english: "The given email did not exist".to_string(),
            };
        }
        FormErrors::StringToShort => {
            message = ErrorMessage {
                french: "La string donné est trop courte".to_string(),
                english: "The given string is too short".to_string(),
            };
        }
        FormErrors::StringToLong => {
            message = ErrorMessage {
                french: "La string donné est trop long".to_string(),
                english: "The given string is too long".to_string(),
            };
        }
        FormErrors::PasswordMissMatch => {
            message = ErrorMessage {
                french: "Les mots de passe données ne correspondent pas".to_string(),
                english: "Both password did not match".to_string(),
            };
        }
        FormErrors::EmailAllreadyUsed => {
            message = ErrorMessage {
                french: "L'email donné est déjà utilisé".to_string(),
                english: "The given email did not exist".to_string(),
            };
        }
        FormErrors::CGUNotAccepted => {
            message = ErrorMessage {
                french: "Les conditions d'utilisations n'ont pas été accepté".to_string(),
                english: "The GCU have not been accepted".to_string(),
            };
        }
    };
    return Error {
        field: fields,
        error_message: message,
    };
}
