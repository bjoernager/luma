// Copyright 2021-2023 Gabriel Jensen.

pub mod bootloader;
pub mod drop;
pub mod read;
pub mod image;
pub mod new;
pub mod opcode;
pub mod run;
pub mod trap;

pub enum TrapKind {
	BadAlignment,
	InvalidOpcode,
	OutOfBounds,
}

pub struct Emulator {
	memory:    *mut u8,
	registers: [u32; 0x10],
	psr:       u32,
}
