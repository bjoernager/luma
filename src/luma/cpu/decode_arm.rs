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

use crate::luma::{Error, log};
use crate::luma::cpu::Cpu;
use crate::luma::instruction::Instruction;

use std::hint::unreachable_unchecked;

impl Cpu {
	#[must_use]
	pub(super) fn decode_arm(&mut self) -> Instruction {
		let state = self.state.lock().unwrap();

		let address = state.read_register(0xF).wrapping_sub(0x8);
		let opcode  = state.read_word(address);

		drop(state);

		log!();
		log!("\u{1B}[1m{opcode:032b}\u{1B}[0m @ \u{1B}[1m{address:08X}\u{1B}[0m - ({})", self.cycle);

		return decode(address, opcode);
	}
}

#[must_use]
fn decode(address: u32, opcode: u32) -> Instruction {
	use Instruction::*;

	match (opcode & 0b00001110000000000000000000000000).wrapping_shr(0x19) {
		0b000 => {
			match (opcode & 0b00000000000000000000000000010000).wrapping_shr(0x4) {
				0b0 => {
					let _source = (opcode & 0b00000000000000000000000000001111) as u8;

					let _destination = (opcode & 0b00000000000000001111000000000000).wrapping_shr(0xC) as u8;

					let _base = (opcode & 0b00000000000011110000000000000000).wrapping_shr(0x10) as u8;

					let _s = opcode & 0b00000000000100000000000000000000 != 0x0;

					match (opcode & 0b00000001111000000000000000000000).wrapping_shr(0x15) {
						0b0000 => {},

						0b0001 => {},

						0b0010 => {},

						0b0011 => {},

						0b0100 => {},

						0b0101 => {},

						0b0110 => {},

						0b0111 => {},

						0b1000 => {},

						0b1001 => {},

						0b1010 => {},

						0b1011 => {},

						0b1100 => {},

						0b1101 => {},

						0b1110 => {},

						0b1111 => {},

						_ => unsafe { unreachable_unchecked() },
					}
				},

				0b1 => {
					let source = (opcode & 0b00000000000000000000000000001111) as u8;

					let _shift = (opcode & 0b00000000000000000000000011110000) as u8;

					let _destination = (opcode & 0b00000000000000001111000000000000).wrapping_shr(0xC) as u8;

					let _base = (opcode & 0b00000000000011110000000000000000).wrapping_shr(0x10) as u8;

					let _s = opcode & 0b00000000000100000000000000000000 != 0x0;

					match (opcode & 0b00000001111000000000000000000000).wrapping_shr(0x15) {
						0b0000 => {},

						0b0001 => {},

						0b0010 => {},

						0b0011 => {},

						0b0100 => {},

						0b0101 => {},

						0b0110 => {},

						0b0111 => {},

						0b1000 => {},

						0b1001 => {
							// Unpredictable if any of shift, destination, or
							// base is non-zero. Unpredictable if s is true.

							return BranchExchange(source);
						},

						0b1010 => {},

						0b1011 => {},

						0b1100 => {},

						0b1101 => {},

						0b1110 => {},

						0b1111 => {},

						_ => unsafe { unreachable_unchecked() },
					}
				},

				_ => unsafe { unreachable_unchecked() },
			}
		},

		0b001 => {
			let immediate = (opcode & 0b00000000000000000000000011111111) as u8;

			let rotate = (opcode & 0b00000000000000000000111100000000).wrapping_shr(0x8);

			let destination = (opcode & 0b00000000000000001111000000000000).wrapping_shr(0xC) as u8;

			let base = (opcode & 0b00000000000011110000000000000000).wrapping_shr(0x10) as u8;

			let _s = opcode & 0b00000000000100000000000000000000 != 0x0;

			let rotate = rotate << 0x1;
			let immediate = (immediate as u32).rotate_right(rotate);

			match (opcode & 0b00000001111000000000000000000000).wrapping_shr(0x15) {
				0b0000 => {},

				0b0001 => {},

				0b0010 => {},

				0b0011 => {},

				0b0100 => return AddImmediate(destination, base, immediate),

				0b0101 => {},

				0b0110 => {},

				0b0111 => {},

				0b1000 => {},

				0b1001 => {},

				0b1010 => {},

				0b1011 => {},

				0b1100 => {},

				// Unpredictable if base is non-zero.
				0b1101 => return MoveImmediate(destination, immediate),

				0b1110 => {},

				0b1111 => {},

				_ => unsafe { unreachable_unchecked() },
			}
		},

		0b010 => {
			let immediate = (opcode & 0b00000000000000000000111111111111) as u16;

			let register = (opcode & 0b00000000000000001111000000000000).wrapping_shr(0xC) as u8;

			let base = (opcode & 0b00000000000011110000000000000000).wrapping_shr(0x10) as u8;

			let l = opcode & 0b00000000000100000000000000000000 != 0x0;
			let _w = opcode & 0b00000000001000000000000000000000 != 0x0;
			let _b = opcode & 0b00000000010000000000000000000000 != 0x0;
			let u = opcode & 0b00000000100000000000000000000000 != 0x0;
			let _p = opcode & 0b00000001000000000000000000000000 != 0x0;

			let offset = match u {
				false => 0x0 - immediate as i16,
				true  => 0x0 + immediate as i16,
			};

			return match l {
				false => StoreImmediateOffset(register, base, offset),
				true  => LoadImmediateOffset( register, base, offset),
			};
		},

		0b011 => {},

		0b100 => {},

		0b101 => {
			let offset = opcode & 0b00000000111111111111111111111111;
			let offset = (offset << 0x8) as i32 >> 0x6;

			let l = opcode & 0b00000001000000000000000000000000 != 0x0;

			return match l {
				false => Branch(    offset),
				true  => BranchLink(offset),
			};
		},

		0b110 => {},

		0b111 => {},

		_ => unsafe { unreachable_unchecked() },
	}

	Error::InvalidArmOpcode(address, opcode).trap();
	return Undefined;
}
