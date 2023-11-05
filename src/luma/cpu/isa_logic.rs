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
	pub(super) fn isa_compare_immediate(&mut self, rn: u8, imm: u32) {
		log!("cmp r{rn}, {imm:#X}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);

		let (temporary, c) = rn_value.overflowing_sub(imm);

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

	pub(super) fn isa_compare_negative_register(&mut self, rn: u8, rm: u8) {
		log!("cmn r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let (temporary, c) = rn_value.overflowing_add(rm_value);

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

	pub(super) fn isa_compare_register(&mut self, rn: u8, rm: u8) {
		log!("cmp r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let (temporary, c) = rn_value.overflowing_sub(rm_value);

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

	pub(super) fn isa_test_register(&mut self, rn: u8, rm: u8) {
		log!("tst r{rn}, r{rm}");

		let mut state = self.state.lock().unwrap();

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let temporary = rn_value & !rm_value;

		let z = temporary == 0x0;
		let n = temporary & 0b10000000000000000000000000000000 != 0x0;

		let mut cpsr = state.read_cpsr() & 0b00001111111111111111111111111111;
		cpsr |= (z as u32) << 0x1E;
		cpsr |= (n as u32) << 0x1F;
		state.write_cpsr(cpsr);
	}
}
