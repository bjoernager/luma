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
	pub(super) fn isa_add_carry_register(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("adc r{rd}, r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let (mut value, c) = rn_value.overflowing_add(rm_value);
		value += c as u32;
		state.write_register(rd, value);
	}

	pub(super) fn isa_add_immediate(&mut self, rd: u8, rn: u8, imm: u32) {
		log!("add r{rd}, r{rn}, {imm:#X}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);

		let value = rn_value.wrapping_add(imm);
		state.write_register(rd, value);
	}

	pub(super) fn isa_add_register(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("add r{rd}, r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let value = rn_value.wrapping_add(rm_value);
		state.write_register(rd, value);
	}

	pub(super) fn isa_multiply_register(&mut self, rd: u8, rm: u8, rs: u8) {
		log!("mul r{rd}, r{rm}, r{rs}");

		let mut state = self.state.lock().unwrap();

		let rm_value = state.read_register(rm);
		let rs_value = state.read_register(rs);

		let result = rm_value.wrapping_mul(rs_value);
		state.write_register(rd, result);
	}

	pub(super) fn isa_reverse_subtract_immediate(&mut self, rd: u8, rm: u8, imm: u32) {
		log!("rsb r{rd}, r{rm}, {imm:#X}");

		let mut state = self.state.lock().unwrap();

		let rm_value = state.read_register(rm);

		let result = imm.wrapping_sub(rm_value);
		state.write_register(rd, result);
	}

	pub(super) fn isa_subtract_carry_register(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("sbc r{rd}, r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let (mut value, c) = rn_value.overflowing_sub(rm_value);
		value += (!c) as u32;
		state.write_register(rd, value);
	}

	pub(super) fn isa_subtract_immediate(&mut self, rd: u8, rn: u8, imm: u32) {
		log!("sub r{rd}, r{rn}, {imm:#X}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);

		let value = rn_value.wrapping_sub(imm);
		state.write_register(rd, value);
	}

	pub(super) fn isa_subtract_register(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("sub r{rd}, r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let value = rn_value.wrapping_sub(rm_value);
		state.write_register(rd, value);
	}
}
