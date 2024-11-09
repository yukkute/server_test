use std::{collections::HashMap, time::SystemTime};

use anyhow::anyhow;
use base64::{prelude::BASE64_URL_SAFE, Engine};
use log::{info, warn};
use rand::Rng;
use sha2::{Digest, Sha256};
use time::Duration;

type Username = String;
type PrivateSessionId = String;

#[derive(Debug, Default)]
pub struct UserData {
	users: HashMap<Username, UserEntry>,
	password_config: PasswordConfig,
}

impl UserData {
	pub fn register(&mut self, username: &str, password: &str) -> anyhow::Result<()> {
		if self.users.contains_key(username) {
			return Err(anyhow!("username already taken"));
		};

		self.users.insert(
			username.to_owned(),
			UserEntry {
				password: StoredPassword::store(
					password,
					Some(self.password_config),
				)?,
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

		user.session = Some(session);
		info!("Session issued to user: {username}");

		Ok(id)
	}

	pub fn validate_session(&mut self, username: &str, session_id: &str) -> anyhow::Result<()> {
		let Some(user) = self.users.get_mut(username) else {
			return Err(anyhow!("no such user: {username}"));
		};

		let further_error = anyhow!("invalid session: {username}");

		let Some(ref known_session) = user.session else {
			return Err(further_error);
		};

		if known_session.expired() {
			user.session = None;

			warn!("Session expired for user: {username}");

			return Err(further_error);
		}

		if !known_session.id_is(session_id) {
			return Err(further_error);
		}

		// Session is valid
		Ok(())
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
	stored: StoredPassword,
	expires: SystemTime,
}

impl Session {
	const SESSION_ID_LENGTH: usize = 64;
	const SESSION_DURATION: Duration = Duration::seconds(5);

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

	pub fn id_is(&self, session: &str) -> bool {
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

#[derive(Debug, Clone, Copy)]
struct PasswordConfig {
	min_length: usize,
	max_length: usize,
}

impl Default for PasswordConfig {
	fn default() -> Self {
		Self {
			min_length: 0,
			max_length: 64,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StoredPassword(String, String);

impl StoredPassword {
	const SALT_LEN: usize = 16;

	fn store(password: &str, config: Option<PasswordConfig>) -> anyhow::Result<Self> {
		if let Some(config) = config {
			if password.len() < config.min_length {
				return Err(anyhow!(
					"password must be at least {} characters long",
					config.min_length
				));
			}
			if password.len() > config.max_length {
				return Err(anyhow!(
					"password must be at most {} characters long",
					config.max_length
				));
			}
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
	fn same_passwords() {
		let same_password = "SAME PASSWORD IS USED";

		let entry = StoredPassword::store(same_password, None).unwrap();
		let second_entry = StoredPassword::store(same_password, None).unwrap();

		assert_ne!(entry, second_entry);

		dbg!(entry);
	}

	#[test]
	fn is_password_check() {
		let entry = StoredPassword::store("secure password", None).unwrap();
		assert!(entry.is("secure password"));
	}

	#[test]
	fn invalid_len() {
		let constraints = Some(PasswordConfig {
			min_length: 10,
			max_length: 10,
		});

		let entry = StoredPassword::store("short", constraints);
		assert!(entry.is_err());

		let second_entry = StoredPassword::store(
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
			Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
			constraints,
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

		assert!(userdata.validate_session("alice", &received_token).is_ok());
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
