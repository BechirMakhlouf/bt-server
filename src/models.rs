#![allow(dead_code)]

/// Email Type
#[derive(Debug)]
pub struct Email(String);

#[derive(Debug)]
pub struct Password(String);

#[derive(Debug)]
pub struct HashedPassword(String);

#[derive(Debug)]
pub struct Id(String);

impl Email {
    fn parse(email: &str) -> Result<Self, &str> {
        Ok(Self(email.into()))
    }
}

impl Password {
    fn parse(password: &str) -> Result<Self, &str> {
        Ok(Self(password.into()))
    }
}

impl HashedPassword {
    fn parse(password: &str) -> Result<Self, &str> {
        Ok(Self(password.into()))
    }
}

#[derive(Debug)]
pub struct NewUser {
    email: Email,
    password: Password,
}

impl NewUser {
    pub fn new(email: &str, password: &str) -> Result<Self, String> {
        Ok(NewUser {
            email: Email::parse(email)?,
            password: Password::parse(password)?,
        })
    }
}

struct RegisteredUser {
    email: Email,
    password: HashedPassword,
    id: Id,
}
