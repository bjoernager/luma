// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::{MEMORY_SIZE, TrapKind};
use crate::luma::application::Application;

impl Application {
	pub fn trap(&mut self, kind: TrapKind) {
		let message = match kind {
			TrapKind::BadAlignment( address, alignment) => format!("bad alignment of address {address:#010X} (should be {alignment}-byte aligned)"),
			TrapKind::InvalidOpcode(address, opcode)    => format!("invalid opcode {opcode:#010X} at {address:#010X}"),
			TrapKind::OutOfBounds(  address)            => format!("out-of-bounds address {address:#010X} (limit is {MEMORY_SIZE:#010X})"),
		};

		eprintln!("trap - {message}");
		eprintln!("\tr0:   {:#010X}",  self.registers[0x0]);
		eprintln!("\tr1:   {:#010X}",  self.registers[0x1]);
		eprintln!("\tr2:   {:#010X}",  self.registers[0x2]);
		eprintln!("\tr3:   {:#010X}",  self.registers[0x3]);
		eprintln!("\tr4:   {:#010X}",  self.registers[0x4]);
		eprintln!("\tr5:   {:#010X}",  self.registers[0x5]);
		eprintln!("\tr6:   {:#010X}",  self.registers[0x6]);
		eprintln!("\tr7:   {:#010X}",  self.registers[0x7]);
		eprintln!("\tr8:   {:#010X}",  self.registers[0x8]);
		eprintln!("\tr9:   {:#010X}",  self.registers[0x9]);
		eprintln!("\tr10:  {:#010X}",  self.registers[0xA]);
		eprintln!("\tr11:  {:#010X}",  self.registers[0xB]);
		eprintln!("\tr12:  {:#010X}",  self.registers[0xC]);
		eprintln!("\tsp:   {:#010X}",  self.registers[0xD]);
		eprintln!("\tlr:   {:#010X}",  self.registers[0xE]);
		eprintln!("\tpc:   {:#010X}",  self.registers[0xF]);
		eprintln!("\tcpsr: {:#034b}", self.psr);
	}
}
