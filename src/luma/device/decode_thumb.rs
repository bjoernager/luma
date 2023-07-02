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
	pub fn decode_thumb(&mut self) {
		debug_assert!(self.thumb());

		let address = self.registers[0xF] - 0x4;

		let opcode = self.read_halfword(address);
		if cfg!(debug_assertions) { eprintln!("{opcode:#018b}                 @ {address:#010X}") }

		// b
		if opcode & 0b1111100000000000 == 0b1110000000000000 {
			let offset = {
				let mut offset = (opcode & 0b0000011111111111) as u32;

				offset <<= 0x1;

				if offset & 0b00000000000000000000100000000000 != 0x0 { offset |= 0b11111111111111111111000000000000 }

				offset as i32
			};

			return self.branch(Branch::Offset(offset, false));
		}

		// b{cond}
		if opcode & 0b1111000000000000 == 0b1101000000000000 {
			let offset = {
				let mut offset = (opcode & 0b0000000011111111) as u32;

				offset <<= 0x1;

				if offset & 0b00000000000000000000000100000000 != 0x0 { offset |= 0b11111111111111111111111000000000 }

				offset as i32
			};

			return self.branch(Branch::Offset(offset, false));
		}

		// bx
		if opcode & 0b1111111110000111 == 0b0100011100000000 {
			let register = ((opcode & 0b0000000001111000) >> 0x3) as u8;

			return self.branch(Branch::Register(register));
		}

		// mov Rd, Rm
		if opcode & 0b1111111100000000 == 0b0100011000000000 {
			let destination = ((opcode & 0b0000000000000111) | (opcode & 0b0000000010000000) >> 0x4) as u8;

			let source = ((opcode & 0b0000000001111000) >> 0x3) as u8;

			self.r#move(destination, Move::Register(source), false);
			return self.r#continue();
		}

		// movs Rd, immediate_8
		if opcode & 0b1111100000000000 == 0b0010000000000000 {
			let destination = ((opcode & 0b0000011100000000) >> 0x8) as u8;

			let immediate = (opcode & 0b0000000011111111) as u8;

			self.r#move(destination, Move::Immediate(immediate), true);
			return self.r#continue();
		}

		// movs Rd, Rn
		if opcode & 0b1111111111000000 == 0b0001110000000000 {
			let destination = ((opcode & 0b0000000000000111) >> 0x3) as u8;

			let source = ((opcode & 0b0000000000111000) >> 0x3) as u8;

			self.r#move(destination, Move::Register(source), true);
			return self.r#continue();
		}

		self.trap(Trap::InvalidThumbOpcode(address, opcode));

		self.r#continue();
	}
}
