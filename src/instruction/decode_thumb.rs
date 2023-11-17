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
	pub fn decode_thumb(opcode: u16) -> (Self, Predicate) {
		use Instruction::*;
		use Predicate::*;
		use Shifter::*;

		return match (opcode & 0b1110000000000000).wrapping_shr(0xD) {
			0b000 => {
				match (opcode & 0b0001100000000000).wrapping_shr(0xB) {
					0b11 => {
						match (opcode & 0b0000010000000000).wrapping_shr(0xA) {
							0b0 => {
								let rd = (opcode & 0b0000000000000111) as u8;
								let rn = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;
								let rm = ((opcode & 0b0000000111000000).wrapping_shr(0x6)) as u8;

								match (opcode & 0b0000001000000000).wrapping_shr(0x9) {
									0b0 => (Add(rd, rn, Shifter::from_register(rm), true), Al),
									0b1 => (Sub(rd, rn, Shifter::from_register(rm), true), Al),

									_ => unsafe { unreachable_unchecked() },
								}
							},

							0b1 => (Undefined, Al),

							_ => unsafe { unreachable_unchecked() },
						}
					},

					pattern => {
						let rd  = (opcode & 0b0000000000000111) as u8;
						let rm  = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;
						let imm = ((opcode & 0b0000011111000000).wrapping_shr(0x6)) as u8;

						match pattern {
							0b00 => (Mov(rd, LogicalShiftLeftImmediate(rm, imm), true), Al),
							0b01 => (Mov(rd, LogicalShiftRightImmediate(rm, imm), true), Al),
							0b10 => (Mov(rd, ArithmeticShiftRightImmediate(rm, imm), true), Al),

							_ => unsafe { unreachable_unchecked() },
						}
					},
				}
			},

			0b001 => {
				let imm = (opcode & 0b0000000011111111) as u8;
				let rd  = ((opcode & 0b0000111100000000).wrapping_shr(0x8)) as u8;

				match (opcode & 0b0001100000000000).wrapping_shr(0xB) {
					0b00 => (Mov(rd, Immediate(imm, 0x0), true), Al),
					0b01 => (Cmp(rd, Immediate(imm, 0x0)), Al),
					0b10 => (Add(rd, rd, Immediate(imm, 0x0), true), Al),
					0b11 => (Sub(rd, rd, Immediate(imm, 0x0), true), Al),

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b010 => {
				match (opcode & 0b0001000000000000).wrapping_shr(0xC) {
					0b0 => {
						match (opcode & 0b0000100000000000).wrapping_shr(0xB) {
							0b0 => {
								match (opcode & 0b0000010000000000).wrapping_shr(0xA) {
									0b0 => {
										let rd = (opcode & 0b0000000000000111) as u8;
										let rm = (opcode & 0b0000000000111000).wrapping_shr(0x3) as u8;

										match (opcode & 0b0000001111000000).wrapping_shr(0x6) {
											0b0000 => (And(rd, rd, Shifter::from_register(rm), true), Al),
											0b0001 => (Eor(rd, rd, Shifter::from_register(rm), true), Al),
											0b0010 => (Mov(rd, LogicalShiftLeftRegister(rd, rm), true), Al),
											0b0011 => (Mov(rd, LogicalShiftRightRegister(rd, rm), true), Al),
											0b0100 => (Mov(rd, ArithmeticShiftRightRegister(rd, rm), true), Al),
											0b0101 => (Adc(rd, rd, Shifter::from_register(rm), true), Al),
											0b0110 => (Sbc(rd, rd, Shifter::from_register(rm), true), Al),
											0b0111 => (Mov(rd, RotateRightRegister(rd, rm), true), Al),
											0b1000 => (Tst(rd, Shifter::from_register(rm)), Al),
											0b1001 => (Rsb(rd, rm, Immediate(0x0, 0x0), true), Al),
											0b1010 => (Cmp(rd, Shifter::from_register(rm)), Al),
											0b1011 => (Cmn(rd, Shifter::from_register(rm)), Al),
											0b1100 => (Orr(rd, rd, Shifter::from_register(rm), true), Al),
											0b1101 => (Mul(rd, rd, Shifter::from_register(rm), true), Al),
											0b1110 => (Bic(rd, rd, Shifter::from_register(rm), true), Al),
											0b1111 => (Mvn(rd, Shifter::from_register(rm), true), Al),

											_ => unsafe { unreachable_unchecked() },
										}
									},

									0b1 => {
										let h0 = opcode & 0b0000000010000000 != 0x0;
										let h1 = opcode & 0b0000000001000000 != 0x0;

										let rd = (opcode & 0b0000000000000111) as u8 | (h0 as u8) << 0x3;
										let rm = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8 | (h1 as u8) << 0x3;

										match (opcode & 0b0000001100000000).wrapping_shr(0x8) {
											0b00 => (Undefined, Al),

											// Unpredictable if rd is pc or if both rd and rm
											// are low registers.
											0b01 => (Cmp(rd, Shifter::from_register(rm)), Al),
											0b10 => (Mov(rd, Shifter::from_register(rm), true), Al),
											// Unpredictable if h0 is true or if rd is non-
											// zero.
											0b11 => (Bx(rm), Al),

											_ => unsafe { unreachable_unchecked() },
										}
									},

									_ => unsafe { unreachable_unchecked() },
								}
							},

							0b1 => {
								let imm = (opcode & 0b0000000011111111) as u8;
								let rd  = ((opcode & 0b0000011100000000).wrapping_shr(0x8)) as u8;

								let imm = (imm as u16).wrapping_mul(0x4);

								(LoadPc(rd, imm), Al)
							},

							_ => unsafe { unreachable_unchecked() },
						}
					},

					0b1 => (Undefined, Al),

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b011 => {
				let rd  = (opcode & 0b0000000000000111) as u8;
				let rn  = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;
				let imm = (opcode & 0b0000011111000000).wrapping_shr(0x6);
				let l   = opcode & 0b0000100000000000 != 0x0;
				let b   = opcode & 0b0001000000000000 != 0x0;

				let imm = imm.wrapping_shl(0x2) as i16;

				match l {
					false => match b {
						false => (StoreImmediateOffset(    rd, rn, imm), Al),
						true  => (StoreByteImmediateOffset(rd, rn, imm), Al),
					},

					true => match b {
						false => (LoadImmediateOffset(    rd, rn, imm), Al),
						true  => (StoreByteImmediateOffset(rd, rn, imm), Al), // TODO
					},
				}
			},

			0b100 => {
				match (opcode & 0b0001000000000000).wrapping_shr(0xC) {
					0b0 => {
						let rd  = (opcode & 0b0000000000000111) as u8;
						let rn  = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;
						let imm = (opcode & 0b0000011111000000).wrapping_shr(0x6) as u8;
						let l   = opcode & 0b0000010000000000 != 0x0;

						let imm = imm.wrapping_shl(0x1) as i16;

						match l {
							false => (StoreHalfword(rd, rn, imm), Al),
							true  => (LoadHalfword( rd, rn, imm), Al),
						}
					},

					0b1 => (Undefined, Al),

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b101 => (Undefined, Al),

			0b110 => {
				match (opcode & 0b0001000000000000).wrapping_shr(0xC) {
					0b0 => (Undefined, Al),

					0b1 => {
						let predicate = ((opcode & 0b0000111100000000).wrapping_shr(0x8)) as u8;

						let imm = opcode & 0b0000000011111111;

						match predicate {
							0b1111 => (Swi(imm as u32), Al),

							_ => {
								let predicate = Predicate::from_raw(predicate);
								let imm       = ((imm as u32) << 0x18) as i32 >> 0x17;

								(B(imm), predicate)
							},
						}
					},

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b111 => {
				let imm = (opcode & 0b0000011111111111) as u32;

				match (opcode & 0b0001100000000000).wrapping_shr(0xB) {
					0b00 => {
						let imm = imm.wrapping_shl(0x15) as i32 >> 0x14;
						(B(imm), Al)
					},

					// Undefined in ARMv4 (later blx suffix).
					0b01 => (Undefined, Al),

					0b10 => {
						let imm = imm.wrapping_shl(0x15) as i32 >> 0x9;
						(Bl0(imm), Al)
					},

					0b11 => {
						let imm = imm.wrapping_shl(0x1) as i32;
						(Bl1(imm), Al)
					},

					_ => unsafe { unreachable_unchecked() },
				}
			},

			_ => unsafe { unreachable_unchecked() },
		};
	}
}
