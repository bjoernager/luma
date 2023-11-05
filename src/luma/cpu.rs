/*
	Copyright 2021-2023 Gabriel Jensen.

	This file is part of Luma.

	Luma is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero General Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Luma is distributed in the hope that it will be
	useful, but WITHOUT ANY WARRANTY; without even
	the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU
	Affero General Public License along with Luma.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::luma::instruction::Instruction;
use crate::luma::state::State;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;

pub mod boot;
pub mod decode_arm;
pub mod decode_thumb;
pub mod isa_arithmetic;
pub mod isa_bitwise;
pub mod isa_branch;
pub mod isa_logic;
pub mod isa_memory;
pub mod isa_move;
pub mod r#continue;

mod exchange;
mod test_predicate;

// https://github.com/rust-lang/rust/issues/115966

#[allow(unused_imports)]
pub use exchange::*;

#[allow(unused_imports)]
pub use test_predicate::*;

pub type Decoder = fn(&mut Cpu) -> Instruction;

pub struct Cpu {
	state: Arc<Mutex<State>>,
	cycle: u64,
	dead:  Arc<AtomicBool>,

	instruction_size: u32,
	decoder:          Decoder,
}

impl Cpu {
	pub fn new(state: State) -> Self {
		return Self {
			state: Arc::new(Mutex::new(state)),
			cycle: 0x0,
			dead:  Arc::new(AtomicBool::new(false)),

			instruction_size: 0x4,
			decoder:          Self::decode_arm,
		};
	}

	#[inline(always)]
	#[must_use]
	fn decode(&mut self) -> Instruction { (self.decoder)(self) }
}
