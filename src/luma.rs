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
	Affero General Public License along with Luma.
	If not, see <https://www.gnu.org/licenses/>.
*/

use sdl2::pixels::Color;

pub mod app;
pub mod configuration;
pub mod cpu;
pub mod cpu_handle;
pub mod instruction;
pub mod state;

pub const VERSION: (u32, u32) = (
	0x0,  // major
	0x2D, // minor
);

pub enum Error {
	BadAlignment(      u32, u32),
	InvalidArmOpcode(  u32, u32),
	InvalidThumbOpcode(u32, u16),
	OutOfBounds(       u32),
}

impl Error {
	pub fn trap(&self) {
		let message = match self {
			Error::BadAlignment(        address, alignment) => format!("bad alignment of address {address:#010X} (should be {alignment}-byte aligned)"),
			Error::InvalidArmOpcode(    address, opcode)    => format!("invalid opcode {opcode:#034b} at {address:#010X}"),
			Error::InvalidThumbOpcode(  address, opcode)    => format!("invalid opcode {opcode:#018b} at {address:#010X}"),
			Error::OutOfBounds(         address)            => format!("out-of-bounds address {address:#010X} (limit is {:#010X})", MEMORY_LENGTH),
		};

		eprintln!("trap: {message}");
	}
}

pub const MEMORY_LENGTH: u32 = 0x0E010000;

pub const BOOTLOADER_LENGTH: u32 = 0x00004000;
pub const IMAGE_LENGTH:      u32 = 0x02000000;
pub const VIDEO_LENGTH:      u32 = 0x00018000;
pub const PALETTE_LENGTH:    u32 = 0x00000400;

pub const SCREEN_SIZE: (u8, u8) = (
	0xF0, // width
	0xA0, // height
);

pub const fn decode_colour(colour: u16) -> Color {
	let red   = ((colour & 0b0000000000011111) << 0x3) as u8;
	let green = ((colour & 0b0000001111100000) >> 0x2) as u8;
	let blue  = ((colour & 0b0111110000000000) >> 0x7) as u8;

	return Color::RGB(red, green, blue);
}

pub fn log(message: &str) {
	// This optimises the function away.
   if cfg!(debug_assertions) {
	   eprintln!("{message}");
   }
}
