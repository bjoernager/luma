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
	pub(super) fn load_halfword(&mut self, destination: u8, base: u8, offset: i16) {
		let mut state = self.state.lock().unwrap();

		log(&format!("ldrh r{destination}, [r{base}, {offset:#X}]"));

		let base_value = state.read_register(base);

		let target = base_value.wrapping_add_signed(offset as i32);

		let value = state.read_halfword(target) as u32;
		state.write_register(destination, value);
	}

	pub(super) fn load_immediate_offset(&mut self, destination: u8, base: u8, offset: i16) {
		let mut state = self.state.lock().unwrap();

		log(&format!("ldr r{destination}, [r{base}, {offset:#X}]"));

		let base_value = state.read_register(base);

		let target = base_value.wrapping_add_signed(offset as i32);

		let value = state.read_word(target);
		state.write_register(destination, value);
	}

	pub(super) fn load_pc(&mut self, destination: u8, offset: i16) {
		// Slightly different from load_immediate_offset
		// due to the target being forced word-aligned.

		let mut state = self.state.lock().unwrap();

		log(&format!("ldr r{destination}, [pc, {offset:#X}]"));

		let base_value = state.read_register(0xF) & 0b11111111111111111111111111111100;

		let target = base_value.wrapping_add_signed(offset as i32);

		let value = state.read_word(target);
		state.write_register(destination, value);
	}
}
