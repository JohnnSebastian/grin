// Copyright 2016 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Networking code to connect to other peers and exchange block, transactions,
//! etc.

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate grin_core as core;
extern crate grin_util as util;
#[macro_use]
extern crate log;
extern crate futures;
#[macro_use]
extern crate tokio_core;
extern crate tokio_timer;
extern crate rand;
extern crate time;
extern crate num;

mod conn;
pub mod handshake;
mod msg;
mod peer;
mod protocol;
mod server;
mod types;

pub use server::{Server, DummyAdapter};
pub use peer::Peer;
pub use types::{P2PConfig, NetAdapter, MAX_LOCATORS, MAX_BLOCK_HEADERS};
