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
use crate::cpu::{Cpu, exchange, take_state_mut};

impl Cpu {
	pub(super) fn isa_bx(&mut self, rm: u8) {
		take_state_mut!(state, self);

		log!("bx r{rm}");

		let mut target = state.read_register(rm);

		let t = target & 0b00000000000000000000000000000001 != 0x0;
		exchange!(self, t);

		let cpsr = state.read_cpsr() & 0b11111111111111111111111111011111 | (t as u32) << 0x5;
		state.write_cpsr(cpsr);

		target &= 0b11111111111111111111111111111110;
		target = target.wrapping_add(self.instruction_size);
		state.write_register(0xF, target);
	}
}
