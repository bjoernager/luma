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
use crate::exception::Exception;

impl Cpu {
	pub(super) fn enter_exception(&mut self, exception: Exception) {
		take_state_mut!(state, self);

		let mode = exception.mode();

		state.bank(mode);

		let lr = state.read_register(0xF);
		state.write_register(0xE, lr);

		let mut cpsr = state.read_cpsr();
		state.write_spsr(mode, cpsr);

		cpsr &= 0b11111111111111111111111101000000;
		cpsr |= 0b00000000000000000000000010000000;
		cpsr |= mode as u32;

		cpsr &= match exception {
			| Exception::FastInterruptRequest
			| Exception::Reset                => 0b11111111111111111111111110111111,

			_ => 0b11111111111111111111111111111111,
		};

		state.write_cpsr(cpsr);

		exchange!(self, false);

		let pc = exception.vector_address().wrapping_add(0x4);
		state.write_register(0xF, pc);
	}
}
