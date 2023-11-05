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
	pub(super) fn isa_move_immediate(&mut self, rd: u8, immediate: u32) {
		log!("mov r{rd}, {immediate:#X}");

		let mut state = self.state.lock().unwrap();

		state.write_register(rd, immediate);
	}

	pub(super) fn isa_move_immediate_arithmetic_shift_right(&mut self, rd: u8, base: u8, shift: u8) {
		log!("mov r{rd}, r{base}, ASR {shift:#X}");

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let value = (base_value as i32).wrapping_shr(shift as u32) as u32;
		state.write_register(rd, value);
	}

	pub(super) fn isa_move_immediate_logical_shift_left(&mut self, rd: u8, base: u8, shift: u8) {
		log!("mov r{rd}, r{base}, LSL {shift:#X}");

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let value = base_value.wrapping_shl(shift as u32);
		state.write_register(rd, value);
	}

	pub(super) fn isa_move_immediate_logical_shift_right(&mut self, rd: u8, base: u8, shift: u8) {
		log!("mov r{rd}, r{base}, LSR {shift:#X}");

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let value = base_value.wrapping_shr(shift as u32);
		state.write_register(rd, value);
	}

	pub(super) fn isa_move_not_register(&mut self, rd: u8, rm: u8) {
		log!("mvn r{rd}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rm_value = state.read_register(rm);

		let result = !rm_value;
		state.write_register(rd, result);
	}

	pub(super) fn isa_move_register(&mut self, rd: u8, source: u8) {
		log!("mov r{rd}, r{source}");

		let mut state = self.state.lock().unwrap();

		let value = state.read_register(source);
		state.write_register(rd, value);
	}

	pub(super) fn isa_move_register_arithmetic_shift_right(&mut self, rd: u8, base: u8, shift: u8) {
		log!("mov r{rd}, r{base}, ASR r{shift}");

		let mut state = self.state.lock().unwrap();

		let base_value  = state.read_register(base) as i32;
		let shift_value = state.read_register(shift);

		let value = (base_value.wrapping_shr(shift_value)) as u32;
		state.write_register(rd, value);
	}

	pub(super) fn isa_move_register_logical_shift_left(&mut self, rd: u8, base: u8, shift: u8) {
		log!("mov r{rd}, r{base}, LSL r{shift}");

		let mut state = self.state.lock().unwrap();

		let base_value  = state.read_register(base);
		let shift_value = state.read_register(shift);

		let value = base_value.wrapping_shl(shift_value);
		state.write_register(rd, value);
	}

	pub(super) fn isa_move_register_logical_shift_right(&mut self, rd: u8, base: u8, shift: u8) {
		log!("mov r{rd}, r{base}, LSR r{shift}");

		let mut state = self.state.lock().unwrap();

		let base_value  = state.read_register(base);
		let shift_value = state.read_register(shift);

		let value = base_value.wrapping_shr(shift_value);
		state.write_register(rd, value);
	}

	pub(super) fn isa_move_register_rotate_right(&mut self, rd: u8, base: u8, rotate: u8) {
		log!("mov r{rd}, r{base}, ROR r{rotate}");

		let mut state = self.state.lock().unwrap();

		let base_value   = state.read_register(base);
		let rotate_value = state.read_register(rotate);

		let value = base_value.rotate_right(rotate_value);
		state.write_register(rd, value);
	}
}
