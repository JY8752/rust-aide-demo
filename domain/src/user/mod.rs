pub mod error;

use derive_more::{From, Into};
use email_address::EmailAddress;
use error::Error;
use uuid::Uuid;

#[derive(From, Debug, Into)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: Uuid) -> Result<Self, Error> {
        if id.is_nil() {
            return Err(Error::InvalidUserId("User ID cannot be empty"));
        }
        Ok(Self(id))
    }
}

#[derive(Debug, From, Into)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: impl Into<String>) -> Result<Self, Error> {
        let name = name.into();
        if name.is_empty() {
            return Err(Error::InvalidUserName("User name cannot be empty"));
        }
        if name.len() > 100 {
            return Err(Error::InvalidUserName(
                "User name cannot be longer than 100 characters",
            ));
        }
        Ok(Self(name))
    }
}

#[derive(From, Debug, Into)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(email: impl Into<String>) -> Result<Self, Error> {
        let email = email.into();
        if email.is_empty() {
            return Err(Error::InvalidUserEmail("User email cannot be empty"));
        }
        if !EmailAddress::is_valid(&email) {
            return Err(Error::InvalidUserEmail("User email format is invalid"));
        }
        Ok(Self(email))
    }
}

pub struct User {
    id: UserId,
    name: UserName,
    email: UserEmail,
}

impl User {
    pub fn new(name: &str, email: &str) -> Result<Self, Error> {
        Ok(Self {
            id: UserId::new(Uuid::now_v7())?,
            name: UserName::new(name)?,
            email: UserEmail::new(email)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn assert_user(user: Result<User, Error>, name: &str, email: &str) {
        assert!(user.is_ok());
        let user = user.unwrap();

        let user_id: Uuid = user.id.into();
        let user_name: String = user.name.into();
        let user_email: String = user.email.into();

        assert!(!user_id.is_nil());
        assert_eq!(user_name, name);
        assert_eq!(user_email, email);
    }

    #[test]
    fn ユーザーを1つ作成できること() {
        let user = User::new("Alice", "alice@example.com");
        assert_user(user, "Alice", "alice@example.com");
    }

    #[rstest]
    #[case("")]
    #[case("alice")]
    #[case("alice@")]
    #[case("@example.com")]
    #[case("alice@@example.com")]
    #[case("alice @example.com")]
    fn ユーザーメール形式が不正ならエラーになる(#[case] email: &str) {
        let user = User::new("Alice", email);
        assert!(matches!(user, Err(Error::InvalidUserEmail(_))));
    }

    #[test]
    fn ユーザー名が100文字ではエラーにならない() {
        let user = User::new(&"a".repeat(100).to_string(), "alice@example.com");
        assert!(user.is_ok());
    }

    #[rstest]
    #[case("")]
    #[case("a".repeat(101))]
    fn ユーザー名が不正ならエラーになる(#[case] name: String) {
        let user = User::new(&name, "alice@example.com");
        assert!(matches!(user, Err(Error::InvalidUserName(_))));
    }
}
