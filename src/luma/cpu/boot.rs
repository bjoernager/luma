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

use crate::luma::cpu::Cpu;
use crate::luma::cpu_handle::CpuHandle;
use crate::luma::instruction::Instruction;

use std::sync::atomic::Ordering;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

impl Cpu {
	pub fn boot(self) -> CpuHandle {
		let state = self.state.lock().unwrap();

		eprintln!("starting emulation at {:#010X}", state.read_register(0xF).wrapping_sub(0x8));

		drop(state);

		let state = self.state.clone();
		let dead  = self.dead.clone();

		let handle = spawn(move || { self.run() });

		return CpuHandle::new(
			state,
			dead,

			handle,
		);
	}

	fn run(mut self) {
		let run_timer = Instant::now();

		'main_loop: loop {
			if self.dead.load(Ordering::Relaxed) { break 'main_loop };

			let instruction = self.decode();
			self.execute(instruction);

			if cfg!(debug_assertions) { sleep(Duration::from_millis(125)) };

			self.cycle += 0x1;
		}

		let frequency = self.cycle as f64 / run_timer.elapsed().as_micros() as f64;
		eprintln!("emulated {} cycle(s) ({frequency:.9} MHz)", self.cycle);
	}

	#[inline(always)]
	fn execute(&mut self, instruction: Instruction) {
		use Instruction::*;

		match instruction {
			// Arithmetic:
			AddCarryRegister(        rd, rn, rm)  => self.isa_add_carry_register(        rd, rn, rm),
			AddImmediate(            rd, rn, imm) => self.isa_add_immediate(             rd, rn, imm),
			AddRegister(             rd, rn, rm)  => self.isa_add_register(              rd, rn, rm),
			MultiplyRegister(        rd, rm, rs)  => self.isa_multiply_register(         rd, rm, rs),
			ReverseSubtractImmediate(rd, rm, imm) => self.isa_reverse_subtract_immediate(rd, rm, imm),
			SubtractCarryRegister(   rd, rn, rm)  => self.isa_subtract_carry_register(   rd, rn, rm),
			SubtractImmediate(       rd, rn, imm) => self.isa_subtract_immediate(        rd, rn, imm),
			SubtractRegister(        rd, rn, rm)  => self.isa_subtract_register(         rd, rn, rm),

			// Bitwise:
			AndRegister(        rd, rn, rm) => self.isa_and_register(         rd, rn, rm),
			BitClearRegister(   rd, rn, rm) => self.isa_bit_clear_register(   rd, rn, rm),
			LogicalOrRegister(  rd, rn, rm) => self.isa_logical_or_register(  rd, rn, rm),
			ExclusiveOrRegister(rd, rn, rm) => self.isa_exclusive_or_register(rd, rn, rm),

			// Branch:
			Branch(          imm) => self.isa_branch(            imm),
			BranchExchange(  rm)  => self.isa_branch_exchange(   rm),
			BranchLink(      imm) => self.isa_branch_link(       imm),
			BranchLinkPrefix(imm) => self.isa_branch_link_prefix(imm),
			BranchLinkSuffix(imm) => self.isa_branch_link_suffix(imm),

			// Logic:
			CompareImmediate(       rn, imm) => self.isa_compare_immediate(        rn, imm),
			CompareNegativeRegister(rn, rm)  => self.isa_compare_negative_register(rn, rm),
			CompareRegister(        rn, rm)  => self.isa_compare_register(         rn, rm),
			TestRegister(           rn, rm)  => self.isa_test_register(            rn, rm),

			// Memory:
			LoadHalfword(            rd, rn, imm) => self.isa_load_halfword(              rd, rn, imm),
			LoadImmediateOffset(     rd, rn, imm) => self.isa_load_immediate_offset(      rd, rn, imm),
			LoadPc(                  rd, rn)      => self.isa_load_pc(                    rd, rn),
			StoreByteImmediateOffset(rd, rn, imm) => self.isa_store_byte_immediate_offset(rd, rn, imm),
			StoreByteRegisterOffset( rd, rn, rm)  => self.isa_store_byte_register_offset( rd, rn, rm),
			StoreHalfword(           rd, rn, imm) => self.isa_store_halfword(             rd, rn, imm),
			StoreImmediateOffset(    rd, rn, imm) => self.isa_store_immediate_offset(     rd, rn, imm),

			// Move:
			MoveImmediate(                          rd, imm)    => self.isa_move_immediate(                       rd, imm),
			MoveImmediateArithmeticShiftRight(      rd, rm, rs) => self.isa_move_immediate_arithmetic_shift_right(rd, rm, rs),
			MoveImmediateLogicalShiftLeftImmediate( rd, rm, rs) => self.isa_move_immediate_logical_shift_left(    rd, rm, rs),
			MoveImmediateLogicalShiftRightImmediate(rd, rm, rs) => self.isa_move_immediate_logical_shift_right(   rd, rm, rs),
			MoveNotRegister(                        rd, rm)     => self.isa_move_not_register(                    rd, rm),
			MoveRegister(                           rd, rm)     => self.isa_move_register(                        rd, rm),
			MoveRegisterArithmeticShiftRight(       rd, rm, rs) => self.isa_move_register_arithmetic_shift_right( rd, rm, rs),
			MoveRegisterLogicalShiftLeftImmediate(  rd, rm, rs) => self.isa_move_register_logical_shift_left(     rd, rm, rs),
			MoveRegisterLogicalShiftRightImmediate( rd, rm, rs) => self.isa_move_register_logical_shift_right(    rd, rm, rs),
			MoveRegisterRotateRight(                rd, rm, rs) => self.isa_move_register_rotate_right(           rd, rm, rs),

			Undefined => {},
		};

		self.r#continue();
	}
}
