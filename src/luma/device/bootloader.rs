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

use crate::luma::device::Device;
use crate::luma::BOOTLOADER_SIZE;

use std::slice;

impl Device {
	pub fn bootloader<'a>(&mut self) -> &'a mut [u8] {
		return unsafe { slice::from_raw_parts_mut(self.memory.offset(0x00000000), BOOTLOADER_SIZE) };
	}
}
