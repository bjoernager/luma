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

use crate::instruction::Instruction;
use crate::predicate::Predicate;
use crate::state::State;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;

mod alu_exit_exception;
mod boot;
mod r#continue;
mod enter_exception;
mod execute;
mod exit_exception;
mod fetch_arm;
mod fetch_thumb;

mod isa_adc;
mod isa_add;
mod isa_and;
mod isa_bic;
mod isa_bl;
mod isa_b;
mod isa_bx;
mod isa_cmn;
mod isa_cmp;
mod isa_eor;
mod isa_mov;
mod isa_mul;
mod isa_mvn;
mod isa_orr;
mod isa_rsb;
mod isa_rsc;
mod isa_sbc;
mod isa_sub;
mod isa_swi;
mod isa_teq;
mod isa_tst;

mod isa_memory;

mod exchange;
mod take_state;

// <https://github.com/rust-lang/rust/issues/115966>

#[allow(unused_imports)]
pub use alu_exit_exception::*;

#[allow(unused_imports)]
pub use exchange::*;

#[allow(unused_imports)]
pub use take_state::*;

pub type Fetcher = fn(&mut Cpu) -> (Instruction, Predicate, u32);

pub struct Cpu {
	state: Arc<Mutex<State>>,
	cycle: u64,
	dead:  Arc<AtomicBool>,

	instruction_size: u32,
	fetcher:          Fetcher,
}

impl Cpu {
	pub fn new(state: State) -> Self {
		return Self {
			state: Arc::new(Mutex::new(state)),
			cycle: 0x0,
			dead:  Arc::new(AtomicBool::new(false)),

			instruction_size: 0x4,
			fetcher:          Self::fetch_arm,
		};
	}

	#[inline(always)]
	#[must_use]
	fn fetch(&mut self) -> (Instruction, Predicate, u32) { (self.fetcher)(self) }
}
