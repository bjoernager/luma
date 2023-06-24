// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::emu::Emu;

impl Emu {
	pub fn trpadr(&mut self, adr: u32) {
		self.trp(format!("out-of-range address {adr:08X}"));
	}

	pub fn trpcnd(&mut self, cnd: u8) {
		self.trp(format!("invalid condition {cnd:02X}"));
	}

	pub fn trpopc(&mut self, adr: u32, opc: u32) {
		self.trp(format!("invalid opcode {opc:08X} at {adr:08X}"));
	}

	pub fn trpreg(&mut self, reg: u8) {
		self.trp(format!("invalid register {reg:02X}"));
	}

	fn trp(&mut self, msg: String) {
		eprintln!("trap - {msg}");
		eprintln!();
		eprintln!("\tr0:   {:08X}",  self.reg.r0);
		eprintln!("\tr1:   {:08X}",  self.reg.r1);
		eprintln!("\tr2:   {:08X}",  self.reg.r2);
		eprintln!("\tr3:   {:08X}",  self.reg.r3);
		eprintln!("\tr4:   {:08X}",  self.reg.r4);
		eprintln!("\tr5:   {:08X}",  self.reg.r5);
		eprintln!("\tr6:   {:08X}",  self.reg.r6);
		eprintln!("\tr7:   {:08X}",  self.reg.r7);
		eprintln!("\tr8:   {:08X}",  self.reg.r8);
		eprintln!("\tr9:   {:08X}",  self.reg.r9);
		eprintln!("\tr10:  {:08X}",  self.reg.r10);
		eprintln!("\tr11:  {:08X}",  self.reg.r11);
		eprintln!("\tr12:  {:08X}",  self.reg.r12);
		eprintln!("\tsp:   {:08X}",  self.reg.sp);
		eprintln!("\tlr:   {:08X}",  self.reg.lr);
		eprintln!("\tpc:   {:08X}",  self.reg.pc);
		eprintln!("\tcpsr: {:032b}", self.reg.cpsr);

		panic!("{msg}");
	}
}
