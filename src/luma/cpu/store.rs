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
	pub(super) fn store_byte_immediate_offset(&mut self, source: u8, base: u8, offset: i16) {
		log(&format!("strb r{source}, [r{base}, {offset:#X}]"));

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let target = base_value.wrapping_add_signed(offset as i32);

		let value = state.read_register(source) as u8;
		state.write_byte(target, value);
	}

	pub(super) fn store_byte_register_offset(&mut self, source: u8, base: u8, offset: u8) {
		log(&format!("strb r{source}, [r{base}, r{offset}]"));

		let mut state = self.state.lock().unwrap();

		let base_value   = state.read_register(base);
		let offset_value = state.read_register(offset);

		let target = base_value.wrapping_add(offset_value);

		let value = state.read_register(source) as u8;
		state.write_byte(target, value);
	}

	pub(super) fn store_halfword(&mut self, source: u8, base: u8, offset: i16) {
		log(&format!("strh r{source}, [r{base}, {offset:#X}]"));

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let target = base_value.wrapping_add_signed(offset as i32);

		let value = state.read_register(source) as u16;
		state.write_halfword(target, value);
	}

	pub(super) fn store_immediate_offset(&mut self, source: u8, base: u8, offset: i16) {
		log(&format!("str r{source}, [r{base}, {offset:#X}]"));

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let target = base_value.wrapping_add_signed(offset as i32);

		let value = state.read_register(source);
		state.write_word(target, value);
	}
}
