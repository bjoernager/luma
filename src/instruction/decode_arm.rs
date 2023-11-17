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

use crate::instruction::Instruction;
use crate::predicate::Predicate;
use crate::shifter::Shifter;

use std::hint::unreachable_unchecked;

impl Instruction {
	#[must_use]
	pub fn decode_arm(opcode: u32) -> (Self, Predicate) {
		use Instruction::*;
		use Predicate::*;

		let predicate = (opcode & 0b11110000000000000000000000000000).wrapping_shr(0x1C) as u8;
		let predicate = Predicate::from_raw(predicate);

		return match (opcode & 0b00001110000000000000000000000000).wrapping_shr(0x19) {
			0b000 => {
				match (opcode & 0b00000000000000000000000000010000).wrapping_shr(0x4) {
					0b0 => {
						let rd = (opcode & 0b00000000000000001111000000000000).wrapping_shr(0xC) as u8;
						let rn = (opcode & 0b00000000000011110000000000000000).wrapping_shr(0x10) as u8;
						let s  =  opcode & 0b00000000000100000000000000000000 != 0x0;

						let shifter = Shifter::extract(opcode);

						match (opcode & 0b00000001111000000000000000000000).wrapping_shr(0x15) {
							0b0000 => (And(rd, rn, shifter, s), predicate),
							0b0001 => (Eor(rd, rn, shifter, s), predicate),
							0b0010 => (Sub(rd, rn, shifter,s), predicate),
							0b0011 => (Rsb(rd, rn, shifter, s), predicate),
							0b0100 => (Add(rd, rn, shifter, s), predicate),
							0b0101 => (Adc(rd, rn, shifter, s), predicate),
							0b0110 => (Sbc(rd, rn, shifter, s), predicate),
							0b0111 => (Rsc(rd, rn, shifter, s), predicate),
							0b1000 => (Tst(rd, shifter), predicate),
							0b1001 => (Teq(rd, shifter), predicate),
							0b1010 => (Cmp(rd, shifter), predicate),
							0b1011 => (Cmn(rd, shifter), predicate),
							0b1100 => (Orr(rd, rn, shifter, s), predicate),
							0b1101 => (Mov(rd, shifter, s), predicate),
							0b1110 => (Bic(rd, rn, shifter, s), predicate),
							0b1111 => (Mvn(rd, shifter, s), predicate),

							_ => unsafe { unreachable_unchecked() },
						}
					},

					0b1 => {
						match (opcode & 0b00000000000000000000000010000000).wrapping_shr(0x7) {
							0b0 => {
								match opcode & 0b00000001100100000000000000000000 {
									0b00000001000000000000000000000000 => {
										let rm = (opcode & 0b00000000000000000000000000001111) as u8;

										match (opcode & 0b00000000011000000000000000000000).wrapping_shr(0x13) | (opcode & 0b00000000000000000000000001100000).wrapping_shr(0x5) {
											0b0000 => (Undefined, Al),
											0b0001 => (Undefined, Al),
											0b0010 => (Undefined, Al),
											0b0011 => (Undefined, Al),
											0b0100 => (Bx(rm), predicate),
											0b0101 => (Undefined, Al),
											0b0110 => (Undefined, Al),
											0b0111 => (Undefined, Al),
											0b1000 => (Undefined, Al),
											0b1001 => (Undefined, Al),
											0b1010 => (Undefined, Al),
											0b1011 => (Undefined, Al),
											0b1100 => (Undefined, Al),
											0b1101 => (Undefined, Al),
											0b1110 => (Undefined, Al),
											0b1111 => (Undefined, Al),

											_ => unsafe { unreachable_unchecked() },
										}
									},

									_ => (Undefined, Al),
								}
							},

							0b1 => (Undefined, Al),

							_ => unsafe { unreachable_unchecked() },
						}
					},

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b001 => {
				let rd = (opcode & 0b00000000000000001111000000000000).wrapping_shr(0xC) as u8;
				let rn = (opcode & 0b00000000000011110000000000000000).wrapping_shr(0x10) as u8;
				let s  = opcode & 0b00000000000100000000000000000000 != 0x0;

				let shifter = Shifter::extract(opcode);

				match (opcode & 0b00000001111000000000000000000000).wrapping_shr(0x15) {
					0b0000 => (And(rd, rn, shifter, s), predicate),
					0b0001 => (Eor(rd, rn, shifter, s), predicate),
					0b0010 => (Sub(rd, rn, shifter, s), predicate),
					0b0011 => (Rsb(rd, rn, shifter, s), predicate),
					0b0100 => (Add(rd, rn, shifter, s), predicate),
					0b0101 => (Adc(rd, rn, shifter, s), predicate),
					0b0110 => (Sbc(rd, rn, shifter, s), predicate),
					0b0111 => (Rsc(rd, rn, shifter, s), predicate),
					0b1000 => (Tst(rn, shifter), predicate),
					0b1001 => (Teq(rn, shifter), predicate),
					0b1010 => (Cmp(rn, shifter), predicate),
					0b1011 => (Cmn(rn, shifter), predicate),
					0b1100 => (Orr(rd, rn, shifter, s), predicate),
					0b1101 => (Mov(rd, shifter, s), predicate),
					0b1110 => (Bic(rd, rn, shifter, s), predicate),
					0b1111 => (Mvn(rd, shifter, s), predicate),

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b010 => {
				let imm = (opcode & 0b00000000000000000000111111111111) as u16;

				let register = (opcode & 0b00000000000000001111000000000000).wrapping_shr(0xC) as u8;

				let base = (opcode & 0b00000000000011110000000000000000).wrapping_shr(0x10) as u8;

				let l = opcode & 0b00000000000100000000000000000000 != 0x0;
				let _w = opcode & 0b00000000001000000000000000000000 != 0x0;
				let _b = opcode & 0b00000000010000000000000000000000 != 0x0;
				let u = opcode & 0b00000000100000000000000000000000 != 0x0;
				let _p = opcode & 0b00000001000000000000000000000000 != 0x0;

				let imm = match u {
					false => 0x0 - imm as i16,
					true  =>       imm as i16,
				};

				match l {
					false => (StoreImmediateOffset(register, base, imm), predicate),
					true  => (LoadImmediateOffset( register, base, imm), predicate),
				}
			},

			0b011 => (Undefined, Al),

			0b100 => (Undefined, Al),

			0b101 => {
				let offset = opcode & 0b00000000111111111111111111111111;
				let offset = (offset << 0x8) as i32 >> 0x6;

				let l = opcode & 0b00000001000000000000000000000000 != 0x0;

				match l {
					false => (B(offset), predicate),
					true  => (Bl(offset), predicate),
				}
			},

			0b110 => (Undefined, Al),

			0b111 => {
				match (opcode & 0b00000001000000000000000000000000).wrapping_shr(0x18) {
					0b0 => (Undefined, Al),

					0b1 => {
						let imm = opcode & 0b00000000111111111111111111111111;
						(Swi(imm), predicate)
					},

					_ => unsafe { unreachable_unchecked() },
				}
			},

			_ => unsafe { unreachable_unchecked() },
		};
	}
}
