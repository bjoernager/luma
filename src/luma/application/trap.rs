// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::{MEMORY_SIZE, TrapKind};
use crate::luma::application::Application;

impl Application {
	pub fn trap(&mut self, kind: TrapKind) {
		let message = match kind {
			TrapKind::BadAlignment( address, alignment) => format!("bad alignment of address 0x{address:08X} (should be {alignment}-byte aligned)"),
			TrapKind::InvalidOpcode(address, opcode)    => format!("invalid opcode 0x{opcode:08X} at 0x{address:08X}"),
			TrapKind::OutOfBounds(  address)            => format!("out-of-bounds address 0x{address:08X} (limit is 0x{MEMORY_SIZE:08X})"),
		};

		eprintln!("trap - {message}");
		eprintln!();
		eprintln!("\tr0:   {:08X}",  self.registers[0x0]);
		eprintln!("\tr1:   {:08X}",  self.registers[0x1]);
		eprintln!("\tr2:   {:08X}",  self.registers[0x2]);
		eprintln!("\tr3:   {:08X}",  self.registers[0x3]);
		eprintln!("\tr4:   {:08X}",  self.registers[0x4]);
		eprintln!("\tr5:   {:08X}",  self.registers[0x5]);
		eprintln!("\tr6:   {:08X}",  self.registers[0x6]);
		eprintln!("\tr7:   {:08X}",  self.registers[0x7]);
		eprintln!("\tr8:   {:08X}",  self.registers[0x8]);
		eprintln!("\tr9:   {:08X}",  self.registers[0x9]);
		eprintln!("\tr10:  {:08X}",  self.registers[0xA]);
		eprintln!("\tr11:  {:08X}",  self.registers[0xB]);
		eprintln!("\tr12:  {:08X}",  self.registers[0xC]);
		eprintln!("\tsp:   {:08X}",  self.registers[0xD]);
		eprintln!("\tlr:   {:08X}",  self.registers[0xE]);
		eprintln!("\tpc:   {:08X}",  self.registers[0xF]);
		eprintln!("\tcpsr: {:032b}", self.psr);
	}
}
