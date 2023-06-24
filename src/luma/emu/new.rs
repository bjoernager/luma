// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::MEMSIZ;
use crate::luma::emu::{Emu, Reg};

use std::alloc::{alloc_zeroed, Layout};
use std::mem::size_of;

impl Emu {
	pub fn new() -> Emu {
		let mem = unsafe { alloc_zeroed(Layout::new::<[u32; MEMSIZ / size_of::<u32>()]>()) };
		if mem.is_null() { panic!("unable to allocate memory buffer") }

		eprintln!("allocated memory buffer at 0x{:0X}", mem as usize);

		return Emu {
			mem: mem,
			reg: Reg {
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
			},
		};
	}
}
