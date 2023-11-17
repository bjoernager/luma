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

use std::fmt::{Display, Formatter};

mod extract;

#[derive(Clone, Copy)]
pub enum Shifter {
	Immediate(                    u8, u8),
	ArithmeticShiftRightImmediate(u8, u8),
	ArithmeticShiftRightRegister( u8, u8),
	LogicalShiftLeftImmediate(    u8, u8),
	LogicalShiftLeftRegister(     u8, u8),
	LogicalShiftRightImmediate(   u8, u8),
	LogicalShiftRightRegister(    u8, u8),
	RotateRightImmediate(         u8, u8),
	RotateRightRegister(          u8, u8),
	RotateRightExtend(            u8),
}

impl Shifter {
	#[inline(always)]
	pub const fn from_register(register: u8) -> Shifter { Shifter::LogicalShiftLeftImmediate(register, 0x0) }
}

impl Display for Shifter {
	#[must_use]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		use Shifter::*;

		return match *self {
			Immediate(                    imm, rot) => write!(f, "#{:#X}", (imm as u32).rotate_right(rot as u32)),
			ArithmeticShiftRightImmediate(rm, imm)  => write!(f, "r{rm}, ASR #{imm:#X}"),
			ArithmeticShiftRightRegister( rm, rs)   => write!(f, "r{rm}, ASR r{rs}"),
			LogicalShiftLeftImmediate(    rm, imm)  => write!(f, "r{rm}, LSL #{imm:#X}"),
			LogicalShiftLeftRegister(     rm, rs)   => write!(f, "r{rm}, LSL r{rs}"),
			LogicalShiftRightImmediate(   rm, imm)  => write!(f, "r{rm}, LSR #{imm:#X}"),
			LogicalShiftRightRegister(    rm, rs)   => write!(f, "r{rm}, LSR r{rs}"),
			RotateRightImmediate(         rm, imm)  => write!(f, "r{rm}, ROR #{imm:#X}"),
			RotateRightRegister(          rm, rs)   => write!(f, "r{rm}, ROR r{rs}"),
			RotateRightExtend(            rm)       => write!(f, "RXX"),
		};
	}
}
