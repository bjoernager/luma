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

extern crate ctrlc;
extern crate sdl2;
extern crate toml;

pub mod app;
pub mod configuration;
pub mod cpu;
pub mod cpu_handle;
pub mod cpu_mode;
pub mod exception;
pub mod instruction;
pub mod predicate;
pub mod shifter;
pub mod state;

pub const VERSION: (u32, u32) = (
	0x0,  // major
	0x2F, // minor
);

#[derive(Clone, Copy)]
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

		eprintln!("\u{1B}[1m\u{1B}[91mtrap\u{1B}[0m: {message}");
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

macro_rules! log {
	() => {
		if cfg!(debug_assertions) { eprintln!() };
	};

	($($message: tt)*) => {{
		if cfg!(debug_assertions) {
			eprintln!("{}", format!($($message)?));
		}
	}};
}
pub(crate) use log;

macro_rules! log_assignment {
	($name: expr, $value: expr) => {{
		use crate::log;

		log!("\u{1B}[3m\u{B7} \u{1B}[1m{}\u{1B}[22m = {}\u{1B}[0m", $name, $value);
	}};
}
pub(crate) use log_assignment;

macro_rules! log_status {
	($($message: tt)*) => {{
		use crate::log;

		log!("({})", format!($($message)?));
	}};
}
pub(crate) use log_status;

fn main() { app::App::main() }
