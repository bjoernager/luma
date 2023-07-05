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

use crate::luma::device::{Device, Trap};

impl Device {
	pub fn decode_thumb(&mut self) {
		debug_assert!(self.thumb());

		let (address, _) = self.registers[0xF].overflowing_sub(0x4);

		let opcode = self.read_halfword(address);
		if cfg!(debug_assertions) { eprintln!("{opcode:#018b}                 @ {address:#010X}") }

		// b
		if opcode & 0b1111100000000000 == 0b1110000000000000 {
			let immediate = opcode & 0b0000011111111111;

			let offset = (((immediate as u32) << 0x15) as i32) >> 0x14;

			return self.thumb_branch(offset);
		}

		// b{cond}, svc Immediate8
		if opcode & 0b1111000000000000 == 0b1101000000000000 {
			let condition = ((opcode & 0b0000111100000000) >> 0x8) as u8;

			if condition == 0xF {
				let immediate = (opcode & 0b0000000011111111) as u8;

				return self.interrupt(immediate as u32);
			}

			if !self.check_condition(condition) { return self.thumb_continue() }

			let immediate = opcode & 0b0000000011111111;

			let offset = (((immediate as u32) << 0x18) as i32) >> 0x17;

			return self.thumb_branch(offset);
		}

		// bl Offset11
		if opcode & 0b1111100000000000 == 0b1111000000000000 {
			let immediate = opcode & 0b0000011111111111;

			let offset = (((immediate as u32) << 0x15) as i32) >> 0x9;

			self.thumb_branch_link0(offset);
			return self.thumb_continue();
		}

		// bl Offset11
		if opcode & 0b1111100000000000 == 0b1111100000000000 {
			let immediate = opcode & 0b0000011111111111;

			let offset = ((immediate as u32) << 0x1) as i32;

			return self.thumb_branch_link1(offset);
		}

		// bx Rm
		if opcode & 0b1111111110000111 == 0b0100011100000000 {
			let register = ((opcode & 0b0000000001111000) >> 0x3) as u8;

			return self.thumb_branch_exchange(register);
		}

		// ldr Rd, [Rn, Immediate5]
		if opcode & 0b1111100000000000 == 0b0110100000000000 {
			let destination = (opcode & 0b0000000000000111) as u8;

			let base = ((opcode & 0b0000000000111000) >> 0x3) as u8;

			let immediate = ((opcode & 0b0000011111000000) >> 0x5) as u8;

			self.thumb_load_immediate(destination, base, immediate);
			return self.thumb_continue();
		}

		// ldr Rd, [Rn, Rm]
		if opcode & 0b1111111000000000 == 0b0101100000000000 {
			let destination = (opcode & 0b0000000000000111) as u8;

			let base = ((opcode & 0b0000000000111000) >> 0x3) as u8;

			let offset = ((opcode & 0b0000000111000000) >> 0x5) as u8;

			self.thumb_load_register(destination, base, offset);
			return self.thumb_continue();
		}

		// ldr Rd, [r13, Immediate8]
		if opcode & 0b1111100000000000 == 0b1001100000000000 {
			let destination = ((opcode & 0b0000011100000000) >> 0x8) as u8;

			let immediate = (opcode & 0b0000000011111111) as u8;

			self.thumb_load_sp(destination, immediate);
			return self.thumb_continue();
		}

		// ldr Rd, [r15, Immediate8]
		if opcode & 0b1111100000000000 == 0b0100100000000000 {
			let destination = ((opcode & 0b0000011100000000) >> 0x8) as u8;

			let immediate = (opcode & 0b0000000011111111) as u8;

			self.thumb_load_pc(destination, immediate);
			return self.thumb_continue();
		}

		// lsl Rd, Rm, Immediate5
		if opcode & 0b1111100000000000 == 0b0000000000000000 {
			let destination = (opcode & 0b0000000000000111) as u8;

			let source = ((opcode & 0b0000000000111000) >> 0x3) as u8;

			let immediate = ((opcode & 0b0000011111000000) >> 0x5) as u8;

			self.thumb_shift_left(destination, source, immediate);
			return self.thumb_continue();
		}

		// lsr Rd, Rm, Immediate5
		if opcode & 0b1111100000000000 == 0b0000100000000000 {
			let destination = (opcode & 0b0000000000000111) as u8;

			let source = ((opcode & 0b0000000000111000) >> 0x3) as u8;

			let immediate = ((opcode & 0b0000011111000000) >> 0x5) as u8;

			self.thumb_shift_right(destination, source, immediate);
			return self.thumb_continue();
		}

		// mov Rd, Rm
		if opcode & 0b1111111100000000 == 0b0100011000000000 {
			let destination = ((opcode & 0b0000000000000111) | (opcode & 0b0000000010000000) >> 0x4) as u8;

			let source = ((opcode & 0b0000000001111000) >> 0x3) as u8;

			self.thumb_move_high(destination, source);
			return self.thumb_continue();
		}

		// movs Rd, Immediate8
		if opcode & 0b1111100000000000 == 0b0010000000000000 {
			let destination = ((opcode & 0b0000011100000000) >> 0x8) as u8;

			let immediate = (opcode & 0b0000000011111111) as u8;

			self.thumb_move_immediate(destination, immediate);
			return self.thumb_continue();
		}

		// movs Rd, Rn
		if opcode & 0b1111111111000000 == 0b0001110000000000 {
			let destination = ((opcode & 0b0000000000000111) >> 0x3) as u8;

			let source = ((opcode & 0b0000000000111000) >> 0x3) as u8;

			self.thumb_move(destination, source);
			return self.thumb_continue();
		}

		// pop Registers
		if opcode & 0b1111111000000000 == 0b1011110000000000 {
			let list = (opcode & 0b0000000011111111) as u8;

			let r = opcode & 0b0000000100000000 != 0x0;

			self.thumb_pop(list, r);
			if !r { return self.thumb_continue() }
		}

		// push Registers
		if opcode & 0b1111111000000000 == 0b1011010000000000 {
			let list = (opcode & 0b0000000011111111) as u8;

			let r = opcode & 0b0000000100000000 != 0x0;

			self.thumb_push(list, r);
			return self.thumb_continue();
		}

		// strh Rd, [Rn, Immediate5]
		if opcode & 0b1111100000000000 == 0b1000000000000000 {
			let source = (opcode & 0b0000000000000111) as u8;

			let base = ((opcode & 0b0000000000000111) >> 0x3) as u8;

			let immediate = ((opcode & 0b0000011111000000) >> 0x6) as u8;

			self.thumb_store_halfword_immediate(source, base, immediate);
			return self.thumb_continue();
		}

		self.trap(Trap::InvalidThumbOpcode(address, opcode));

		self.thumb_continue();
	}
}
