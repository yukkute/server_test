#![forbid(
	future_incompatible,
	clippy::clone_on_ref_ptr,
	clippy::allow_attributes_without_reason
)]
#![deny(clippy::pedantic)]
#![allow(dead_code, reason = "Development is underway")]
#![allow(clippy::module_name_repetitions, reason = "Not an issue")]

mod generated; // subdirectory "server/src/generated"

mod available_port;
mod bank;
mod clicker;
mod data;
mod events;
mod grpc;
mod local_server;
mod multicast;
mod mutcell;
mod observer;
mod pb;
mod runtime;
mod user_data;
