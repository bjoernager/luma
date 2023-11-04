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
			AddImmediate(                           destination, base, immediate) => self.add_immediate(                        destination, base, immediate),
			AddRegister(                            destination, base, immediate) => self.add_register(                         destination, base, immediate),
			Branch(                                 offset)                       => self.branch(                               offset),
			BranchExchange(                         source)                       => self.branch_exchange(                      source),
			BranchLink(                             offset)                       => self.branch_link(                          offset),
			BranchLinkPrefix(                       offset)                       => self.branch_link_prefix(                   offset),
			BranchLinkSuffix(                       offset)                       => self.branch_link_suffix(                   offset),
			CompareImmediate(                       register, immediate)          => self.compare_immediate(                    register, immediate),
			CompareRegister(                        left, right)                  => self.compare_register(                     left, right),
			LoadHalfword(                           destination, base, offset)    => self.load_halfword(                        destination, base, offset),
			LoadImmediateOffset(                    destination, base, offset)    => self.load_immediate_offset(                destination, base, offset),
			LoadPc(                                 destination, offset)          => self.load_pc(                              destination, offset),
			MoveImmediate(                          destination, immediate)       => self.move_immediate(                       destination, immediate),
			MoveImmediateArithmeticShiftRight(      destination, base, shift)     => self.move_immediate_arithmetic_shift_right(destination, base, shift),
			MoveImmediateLogicalShiftLeftImmediate( destination, base, shift)     => self.move_immediate_logical_shift_left(    destination, base, shift),
			MoveImmediateLogicalShiftRightImmediate(destination, base, shift)     => self.move_immediate_logical_shift_right(   destination, base, shift),
			MoveRegister(                           destination, source)          => self.move_register(                        destination, source),
			MoveRegisterArithmeticShiftRight(       destination, base, shift)     => self.move_register_arithmetic_shift_right( destination, base, shift),
			MoveRegisterLogicalShiftLeftImmediate(  destination, base, shift)     => self.move_register_logical_shift_left(     destination, base, shift),
			MoveRegisterLogicalShiftRightImmediate( destination, base, shift)     => self.move_register_logical_shift_right(    destination, base, shift),
			StoreByteImmediateOffset(               source, base, offset)         => self.store_byte_immediate_offset(          source, base, offset),
			StoreByteRegisterOffset(                source, base, offset)         => self.store_byte_register_offset(           source, base, offset),
			StoreHalfword(                          source, base, offset)         => self.store_halfword(                       source, base, offset),
			StoreImmediateOffset(                   source, base, offset)         => self.store_immediate_offset(               source, base, offset),
			SubtractImmediate(                      destination, base, immediate) => self.subtract_immediate(                    destination, base, immediate),
			SubtractRegister(                       destination, base, immediate) => self.subtract_register(                     destination, base, immediate),

			Undefined => {},
		};

		self.r#continue();
	}
}
