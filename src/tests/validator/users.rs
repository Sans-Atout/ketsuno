
#[cfg(test)]
mod test{
    use crate::validator::users::{validate_email, validate_username, validate_password};

    #[test]
    fn email_test(){
        assert_eq!(validate_email("trytobypass"), false) ;
        assert!(validate_email("contact@rustfoundation.org"));
    }

    #[test]
    fn pseudo_test(){
        assert_eq!(validate_username("32"),1);
        assert_eq!(validate_username("my beautifull pseudo"),0);
        assert_eq!(2, validate_username("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"));
    }

    #[test]
    fn password_test(){
        assert_eq!(validate_password("32","32"),1);
        assert_eq!(validate_password("supersecret","supersecret"),0);
        assert_eq!(validate_password("supersecret","mysupersecret"),2);
    }
}

