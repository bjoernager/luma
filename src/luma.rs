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

pub mod application;
pub mod configuration;
pub mod device;

pub struct VersionType<T> {
	major: T,
	minor: T,
}

pub const VERSION: VersionType::<u32> = VersionType::<u32> {
	major: 0x0,
	minor: 0x2C,
};

pub struct WidthHeight<T> {
	width:  T,
	height: T,
}

pub const CONFIGURATION_VERSION: u32 = 0x0;

pub const MEMORY_SIZE: usize = 0x0E010000;

pub const BOOTLOADER_SIZE: usize = 0x00004000;
pub const IMAGE_SIZE:      usize = 0x02000000;
pub const VIDEO_SIZE:      usize = 0x00018000;
pub const PALETTE_SIZE:    usize = 0x00000400;

pub const SCREEN_SIZE: WidthHeight::<u8> = WidthHeight::<u8> {
	width:  0xF0,
	height: 0xA0,
};
