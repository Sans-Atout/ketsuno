use argon2::{self, Config};

pub fn get_hashed_password(raw_password: String) -> String{
    let conf = Config::default();
    let salt = std::env::var("SALT").unwrap();
    let hash = argon2::hash_encoded(raw_password.as_bytes(), salt.as_bytes(), &conf).unwrap();
    return hash;
}
