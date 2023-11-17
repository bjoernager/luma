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

use crate::cpu::{Cpu, exchange, take_state_mut};
use crate::cpu_mode::CpuMode;

impl Cpu {
	pub(super) fn exit_exception(&mut self) {
		take_state_mut!(state, self);

		let cpsr = state.read_cpsr();
		let old_mode = CpuMode::from_raw((cpsr & 0b00000000000000000000000000011111) as u8);

		let spsr = state.read_spsr(old_mode);
		let new_mode = CpuMode::from_raw((spsr & 0b00000000000000000000000000011111) as u8);

		let t = spsr & 0b00000000000000000000000000100000 != 0x0;
		state.write_cpsr(spsr);
		state.bank(new_mode);
		exchange!(self, t);
	}
}
