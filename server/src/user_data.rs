use std::{collections::HashMap, time::SystemTime};

use anyhow::anyhow;
use base64::{prelude::BASE64_URL_SAFE, Engine};
use log::{info, warn};
use rand::Rng;
use sha2::{Digest, Sha256};
use time::Duration;

type Username = String;
type PrivateSessionId = String;

#[derive(Default)]
pub struct UserData {
	users: HashMap<Username, UserEntry>,
}

impl UserData {
	pub fn register(&mut self, username: &str, password: &str) -> anyhow::Result<()> {
		let None = self.users.get(username) else {
			return Err(anyhow!("username already taken"));
		};
		self.users.insert(
			username.to_owned(),
			UserEntry {
				password: StoredPassword::store(password)?,
				session: None,
			},
		);

		info!("Registered new user: {username}");

		Ok(())
	}

	pub fn authenticate(
		&mut self,
		username: &str,
		password: &str,
	) -> anyhow::Result<PrivateSessionId> {
		let error = || {
			warn!("User failed to authenticate: {username}");
			anyhow!("incorrect username or password")
		};

		let Some(user) = self.users.get_mut(username) else {
			return Err(error());
		};

		if !user.password.is(password) {
			return Err(error());
		}

		let (id, session) = Session::new();

		user.session = Some(session.clone());
		info!("Session issued to user: {username}");

		Ok(id)
	}

	pub fn has_valid_session(&mut self, username: &str, checked_session: &str) -> bool {
		let Some(user) = self.users.get_mut(username) else {
			return false;
		};

		let Some(ref known_session) = user.session else {
			return false;
		};

		if known_session.expired() {
			user.session = None;
			warn!("Session expired for user: {username}");
			return false;
		}

		known_session.is(checked_session)
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
	stored: StoredPassword,
	expires: SystemTime,
}

impl Session {
	const SESSION_ID_LENGTH: usize = 64;
	const SESSION_DURATION: Duration = Duration::hours(72);

	pub fn new() -> (PrivateSessionId, Self) {
		let id = {
			let mut id_bytes = [0_u8; Self::SESSION_ID_LENGTH];
			rand::thread_rng().fill(&mut id_bytes);
			BASE64_URL_SAFE.encode(id_bytes)
		};

		let stored = StoredPassword::store_unchecked(&id);

		(
			id,
			Session {
				stored,
				expires: SystemTime::now() + Self::SESSION_DURATION,
			},
		)
	}

	pub fn is(&self, session: &str) -> bool {
		self.stored.is(session)
	}

	pub fn expired(&self) -> bool {
		self.expires < SystemTime::now()
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserEntry {
	password: StoredPassword,
	session: Option<Session>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StoredPassword(String, String);

impl StoredPassword {
	const SALT_LEN: usize = 16;

	const MIN_PASSWD_LEN: usize = 5;
	const MAX_PASSWD_LEN: usize = 64;

	fn store(password: &str) -> anyhow::Result<Self> {
		match password.len() {
			i if i < Self::MIN_PASSWD_LEN => {
				return Err(anyhow!(
					"password must be at least {} characters long",
					Self::MIN_PASSWD_LEN
				));
			}
			i if i > Self::MAX_PASSWD_LEN => {
				return Err(anyhow!(
					"password must be at most {} characters long",
					Self::MAX_PASSWD_LEN
				));
			}
			_ => {}
		}
		Ok(Self::store_unchecked(password))
	}

	fn store_unchecked(password: &str) -> Self {
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

		StoredPassword(password_hash, salt)
	}

	fn is(&self, password: &str) -> bool {
		let mut hasher = Sha256::new();
		hasher.update(format!("{}{}", password, self.1));
		let result = hasher.finalize();

		self.0 == BASE64_URL_SAFE.encode(result)
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

		assert!(userdata.has_valid_session("alice", &received_token));
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
