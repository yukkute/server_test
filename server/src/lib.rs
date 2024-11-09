#![allow(dead_code)]
#![allow(clippy::module_name_repetitions)]
#![forbid(future_incompatible)]

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
