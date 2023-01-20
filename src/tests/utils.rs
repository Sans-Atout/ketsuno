#[cfg(test)]
mod test {
    use crate::utils::get_hashed_password;

    #[test]
    fn test_hashed_password() {
        let hash = get_hashed_password("MySuperSecretPassword".to_string());
        println!("{}",hash);
        let matches = argon2::verify_encoded(&hash,  b"MySuperSecretPassword").unwrap();
        assert!(matches);
    }
}
