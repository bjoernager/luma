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

mod decode_arm;
mod decode_thumb;

#[derive(Clone, Copy)]
pub enum Instruction {
	Adc(u8, u8, Shifter, bool),
	Add(u8, u8, Shifter, bool),
	And(u8, u8, Shifter, bool),
	B(  i32),
	Bic(u8, u8, Shifter, bool),
	Bl( i32),
	Bl0(i32),
	Bl1(i32),
	Bx( u8),
	Cmn(u8 ,Shifter),
	Cmp(u8, Shifter),
	Eor(u8, u8, Shifter, bool),
	Mov(u8, Shifter, bool),
	Mul(u8, u8, Shifter, bool),
	Mvn(u8, Shifter, bool),
	Orr(u8, u8, Shifter, bool),
	Rsb(u8, u8, Shifter, bool),
	Rsc(u8, u8, Shifter, bool),
	Sbc(u8, u8, Shifter, bool),
	Sub(u8, u8, Shifter, bool),
	Swi(u32),
	Teq(u8, Shifter),
	Tst(u8, Shifter),

	// Rework in next release:
	LoadHalfword(            u8, u8, i16),
	LoadImmediateOffset(     u8, u8, i16),
	LoadPc(                  u8, u16),
	StoreByteImmediateOffset(u8, u8, i16),
	StoreByteRegisterOffset( u8, u8, u8),
	StoreHalfword(           u8, u8, i16),
	StoreImmediateOffset(    u8, u8, i16),

	Undefined,
}
