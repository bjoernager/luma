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
use crate::luma::cpu::{Cpu, test_predicate};
use crate::luma::instruction::Instruction;

use std::hint::unreachable_unchecked;

impl Cpu {
	pub(super) fn decode_thumb(&mut self) -> Instruction {
		use Instruction::*;

		let state = self.state.lock().unwrap();

		let address = state.read_register(0xF).wrapping_sub(0x4);
		let opcode  = state.read_halfword(address);

		let cpsr = state.read_cpsr();

		drop(state);

		log(&format!("{opcode:#018b}                 @ {address:#010X} - ({})", self.cycle));

		match (opcode & 0b1110000000000000).wrapping_shr(0xD) {
			0b000 => {
				match (opcode & 0b0001100000000000).wrapping_shr(0xB) {
					0b11 => {
						match (opcode & 0b0000010000000000).wrapping_shr(0xA) {
							0b0 => {
								let destination = (opcode & 0b0000000000000111) as u8;

								let base = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;

								let register = ((opcode & 0b0000000111000000).wrapping_shr(0x6)) as u8;

								match (opcode & 0b0000001000000000).wrapping_shr(0x9) {
									0b0 => return AddRegister(destination, base, register),

									0b1 => return SubtractRegister(destination, base, register),

									_ => unsafe { unreachable_unchecked() },
								}
							},

							0b1 => {
							},

							_ => unsafe { unreachable_unchecked() },
						}
					},

					pattern => {
						let destination = (opcode & 0b0000000000000111) as u8;

						let base = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;

						let immediate = ((opcode & 0b0000011111000000).wrapping_shr(0x6)) as u8;

						match pattern {
							0b00 => return MoveImmediateLogicalShiftLeftImmediate(destination, base, immediate),

							0b01 => return MoveImmediateLogicalShiftRightImmediate(destination, base, immediate),

							0b10 => return MoveImmediateArithmeticShiftRight(destination, base, immediate),

							_ => unsafe { unreachable_unchecked() },
						}
					},
				}
			},

			0b001 => {
				let immediate = (opcode & 0b0000000011111111) as u32;

				let destination = ((opcode & 0b0000111100000000).wrapping_shr(0x8)) as u8;

				match (opcode & 0b0001100000000000).wrapping_shr(0xB) {
					0b00 => return MoveImmediate(destination, immediate),

					0b01 => return CompareImmediate(destination, immediate),

					0b10 => return AddImmediate(destination, destination, immediate),

					0b11 => return SubtractImmediate(destination, destination, immediate),

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
										let destination = (opcode & 0b0000000000000111) as u8;

										let base = (opcode & 0b0000000000111000).wrapping_shr(0x3) as u8;

										match (opcode & 0b0000001111000000).wrapping_shr(0x6) {
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

											0b1010 => return CompareRegister(destination, base),

											0b1011 => {},

											0b1100 => {},

											0b1101 => {},

											0b1110 => {},

											0b1111 => {},

											_ => unsafe { unreachable_unchecked() },
										}
									},

									0b1 => {
										let h0 = opcode & 0b0000000010000000 != 0x0;
										let h1 = opcode & 0b0000000001000000 != 0x0;

										let destination = (opcode & 0b0000000000000111) as u8;
										let destination = destination | (h0 as u8) << 0x3;

										let source = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;
										let source = source | (h1 as u8) << 0x3;

										match (opcode & 0b0000001100000000).wrapping_shr(0x8) {
											0b00 => {},

											// Unpredictable if destination is pc or if both it
											// and source are low registers.
											0b01 => return CompareRegister(destination, source),

											0b10 => return MoveRegister(destination, source),

											// Unpredictable if h0 is true or if destination is
											// non-zero.
											0b11 => return BranchExchange(source),

											_ => unsafe { unreachable_unchecked() },
										}
									},

									_ => unsafe { unreachable_unchecked() },
								}
							},

							0b1 => {
								let immediate = opcode & 0b0000000011111111;

								let destination = ((opcode & 0b0000011100000000).wrapping_shr(0x8)) as u8;

								let offset = (immediate as i16).wrapping_mul(0x4);

								return LoadPc(destination, offset);
							},

							_ => unsafe { unreachable_unchecked() },
						}
					},

					0b1 => {},

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b011 => {
				let register = (opcode & 0b0000000000000111) as u8;

				let base = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;

				let immediate = (opcode & 0b0000011111000000).wrapping_shr(0x6);

				let _l = opcode & 0b0000100000000000 != 0x0;
				let b = opcode & 0b0001000000000000 != 0x0;

				let offset = immediate.wrapping_shl(0x2) as i16;

				return match b {
					false => StoreImmediateOffset(    register, base, offset),
					true  => StoreByteImmediateOffset(register, base, offset),
				};
			},

			0b100 => {
				match (opcode & 0b0001000000000000).wrapping_shr(0xC) {
					0b0 => {
						let register = (opcode & 0b0000000000000111) as u8;

						let base = ((opcode & 0b0000000000111000).wrapping_shr(0x3)) as u8;

						let immediate = (opcode & 0b0000011111000000).wrapping_shr(0x6);

						let l = opcode & 0b0000010000000000 != 0x0;

						let offset = immediate.wrapping_shl(0x1) as i16;

						return match l {
							false => StoreHalfword(register, base, offset),
							true  => LoadHalfword( register, base, offset),
						};
					},

					0b1 => {},

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b101 => {
			},

			0b110 => {
				match (opcode & 0b0001000000000000).wrapping_shr(0xC) {
					0b0 => {},

					0b1 => {
						let predicate = ((opcode & 0b0000111100000000).wrapping_shr(0x8)) as u8;

						if !test_predicate!(cpsr, predicate) { return Undefined };

						let immediate = opcode & 0b0000000011111111;

						let offset = ((immediate as u32) << 0x18) as i32 >> 0x17;

						return Branch(offset);
					},

					_ => unsafe { unreachable_unchecked() },
				}
			},

			0b111 => {
				let immediate = (opcode & 0b0000011111111111) as u32;

				match (opcode & 0b0001100000000000).wrapping_shr(0xB) {
					0b00 => {
						let offset = immediate.wrapping_shl(0x15) as i32 >> 0x14;

						return Branch(offset);
					},

					// Undefined in ARMv4 (later blx suffix).
					0b01 => {},

					0b10 => {
						let offset = immediate.wrapping_shl(0x15) as i32 >> 0x9;

						return BranchLinkPrefix(offset);
					},

					0b11 => {
						let offset = immediate.wrapping_shl(0x1) as i32;

						return BranchLinkSuffix(offset);
					},

					_ => unsafe { unreachable_unchecked() },
				}
			},

			_ => unsafe { unreachable_unchecked() },
		}

		Error::InvalidThumbOpcode(address, opcode).trap();
		return Undefined;
	}
}
