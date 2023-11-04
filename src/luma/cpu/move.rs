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
	pub(super) fn move_immediate(&mut self, destination: u8, immediate: u32) {
		log(&format!("mov r{destination}, {immediate:#X}"));

		let mut state = self.state.lock().unwrap();

		state.write_register(destination, immediate);
	}

	pub(super) fn move_immediate_arithmetic_shift_right(&mut self, destination: u8, base: u8, shift: u8) {
		log(&format!("mov r{destination}, r{base}, ASR {shift:#X}"));

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let value = (base_value as i32).wrapping_shr(shift as u32) as u32;
		state.write_register(destination, value);
	}

	pub(super) fn move_immediate_logical_shift_left(&mut self, destination: u8, base: u8, shift: u8) {
		log(&format!("mov r{destination}, r{base}, LSL {shift:#X}"));

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let value = base_value.wrapping_shl(shift as u32);
		state.write_register(destination, value);
	}

	pub(super) fn move_immediate_logical_shift_right(&mut self, destination: u8, base: u8, shift: u8) {
		log(&format!("mov r{destination}, r{base}, LSR {shift:#X}"));

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let value = base_value.wrapping_shr(shift as u32);
		state.write_register(destination, value);
	}

	pub(super) fn move_register(&mut self, destination: u8, source: u8) {
		log(&format!("mov r{destination}, r{source}"));

		let mut state = self.state.lock().unwrap();

		let value = state.read_register(source);
		state.write_register(destination, value);
	}

	pub(super) fn move_register_arithmetic_shift_right(&mut self, destination: u8, base: u8, shift: u8) {
		log(&format!("mov r{destination}, r{base}, ASR r{shift}"));

		let mut state = self.state.lock().unwrap();

		let base_value  = state.read_register(base) as i32;
		let shift_value = state.read_register(shift);

		let value = (base_value.wrapping_shr(shift_value)) as u32;
		state.write_register(destination, value);
	}

	pub(super) fn move_register_logical_shift_left(&mut self, destination: u8, base: u8, shift: u8) {
		log(&format!("mov r{destination}, r{base}, LSL r{shift}"));

		let mut state = self.state.lock().unwrap();

		let base_value  = state.read_register(base);
		let shift_value = state.read_register(shift);

		let value = base_value.wrapping_shl(shift_value);
		state.write_register(destination, value);
	}

	pub(super) fn move_register_logical_shift_right(&mut self, destination: u8, base: u8, shift: u8) {
		log(&format!("mov r{destination}, r{base}, LSR r{shift}"));

		let mut state = self.state.lock().unwrap();

		let base_value  = state.read_register(base);
		let shift_value = state.read_register(shift);

		let value = base_value.wrapping_shr(shift_value);
		state.write_register(destination, value);
	}
}
