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

use crate::log;
use crate::cpu::{Cpu, take_state_mut};

impl Cpu {
	pub(super) fn isa_bl(&mut self, imm: i32) {
		take_state_mut!(state, self);

		let     link_target   = state.read_register(0xF).wrapping_sub(self.instruction_size);
		let mut branch_target = state.read_register(0xF).wrapping_add_signed(imm);

		log!("bl {branch_target:#X}");

		branch_target = branch_target.wrapping_add(self.instruction_size);
		state.write_register(0xE, link_target);
		state.write_register(0xF, branch_target);
	}

	pub(super) fn isa_bl0(&mut self, imm: i32) {
		take_state_mut!(state, self);

		let target = state.read_register(0xF).wrapping_add_signed(imm);

		state.write_register(0xE, target);
	}

	pub(super) fn isa_bl1(&mut self, imm: i32) {
		take_state_mut!(state, self);

		let mut branch_target = state.read_register(0xE).wrapping_add_signed(imm);
		let     link_target   = state.read_register(0xF).wrapping_sub(0x2);

		log!("bl {branch_target:#X}");

		state.write_register(0xE, link_target);

		branch_target = branch_target.wrapping_add(0x2);
		state.write_register(0xF, branch_target);
	}
}
