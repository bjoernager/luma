/*
	Copyright 2021-2023 Gabriel Jensen.

	This file is part of Luma.

	Luma is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero register Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Luma is distributed in the hope that it will be
	useful, but WITHOUT ANY WARRANTY; without even
	the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero register Public License for more details.

	You should have received a copy of the GNU
	Affero register Public License along with Luma.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::{log, log_status};
use crate::cpu_mode::CpuMode;
use crate::state::State;

impl State {
	pub fn bank(&mut self, mode: CpuMode) {
		log_status!("banking to {}", mode.name());

		let bank_index = bank_index(mode);

		self.registers[0x0] = &mut self.banks[bank_index.0][ 0x0] as *mut u32;
		self.registers[0x1] = &mut self.banks[bank_index.1][ 0x1] as *mut u32;
		self.registers[0x2] = &mut self.banks[bank_index.2][ 0x2] as *mut u32;
		self.registers[0x3] = &mut self.banks[bank_index.3][ 0x3] as *mut u32;
		self.registers[0x4] = &mut self.banks[bank_index.4][ 0x4] as *mut u32;
		self.registers[0x5] = &mut self.banks[bank_index.5][ 0x5] as *mut u32;
		self.registers[0x6] = &mut self.banks[bank_index.6][ 0x6] as *mut u32;
		self.registers[0x7] = &mut self.banks[bank_index.7][ 0x7] as *mut u32;
		self.registers[0x8] = &mut self.banks[bank_index.8][ 0x8] as *mut u32;
		self.registers[0x9] = &mut self.banks[bank_index.9][ 0x9] as *mut u32;
		self.registers[0xA] = &mut self.banks[bank_index.10][0xA] as *mut u32;
		self.registers[0xB] = &mut self.banks[bank_index.11][0xB] as *mut u32;
		self.registers[0xC] = &mut self.banks[bank_index.12][0xC] as *mut u32;
		self.registers[0xD] = &mut self.banks[bank_index.13][0xD] as *mut u32;
		self.registers[0xE] = &mut self.banks[bank_index.14][0xE] as *mut u32;
		self.registers[0xF] = &mut self.banks[bank_index.15][0xF] as *mut u32;

		log!("new register layout:");
		log!("    r0:  {:#010X}    r4: {:#010X}    r8:  {:#010X}    r12: {:#010X}", self.read_register(0x0), self.read_register(0x4), self.read_register(0x8), self.read_register(0xC));
		log!("    r1:  {:#010X}    r5: {:#010X}    r9:  {:#010X}    sp:  {:#010X}", self.read_register(0x1), self.read_register(0x5), self.read_register(0x9), self.read_register(0xD));
		log!("    r2:  {:#010X}    r6: {:#010X}    r10: {:#010X}    lr:  {:#010X}", self.read_register(0x2), self.read_register(0x6), self.read_register(0xA), self.read_register(0xE));
		log!("    r3:  {:#010X}    r7: {:#010X}    r11: {:#010X}    pc:  {:#010X}", self.read_register(0x3), self.read_register(0x7), self.read_register(0xB), self.read_register(0xF));
	}
}

#[must_use]
fn bank_index(mode: CpuMode) -> (
	usize, // r0
	usize, // r1
	usize, // r2
	usize, // r3
	usize, // r4
	usize, // r5
	usize, // r6
	usize, // r7
	usize, // r8
	usize, // r9
	usize, // r10
	usize, // r11
	usize, // r12
	usize, // sp
	usize, // lr
	usize, // pc
) {
	use CpuMode::*;

	return match mode {
		User                 => (0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0),
		System               => (0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0),
		FastInterruptRequest => (0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x0),
		InterruptRequest     => (0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0),
		Supervisor           => (0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x3, 0x0),
		Abort                => (0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x4, 0x0),
		Undefined            => (0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5, 0x5, 0x0),
	};
}
