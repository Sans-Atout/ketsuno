use email_address::EmailAddress;

pub fn validate_email(email: &str) -> bool {
    if !EmailAddress::is_valid(email.clone()){
        return false;
    }
    //TODO implement ya-chan database verification
    return true;
}

pub fn validate_username(pseudo: &str) -> u8{
    if pseudo.len() < 4 {
        return 1;
    }
    if pseudo.len() > 100{
        return 2;
    }
    return 0;
}

pub fn validate_password(password_1: &str,password_2: &str) -> u8{
    if password_1.len() < 8 {
        return 1;
    }

    if password_1 != password_2 {
        return 2;
    }

    return 0;
}