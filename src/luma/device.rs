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
pub mod r#continue;
pub mod decode;
pub mod drop;
pub mod image;
pub mod store;
pub mod log;
pub mod memory;
pub mod r#move;
pub mod new;
pub mod read;
pub mod trap;
pub mod write;

#[allow(dead_code)]
pub enum Log {
	Branch(       i32, u32),
	Continue(     u32),
	Link(         u32),
	Load(         u8,  u32,  u8,  i32, u32),
	MoveRegister( u8,  u8,   u32),
	MoveImmediate(u8,  u32),
	Store(        u32, u8,   u8,  i32, u32),
}

#[allow(dead_code)]
pub enum Trap {
	BadAlignment( u32, u32),
	InvalidOpcode(u32, u32),
	OutOfBounds(  u32),
}

pub struct Device {
	memory:    *mut u8,
	registers: [u32; 0x10],
	cpsr:      u32,
	spsr:      [u32; 0x10], // We don't actually use all sixteen, we just have this many to enable us to directly use the mode number as the offset.
}
