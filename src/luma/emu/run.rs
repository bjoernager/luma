// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::MEMSIZ;
use crate::luma::app::GOTSIG;
use crate::luma::emu::Emu;

use std::sync::atomic::Ordering;

impl Emu {
	pub fn run(&mut self) {
		eprintln!("starting emulation");

		eprintln!("starting at 0x{:08X}",self.reg.pc - 0x8);

		loop {
			// Check if we have recieved a signal:
			if unsafe { GOTSIG.load(Ordering::Relaxed) } {
				eprintln!("got interrupt");
				break;
			}

			// Check the current address:
			if self.reg.pc >= MEMSIZ as u32 {
				self.trpadr(self.reg.pc);
			}

			// Decode opcode:
			let opc = unsafe { *(self.mem.add((self.reg.pc - 0x8) as usize) as *mut u32) };
			self.opc(opc);

			// Continue:
			self.reg.pc += 0x4;
		}
	}
}
