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

use crate::shifter::Shifter;

use std::hint::unreachable_unchecked;

impl Shifter {
	pub fn extract(opcode: u32) -> Self {
		use Shifter::*;

		match opcode & 0b00000010000000000000000000000000 != 0x0 {
			false => {
				let rm = (opcode & 0b00000000000000000000000000001111) as u8;

				let shift          = (opcode & 0b00000000000000000000000001100000).wrapping_shr(0x5) as u8;
				let register_shift = opcode & 0b00000000000000000000000000010000 != 0x0;

				match register_shift {
					false => {
						let imm = (opcode & 0b00000000000000000000111110000000).wrapping_shr(0x7) as u8;

						return match shift {
							0b11 => match imm {
								0x0 => RotateRightExtend(   rm),
								imm => RotateRightImmediate(rm, imm),
							},

							shift => {
								match shift {
									0b00 => LogicalShiftLeftImmediate(rm, imm),

									0b01 => LogicalShiftRightImmediate(rm, match imm {
										0x0 => 0x20,
										imm => imm,
									}),

									0b10 => ArithmeticShiftRightImmediate(rm, match imm {
										0x0 => 0x20,
										imm => imm,
									}),

									_ => unsafe { unreachable_unchecked() },
								}
							},
						};
					},

					true => {
						let rs = (opcode & 0b0000000000000000000111100000000).wrapping_shr(0x8) as u8;

						return match shift {
							0b00 => LogicalShiftLeftRegister(    rm, rs),
							0b01 => LogicalShiftRightRegister(   rm, rs),
							0b10 => ArithmeticShiftRightRegister(rm, rs),
							0b11 => RotateRightRegister(         rm, rs),

							_ => unsafe { unreachable_unchecked() },
						};
					},
				};
			},

			true => {
				let imm = (opcode & 0b00000000000000000000000011111111) as u8;
				let rot = (opcode & 0b00000000000000000000111100000000).wrapping_shr(0x7) as u8;

				return Immediate(imm, rot);
			},
		};
	}
}
