// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::{MEMSIZ, Reg};
use crate::luma::app::App;

impl App {
	pub fn emu(&mut self) {
		eprintln!("starting emulation");

		// Initialise registers:
		let mut reg = Reg {
			r0:   0x00000000,
			r1:   0x00000000,
			r2:   0x00000000,
			r3:   0x00000000,
			r4:   0x00000000,
			r5:   0x00000000,
			r6:   0x00000000,
			r7:   0x00000000,
			r8:   0x00000000,
			r9:   0x00000000,
			r10:  0x00000000,
			r11:  0x00000000,
			r12:  0x00000000,
			sp:   0x00000000,
			lr:   0x00000000,
			pc:   0x08000008,
			cpsr: 0b00000000000000000000000000001111,
		};

		//let mem = self.mem;                   // For convenience.

		eprintln!("starting at 0x{:08X}",reg.pc - 0x8);

		loop {
			// Check the current address:
			if reg.pc >= MEMSIZ as u32 {
				self.end(0x1, Some(format!("out-of-bounds address {:X}",reg.pc).as_str()));
			}

			// Decode opcode:
			let opc = unsafe { *(self.mem.add((reg.pc - 0x8) as usize) as *mut u32) };

			// Currently, we only support the bal.w instruction.
			if opc & 0b11111111000000000000000000000000 == 0b11101010000000000000000000000000 {
				let off: u32 = opc          & 0b00000000111111111111111111111111;
				let abs: u32 = !(opc - 0x1) & 0b00000000111111111111111111111111;
				
				reg.pc = match (off & 0b00000000100000000000000000000000) != 0x0 { // If negative...
					false => reg.pc + off * 0x4 + 0x8,
					true  => reg.pc - abs * 0x4 + 0x8,
				};
				eprintln!("branch {off:X} => {:X}",reg.pc);

				continue;
			} else {
				self.trp(&reg, opc);
			}

			// Continue:
			reg.pc += 0x4;
		}
	}
}
