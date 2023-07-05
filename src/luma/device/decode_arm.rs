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

use crate::luma::device::{Device, Move, Trap};

impl Device {
	pub fn decode_arm(&mut self) {
		debug_assert!(!self.thumb());

		let (address, _) = self.registers[0xF].overflowing_sub(0x8);

		let opcode = self.read_word(address);
		if cfg!(debug_assertions) { eprintln!("{opcode:#034b} @ {address:#010X}") }

		// b{cond}{l} Offset24
		if opcode & 0b00001110000000000000000000000000 == 0b00001010000000000000000000000000 {
			let condition = ((opcode & 0b11110000000000000000000000000000) >> 0x1C) as u8;
			if !self.check_condition(condition) { return self.arm_continue() }

			let immediate = opcode & 0b00000000111111111111111111111111;

			let l = opcode & 0b00000001000000000000000000000000 != 0x0;

			let offset = ((immediate << 0x8) as i32) >> 0x6;

			return self.arm_branch(offset, l);
		}

		// bx{cond} Rm
		if opcode & 0b00001111111111111111111111110000 == 0b00000001001011111111111100010000 {
			let condition = ((opcode & 0b11110000000000000000000000000000) >> 0x1C) as u8;
			if !self.check_condition(condition) { return self.arm_continue() }

			let register = (opcode & 0b00000000000000000000000000001111) as u8;

			return self.arm_branch_exchange(register);
		}

		// ldr|str{cond}{b} Rd, [Rn, Offset12]
		if opcode & 0b00001111001000000000000000000000 == 0b00000101000000000000000000000000 {
			let condition = ((opcode & 0b11110000000000000000000000000000) >> 0x1C) as u8;
			if !self.check_condition(condition) { return self.arm_continue() }

			let immediate = (opcode & 0b00000000000000000000111111111111) as u16;

			let register = ((opcode & 0b00000000000000001111000000000000) >> 0xC) as u8;

			let base = ((opcode & 0b00000000000011110000000000000000) >> 0x10) as u8;

			let l = opcode & 0b00000000000100000000000000000000 != 0x0;
			let b = opcode & 0b00000000010000000000000000000000 != 0x0;
			let u = opcode & 0b00000000100000000000000000000000 != 0x0;

			self.arm_store(register, base, immediate, u, b, l);
			return self.arm_continue();
		}

		// mov{cond}{s} Rd, Rn
		if opcode & 0b00001101111111100000111111110000 == 0b00000001101000000000000000000000 {
			let condition = ((opcode & 0b11110000000000000000000000000000) >> 0x1C) as u8;
			if !self.check_condition(condition) { return self.arm_continue() }

			let source = (opcode & 0b00000000000000000000000000001111) as u8;

			let destination = ((opcode & 0b00000000000000001111000000000000) >> 0xC) as u8;

			let s = opcode & 0b00000000000100000000000000000000 != 0x0;

			self.arm_move(destination, Move::Register(source), s);
			return self.arm_continue();
		}

		// svc{cond} Immediate24
		if opcode & 0b00001111000000000000000000000000 == 0b00001111000000000000000000000000 {
			let condition = ((opcode & 0b11110000000000000000000000000000) >> 0x1C) as u8;
			if !self.check_condition(condition) { return self.arm_continue() }

			let immediate = opcode & 0b00000000111111111111111111111111;

			return self.interrupt(immediate);
		}

		self.trap(Trap::InvalidArmOpcode(self.registers[0xF] - 0x8, opcode));

		self.arm_continue();
	}
}
