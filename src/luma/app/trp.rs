// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::Reg;
use crate::luma::app::App;

impl App {
	pub fn trp(&mut self, reg: &Reg, opc: u32) {
		eprintln!("trap - register dump:");
		eprintln!("\tr0:   {:08X}",  reg.r0);
		eprintln!("\tr1:   {:08X}",  reg.r1);
		eprintln!("\tr2:   {:08X}",  reg.r2);
		eprintln!("\tr3:   {:08X}",  reg.r3);
		eprintln!("\tr4:   {:08X}",  reg.r4);
		eprintln!("\tr5:   {:08X}",  reg.r5);
		eprintln!("\tr6:   {:08X}",  reg.r6);
		eprintln!("\tr7:   {:08X}",  reg.r7);
		eprintln!("\tr8:   {:08X}",  reg.r8);
		eprintln!("\tr9:   {:08X}",  reg.r9);
		eprintln!("\tr10:  {:08X}",  reg.r10);
		eprintln!("\tr11:  {:08X}",  reg.r11);
		eprintln!("\tr12:  {:08X}",  reg.r12);
		eprintln!("\tsp:   {:08X}",  reg.sp);
		eprintln!("\tlr:   {:08X}",  reg.lr);
		eprintln!("\tpc:   {:08X}",  reg.pc);
		eprintln!("\tcpsr: {:032b}", reg.cpsr);

		self.end(0x1, Some(format!("invalid opcode 0x{opc:08X} at 0x{:08X}",reg.pc-0x8).as_str()));
	}
}
