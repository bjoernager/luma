// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::{Application, TrapKind};
use crate::luma::MEMORY_SIZE;

impl Application {
	#[allow(dead_code)]
	pub fn read_byte(&mut self, address: u32) -> u8 {
		if address >= MEMORY_SIZE as u32 { self.trap(TrapKind::OutOfBounds, Some(address), None, None) };

		return unsafe { *(self.memory.offset(address as isize) as *mut u8) };
	}

	#[allow(dead_code)]
	pub fn read_halfword(&mut self, address: u32) -> u16 {
		if address >= MEMORY_SIZE as u32 { self.trap(TrapKind::OutOfBounds,  Some(address), None, None) };
		if address % 0x2 != 0x0          { self.trap(TrapKind::BadAlignment, Some(address), None, Some(0x2)) };

		return unsafe { *(self.memory.offset(address as isize) as *mut u16) };
	}

	#[allow(dead_code)]
	pub fn read_word(&mut self, address: u32) -> u32 {
		if address >= MEMORY_SIZE as u32 { self.trap(TrapKind::OutOfBounds,  Some(address), None, None) };
		if address % 0x4 != 0x0          { self.trap(TrapKind::BadAlignment, Some(address), None, Some(0x4)) };

		return unsafe { *(self.memory.offset(address as isize) as *mut u32) };
	}
}