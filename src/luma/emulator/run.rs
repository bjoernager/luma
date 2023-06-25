// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::GOT_SIGNAL;
use crate::luma::emulator::Emulator;

use std::sync::atomic::Ordering;

impl Emulator {
	pub fn run(&mut self) {
		eprintln!("starting emulation");

		eprintln!("starting at 0x{:08X}",self.registers[0xF] - 0x8);

		loop {
			// Check if we have recieved a signal:
			if unsafe { GOT_SIGNAL.load(Ordering::Relaxed) } {
				eprintln!("got interrupt");
				break;
			}

			// Decode opcode:
			let opcode = self.read_word(self.registers[0xF] - 0x8);
			self.opcode(opcode);

			// Continue:
			self.registers[0xF] += 0x4;
		}
	}
}
