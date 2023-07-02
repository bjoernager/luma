/*
	Copyright 2021-2023 Gabriel Jensen.

	This file is part of Luma.

	Luma is free software: you can redistribute it 
	and/or modify it under the terms of the GNU 
	Affero General Public License as published by
	the Free Software Foundation, either version 3 
	of the License, or (at your option) any later 
	version.

	Luma is distributed in the hope that it will be 
	useful, but WITHOUT ANY WARRANTY; without even 
	the implied warranty of MERCHANTABILITY or 
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
	Affero General Public License for more details.

	You should have received a copy of the GNU 
	Affero General Public License along with Luma. If not, 
	see <https://www.gnu.org/licenses/>. 
*/

pub mod bootloader;
pub mod branch;
pub mod check_condition;
pub mod r#continue;
pub mod decode_arm;
pub mod decode_thumb;
pub mod drop;
pub mod exchange;
pub mod image;
pub mod log;
pub mod memory;
pub mod r#move;
pub mod new;
pub mod read;
pub mod store;
pub mod thumb;
pub mod trap;
pub mod write;

pub enum Trap {
	BadAlignment(      u32, u32),
	InvalidArmOpcode(  u32, u32),
	InvalidThumbOpcode(u32, u16),
	OutOfBounds(       u32),
}

pub enum Branch {
	Offset(  i32, bool),
	Register(u8),
}

pub enum Move {
	Immediate(u8),
	Register( u8),
}

pub struct Device {
	pub decode:    fn(&mut Device),
	
	memory:    *mut u8,
	registers: [u32; 0x10],
	cpsr:      u32,
	spsr:      [u32; 0x10], // We don't actually use all sixteen, we just have this many to enable us to directly use the mode number as the offset.
}
