use anyhow::anyhow;
use base64::{prelude::BASE64_URL_SAFE, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

type Username = String;

#[derive(Default)]
pub struct UserData {
	users: HashMap<Username, UserEntry>,
}

impl UserData {
	const TOKEN_LENGTH: usize = 64;

	pub fn find_user(&self, name: &str) -> Option<&UserEntry> {
		self.users.get(name)
	}

	pub fn register(&mut self, name: &str, password: &str) -> anyhow::Result<()> {
		let None = self.users.get(name) else {
			return Err(anyhow!("username already taken"));
		};

		let user = {
			let password_entry = StoredPassword::new(password)?;

			UserEntry {
				password: password_entry,
				token: None,
			}
		};
		self.users.insert(name.to_owned(), user);

		Ok(())
	}

	pub fn authenticate(&mut self, name: &str, password: &str) -> anyhow::Result<UserToken> {
		if let Some(user) = self.users.get_mut(name) {
			if user.password.is(password) {
				let user_token = UserToken {
					value: {
						let mut token: [u8; Self::TOKEN_LENGTH] = [0; Self::TOKEN_LENGTH];
						rand::thread_rng().fill(&mut token);
						BASE64_URL_SAFE.encode(token)
					},
				};
				user.token = Some(user_token.clone());

				return Ok(user_token);
			}
		}
		Err(anyhow!("incorrect username or password"))
	}

	pub fn known_by_token(&self, name: &str, token: &str) -> bool {
		let Some(user) = self.find_user(name) else {
			return false;
		};
		let Some(ref known_token) = user.token else {
			return false;
		};

		token == known_token.value
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserToken {
	value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserEntry {
	password: StoredPassword,
	token: Option<UserToken>,
}

#[derive(Clone, Debug, PartialEq)]
struct StoredPassword {
	password_hash: String,
	salt: String,
}

impl StoredPassword {
	const SALT_LEN: usize = 16;

	const MIN_PASSWD_LEN: usize = 5;
	const MAX_PASSWD_LEN: usize = 64;

	fn new(password: &str) -> anyhow::Result<Self> {
		match password.len() {
			0..Self::MIN_PASSWD_LEN => {
				return Err(anyhow!(
					"password must be at least {} characters long",
					Self::MIN_PASSWD_LEN
				));
			}
			Self::MAX_PASSWD_LEN.. => {
				return Err(anyhow!(
					"password must be at most {} characters long",
					Self::MAX_PASSWD_LEN
				));
			}
			_ => {}
		}

		let salt: String = {
			let salt_bytes: [u8; Self::SALT_LEN] = rand::thread_rng().gen();
			BASE64_URL_SAFE.encode(salt_bytes)
		};

		let password_hash = {
			let mut hasher = Sha256::new();
			hasher.update(format!("{}{}", password, salt));
			let result = hasher.finalize();
			BASE64_URL_SAFE.encode(result)
		};

		Ok(StoredPassword {
			password_hash,
			salt,
		})
	}

	fn is(&self, password: &str) -> bool {
		let mut hasher = Sha256::new();
		hasher.update(format!("{}{}", password, self.salt));
		let result = hasher.finalize();

		self.password_hash == BASE64_URL_SAFE.encode(result)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn store_passwords() {
		let entry = StoredPassword::new("secure password").unwrap();
		let second_entry = StoredPassword::new("secure password").unwrap();

		assert_ne!(entry, second_entry);

		dbg!(entry);
	}

	#[test]
	fn identify_same_password() {
		let entry = StoredPassword::new("secure password").unwrap();
		assert!(entry.is("secure password"));
	}

	#[test]
	fn invalid_len() {
		let entry = StoredPassword::new("shrt");
		assert!(entry.is_err());

		let second_entry = StoredPassword::new(
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
            Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
		);
		assert!(second_entry.is_err())
	}

	#[test]
	fn correct_authentification() {
		let mut userdata = UserData::default();

		let _ = userdata.register("alice", "12345");

		let user = userdata.find_user("alice");
		assert!(user.is_some());

		let received_token = userdata.authenticate("alice", "12345");

		assert!(received_token.is_ok());
		let received_token = received_token.unwrap();

		assert!(userdata.known_by_token("alice", &received_token.value));
	}

	#[test]
	fn error_on_wrong_password() {
		let mut userdata = UserData::default();

		let _ = userdata.register("alice", "12345");

		let received_token = userdata.authenticate("alice", "54321");
		let received_token_2 = userdata.authenticate("bob", "12345");

		assert!(received_token.is_err());
		assert!(received_token_2.is_err());

		assert_eq!(
			received_token.unwrap_err().to_string(),
			received_token_2.unwrap_err().to_string()
		);
	}
}
