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

use crate::luma::log;
use crate::luma::cpu::Cpu;

impl Cpu {
	pub(super) fn compare_immediate(&mut self, register: u8, immediate: u32) {
		log(&format!("cmp r{register}, {immediate:#X}"));

		let mut state = self.state.lock().unwrap();

		let register_value = state.read_register(register);

		let (temporary, c) = register_value.overflowing_sub(immediate);

		let v = false; // ???
		let z = temporary == 0x0;
		let n = temporary & 0b10000000000000000000000000000000 != 0x0;

		let mut cpsr = state.read_cpsr() & 0b00001111111111111111111111111111;
		cpsr |= (v as u32) << 0x1C;
		cpsr |= (c as u32) << 0x1D;
		cpsr |= (z as u32) << 0x1E;
		cpsr |= (n as u32) << 0x1F;
		state.write_cpsr(cpsr);
	}

	pub(super) fn compare_register(&mut self, left: u8, right: u8) {
		log(&format!("cmp r{left}, r{right}"));

		let mut state = self.state.lock().unwrap();

		let left_value  = state.read_register(left);
		let right_value = state.read_register(right);

		let (temporary, c) = left_value.overflowing_sub(right_value);

		let v = false; // ???
		let z = temporary == 0x0;
		let n = temporary & 0b10000000000000000000000000000000 != 0x0;

		let mut cpsr = state.read_cpsr() & 0b00001111111111111111111111111111;
		cpsr |= (v as u32) << 0x1C;
		cpsr |= (c as u32) << 0x1D;
		cpsr |= (z as u32) << 0x1E;
		cpsr |= (n as u32) << 0x1F;
		state.write_cpsr(cpsr);
	}
}
