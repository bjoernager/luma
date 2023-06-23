// Copyright 2021-2023 Gabriel Jensen.

pub mod app;

pub const VER: u32 = 0x1E;

pub const MEMSIZ: usize = 0x0E010000;

pub const BTLSIZ: usize = 0x00004000;
pub const IMGSIZ: usize = 0x06000000;

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
