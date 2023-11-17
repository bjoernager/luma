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
use crate::cpu::{alu_exit_exception, Cpu, take_state_mut};
use crate::shifter::Shifter;

impl Cpu {
	pub(super) fn isa_mvn(&mut self, rd: u8, shifter: Shifter, s: bool) {
		log!("{} r{rd}, {shifter}", match s {
			false => "mvn",
			true  => "mvns",
		});

		take_state_mut!(state, self);

		let (mut result, c) = state.shifter_value(shifter);
		result              = !result;

		state.write_register(rd, result);

		if s {
			alu_exit_exception!(self, state, rd, {
				let mut cpsr = state.read_cpsr() & 0b00011111111111111111111111111111;
				cpsr |= (c as u32) << 0x1D;
				cpsr |= ((result == 0x0) as u32) << 0x1E;
				cpsr |= result & 0b10000000000000000000000000000000;
				state.write_cpsr(cpsr);
			});
		}
	}
}
