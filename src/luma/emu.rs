// Copyright 2021-2023 Gabriel Jensen.

pub mod btl;
pub mod drop;
pub mod img;
pub mod new;
pub mod opc;
pub mod run;
pub mod trp;

pub struct Reg {
	r0:   u32,
	r1:   u32,
	r2:   u32,
	r3:   u32,
	r4:   u32,
	r5:   u32,
	r6:   u32,
	r7:   u32,
	r8:   u32,
	r9:   u32,
	r10:  u32,
	r11:  u32,
	r12:  u32,
	sp:   u32,
	lr:   u32,
	pc:   u32,
	cpsr: u32,
}

pub struct Emu {
	mem: *mut u8,
	reg: Reg,
}
