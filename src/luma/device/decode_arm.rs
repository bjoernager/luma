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
	Affero General Public License along with Luma. If not, 
	see <https://www.gnu.org/licenses/>. 
*/

use crate::luma::device::{Branch, Device, Move, Trap};

impl Device {
	pub fn decode_arm(&mut self) {
		debug_assert!(!self.thumb());

		let address = self.registers[0xF] - 0x8;

		let opcode = self.read_word(address);
		if cfg!(debug_assertions) { eprintln!("{opcode:#034b} @ {address:#010X}") }

		let condition = ((opcode & 0b11110000000000000000000000000000) >> 0x1C) as u8;
		if !self.check_condition(condition) { return self.r#continue() }

		// b{cond}{l} +/-offset_24
		if opcode & 0b00001110000000000000000000000000 == 0b00001010000000000000000000000000 {
			let link = opcode & 0b00000001000000000000000000000000 != 0x0;
			
			let offset = {
				let mut offset = opcode & 0b00000000111111111111111111111111;

				if offset & 0b00000000100000000000000000000000 != 0x0 { offset |= 0b00111111000000000000000000000000 } // Sign-extend.

				offset <<= 0x2;

				offset as i32
			};

			return self.branch(Branch::Offset(offset, link));
		}

		// bx{cond} Rm
		if opcode & 0b00001111111111111111111111110000 == 0b00000001001011111111111100010000 {
			let register = (opcode & 0b00000000000000000000000000001111) as u8;

			return self.branch(Branch::Register(register));
		}

		// ldr|str{cond}{b} Rn, +/-offset_12
		if opcode & 0b00001111001000000000000000000000 == 0b00000101000000000000000000000000 {
			let register = ((opcode & 0b00000000000000001111000000000000) >> 0xC) as u8;

			let base = ((opcode & 0b00000000000011110000000000000000) >> 0x10) as u8; 

			let immediate = (opcode & 0b00000000000000000000111111111111) as u16;

			let u = opcode & 0b00000000100000000000000000000000 != 0x0;
			let b = opcode & 0b00000000010000000000000000000000 != 0x0;
			let l = opcode & 0b00000000000100000000000000000000 != 0x0;

			self.store(register, base, immediate, u, b, l);
			return self.r#continue();
		}

		// mov{cond}{s} Rd, Rn
		if opcode & 0b00001101111111100000111111110000 == 0b00000001101000000000000000000000 {
			let destination = ((opcode & 0b00000000000000001111000000000000) >> 0xC) as u8;
			let source      =  (opcode & 0b00000000000000000000000000001111)         as u8;

			let value                            = self.registers[source as usize];
			self.registers[destination as usize] = value;

			let s = opcode & 0b00000000000100000000000000000000 != 0x0;
			
			self.r#move(destination, Move::Register(source), s);
			return self.r#continue();
		}

		self.trap(Trap::InvalidArmOpcode(self.registers[0xF] - 0x8, opcode));

		self.r#continue();
	}
}
