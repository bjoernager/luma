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
	pub(super) fn add_immediate(&mut self, destination: u8, base: u8, immediate: u32) {
		log(&format!("add r{destination}, r{base}, {immediate:#X}"));

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);

		let value = base_value.wrapping_add(immediate);
		state.write_register(destination, value);
	}

	pub(super) fn add_register(&mut self, destination: u8, base: u8, add: u8) {
		log(&format!("add r{destination}, r{base}, r{add}"));

		let mut state = self.state.lock().unwrap();

		let base_value = state.read_register(base);
		let add_value  = state.read_register(add);

		let value = base_value.wrapping_add(add_value);
		state.write_register(destination, value);
	}
}
