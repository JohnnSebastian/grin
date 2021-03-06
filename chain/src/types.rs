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

//! Base types that the block chain pipeline requires.

use grin_store::Error;
use core::core::{Block, BlockHeader};
use core::core::hash::{Hash, Hashed};
use core::core::target::Difficulty;
use core::ser;

/// The tip of a fork. A handle to the fork ancestry from its leaf in the
/// blockchain tree. References the max height and the latest and previous
/// blocks
/// for convenience and the total difficulty.
#[derive(Debug, Clone)]
pub struct Tip {
	/// Height of the tip (max height of the fork)
	pub height: u64,
	/// Last block pushed to the fork
	pub last_block_h: Hash,
	/// Block previous to last
	pub prev_block_h: Hash,
	/// Total difficulty accumulated on that fork
	pub total_difficulty: Difficulty,
}

impl Tip {
	/// Creates a new tip at height zero and the provided genesis hash.
	pub fn new(gbh: Hash) -> Tip {
		Tip {
			height: 0,
			last_block_h: gbh,
			prev_block_h: gbh,
			total_difficulty: Difficulty::one(),
		}
	}

	/// Append a new block to this tip, returning a new updated tip.
	pub fn from_block(bh: &BlockHeader) -> Tip {
		Tip {
			height: bh.height,
			last_block_h: bh.hash(),
			prev_block_h: bh.previous,
			total_difficulty: bh.total_difficulty.clone(),
		}
	}
}

/// Serialization of a tip, required to save to datastore.
impl ser::Writeable for Tip {
	fn write(&self, writer: &mut ser::Writer) -> Result<(), ser::Error> {
		try!(writer.write_u64(self.height));
		try!(writer.write_fixed_bytes(&self.last_block_h));
		try!(writer.write_fixed_bytes(&self.prev_block_h));
		self.total_difficulty.write(writer)
	}
}

impl ser::Readable<Tip> for Tip {
	fn read(reader: &mut ser::Reader) -> Result<Tip, ser::Error> {
		let height = try!(reader.read_u64());
		let last = try!(Hash::read(reader));
		let prev = try!(Hash::read(reader));
		let diff = try!(Difficulty::read(reader));
		Ok(Tip {
			height: height,
			last_block_h: last,
			prev_block_h: prev,
			total_difficulty: diff,
		})
	}
}

/// Trait the chain pipeline requires an implementor for in order to process
/// blocks.
pub trait ChainStore: Send + Sync {
	/// Get the tip that's also the head of the chain
	fn head(&self) -> Result<Tip, Error>;

	/// Block header for the chain head
	fn head_header(&self) -> Result<BlockHeader, Error>;

	/// Save the provided tip as the current head of our chain
	fn save_head(&self, t: &Tip) -> Result<(), Error>;

	/// Gets a block header by hash
	fn get_block(&self, h: &Hash) -> Result<Block, Error>;

	/// Gets a block header by hash
	fn get_block_header(&self, h: &Hash) -> Result<BlockHeader, Error>;

	/// Save the provided block in store
	fn save_block(&self, b: &Block) -> Result<(), Error>;

	/// Save the provided block header in store
	fn save_block_header(&self, bh: &BlockHeader) -> Result<(), Error>;

	/// Get the tip of the header chain
	fn get_header_head(&self) -> Result<Tip, Error>;

	/// Save the provided tip as the current head of the block header chain
	fn save_header_head(&self, t: &Tip) -> Result<(), Error>;

	/// Gets the block header at the provided height
	fn get_header_by_height(&self, height: u64) -> Result<BlockHeader, Error>;

	/// Saves the provided block header at the corresponding height. Also check
	/// the consistency of the height chain in store by assuring previous
	/// headers
	/// are also at their respective heights.
	fn setup_height(&self, bh: &BlockHeader) -> Result<(), Error>;
}

/// Bridge between the chain pipeline and the rest of the system. Handles
/// downstream processing of valid blocks by the rest of the system, most
/// importantly the broadcasting of blocks to our peers.
pub trait ChainAdapter {
	/// The blockchain pipeline has accepted this block as valid and added
	/// it to our chain.
	fn block_accepted(&self, b: &Block);
}

pub struct NoopAdapter { }
impl ChainAdapter for NoopAdapter {
	fn block_accepted(&self, b: &Block) {}
}
