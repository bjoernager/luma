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

use crate::cpu::Cpu;
use crate::instruction::Instruction;

impl Cpu {
	#[inline(always)]
	pub(super) fn execute(&mut self, instruction: Instruction) {
		use Instruction::*;

		match instruction {
			Adc(rd, rn, shifter, s) => self.isa_adc(rd, rn, shifter, s),
			Add(rd, rn, shifter, s) => self.isa_add(rd, rn, shifter, s),
			And(rd, rn, shifter, s) => self.isa_and(rd, rn, shifter, s),
			B(  imm)                => self.isa_b(  imm),
			Bic(rd, rn, shifter, s) => self.isa_bic(rd, rn, shifter, s),
			Bl( imm)                => self.isa_bl( imm),
			Bl0(imm)                => self.isa_bl0(imm),
			Bl1(imm)                => self.isa_bl1(imm),
			Bx( rm)                 => self.isa_bx( rm),
			Cmn(rn, shifter)        => self.isa_cmn(rn, shifter),
			Cmp(rn, shifter)        => self.isa_cmp(rn, shifter),
			Eor(rd, rn, shifter, s) => self.isa_eor(rd, rn, shifter, s),
			Mov(rd, shifter, s)     => self.isa_mov(rd, shifter, s),
			Mul(rd, rn, shifter, s) => self.isa_mul(rd, rn, shifter, s),
			Mvn(rd, shifter, s)     => self.isa_mvn(rd, shifter, s),
			Orr(rd, rn, shifter, s) => self.isa_orr(rd, rn, shifter, s),
			Rsb(rd, rn, shifter, s) => self.isa_rsb(rd, rn, shifter, s),
			Rsc(rd, rn, shifter, s) => self.isa_rsc(rd, rn, shifter, s),
			Sbc(rd, rn, shifter, s) => self.isa_sbc(rd, rn, shifter, s),
			Sub(rd, rn, shifter, s) => self.isa_sub(rd, rn, shifter, s),
			Swi(imm)                => self.isa_swi(imm),
			Teq(rn, shifter)        => self.isa_teq(rn, shifter),
			Tst(rn, shifter)        => self.isa_tst(rn, shifter),

			// Legacy:
			LoadHalfword(            rd, rn, imm) => self.isa_load_halfword(              rd, rn, imm),
			LoadImmediateOffset(     rd, rn, imm) => self.isa_load_immediate_offset(      rd, rn, imm),
			LoadPc(                  rd, imm)     => self.isa_load_pc(                    rd, imm),
			StoreByteImmediateOffset(rd, rn, imm) => self.isa_store_byte_immediate_offset(rd, rn, imm),
			StoreByteRegisterOffset( rd, rn, rm)  => self.isa_store_byte_register_offset( rd, rn, rm),
			StoreHalfword(           rd, rn, imm) => self.isa_store_halfword(             rd, rn, imm),
			StoreImmediateOffset(    rd, rn, imm) => self.isa_store_immediate_offset(     rd, rn, imm),

			Undefined => {},
		};
	}
}
