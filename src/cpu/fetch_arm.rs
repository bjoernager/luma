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

use crate::{Error, log};
use crate::cpu::{Cpu, take_state};
use crate::instruction::Instruction;
use crate::predicate::Predicate;

impl Cpu {
	#[must_use]
	pub(super) fn fetch_arm(&mut self) -> (Instruction, Predicate, u32) {
		take_state!(state, self);

		let address = state.read_register(0xF).wrapping_sub(0x8);
		let opcode  = state.read_word(address);
		let cpsr    = state.read_cpsr();

		drop(state);

		log!();
		log!("\u{1B}[1m{opcode:032b}\u{1B}[0m @ \u{1B}[1m{address:08X}\u{1B}[0m - ({})", self.cycle);

		let (instruction, predicate) = Instruction::decode_arm(opcode);

		match (instruction, predicate) {
			| (Instruction::Undefined, _)
			| (_, Predicate::Nv)
			=> Error::InvalidArmOpcode(address, opcode).trap(),

			_ => {},
		};

		return (instruction, predicate, cpsr);
	}
}
