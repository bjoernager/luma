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
	pub(super) fn isa_and_register(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("and r{rd}, r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let result = rn_value & rm_value;
		state.write_register(rd, result);
	}

	pub(super) fn isa_bit_clear_register(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("bic r{rd}, r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let result = rn_value & !rm_value;
		state.write_register(rd, result);
	}

	pub(super) fn isa_exclusive_or_register(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("eor r{rd}, r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let result = rn_value ^ rm_value;
		state.write_register(rd, result);
	}

	pub(super) fn isa_logical_or_register(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("eor r{rd}, r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let result = rn_value | rm_value;
		state.write_register(rd, result);
	}
}
