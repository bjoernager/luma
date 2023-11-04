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

#[derive(Clone, Copy)]
pub enum Instruction {
	AddImmediate(                           u8, u8, u32),
	AddRegister(                            u8, u8, u8),
	Branch(                                 i32),
	BranchExchange(                         u8),
	BranchLink(                             i32),
	BranchLinkPrefix(                       i32),
	BranchLinkSuffix(                       i32),
	CompareImmediate(                       u8, u32),
	CompareRegister(                        u8, u8),
	LoadHalfword(                           u8, u8, i16),
	LoadImmediateOffset(                    u8, u8, i16),
	LoadPc(                                 u8, i16),
	MoveImmediate(                          u8, u32),
	MoveImmediateArithmeticShiftRight(      u8, u8, u8),
	MoveImmediateLogicalShiftLeftImmediate( u8, u8, u8),
	MoveImmediateLogicalShiftRightImmediate(u8, u8, u8),
	MoveRegister(                           u8, u8),
	MoveRegisterArithmeticShiftRight(       u8, u8, u8),
	MoveRegisterLogicalShiftLeftImmediate(  u8, u8, u8),
	MoveRegisterLogicalShiftRightImmediate( u8, u8, u8),
	StoreByteImmediateOffset(               u8, u8, i16),
	StoreByteRegisterOffset(                u8, u8, u8),
	StoreHalfword(                          u8, u8, i16),
	StoreImmediateOffset(                   u8, u8, i16),
	SubtractImmediate(                      u8, u8, u32),
	SubtractRegister(                       u8, u8, u8),

	Undefined,
}
