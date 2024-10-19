use std::{collections::HashMap, time::SystemTime};

use anyhow::anyhow;
use base64::{prelude::BASE64_URL_SAFE, Engine};
use log::{info, warn};
use rand::Rng;
use sha2::{Digest, Sha256};
use time::Duration;

type Username = String;

#[derive(Default)]
pub struct UserData {
	users: HashMap<Username, UserEntry>,
}

impl UserData {
	const SESSION_ID_LENGTH: usize = 64;
	const SESSION_DURATION: Duration = Duration::hours(72);

	pub fn register(&mut self, name: &str, password: &str) -> anyhow::Result<()> {
		let None = self.users.get(name) else {
			return Err(anyhow!("username already taken"));
		};
		self.users.insert(
			name.to_owned(),
			UserEntry {
				password: StoredPassword::store(password)?,
				session_id: None,
			},
		);

		info!("Registered new user: {name}");

		Ok(())
	}

	pub fn authenticate(&mut self, name: &str, password: &str) -> anyhow::Result<Session> {
		let error = || {
			warn!("User failed to authenticate: {name}");
			anyhow!("incorrect username or password")
		};

		let Some(user) = self.users.get_mut(name) else {
			return Err(error());
		};

		if !user.password.is(password) {
			return Err(error());
		}

		let id = {
			let mut id_bytes = [0_u8; Self::SESSION_ID_LENGTH];
			rand::thread_rng().fill(&mut id_bytes);
			BASE64_URL_SAFE.encode(id_bytes)
		};

		let session = Session {
			value: id,
			expires: SystemTime::now() + Self::SESSION_DURATION,
		};

		user.session_id = Some(session.clone());
		info!("Session issued to user: {name}");

		Ok(session)
	}

	pub fn has_valid_session(&mut self, name: &str, checked_session: &str) -> bool {
		let Some(user) = self.users.get_mut(name) else {
			return false;
		};

		let Some(ref known_session) = user.session_id else {
			return false;
		};

		if known_session.expired() {
			user.session_id = None;
			warn!("Session expired for user: {name}");
			return false;
		}

		checked_session == known_session.value
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
	pub value: String,
	pub expires: SystemTime,
}

impl Session {
	pub fn expired(&self) -> bool {
		self.expires < SystemTime::now()
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserEntry {
	password: StoredPassword,
	session_id: Option<Session>,
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

	fn store(password: &str) -> anyhow::Result<Self> {
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
			hasher.update(format!("{password}{salt}"));
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
		let entry = StoredPassword::store("secure password").unwrap();
		let second_entry = StoredPassword::store("secure password").unwrap();

		assert_ne!(entry, second_entry);

		dbg!(entry);
	}

	#[test]
	fn identify_same_password() {
		let entry = StoredPassword::store("secure password").unwrap();
		assert!(entry.is("secure password"));
	}

	#[test]
	fn invalid_len() {
		let entry = StoredPassword::store("shrt");
		assert!(entry.is_err());

		let second_entry = StoredPassword::store(
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
			Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
		);
		assert!(second_entry.is_err());
	}

	#[test]
	fn correct_authentification() {
		let mut userdata = UserData::default();

		let _ = userdata.register("alice", "12345");

		let user = userdata.users.get("alice");
		assert!(user.is_some());

		let received_token = userdata.authenticate("alice", "12345");

		assert!(received_token.is_ok());
		let received_token = received_token.unwrap();

		assert!(userdata.has_valid_session("alice", &received_token.value));
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
