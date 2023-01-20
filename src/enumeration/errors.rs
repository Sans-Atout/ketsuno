use crate::errors;

pub enum FormErrors{
    NotAnEmailAdress=0,
    EmailDidntExist=1,
    StringToShort=2,
    StringToLong=3,
    PasswordMissMatch=4,
    EmailAllreadyUsed=5, 
    CGUNotAccepted=6,
}