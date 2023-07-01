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

use crate::luma::device::{Device, Log};

impl Device {
	pub fn log(&mut self, kind: Log) {
		let kind_string = match kind {
			Log::Branch(       ..) => "branch  ",
			Log::Continue(     ..) => "continue",
			Log::Link(         ..) => "link    ",
			Log::Load(         ..) => "load    ",
			Log::MoveRegister( ..) => "move    ",
			Log::MoveImmediate(..) => "move    ",
			Log::Store(        ..) => "store   ",
		};

		let message = match kind {
			Log::Branch(       offset,      address)                        => format!("r15{offset:+} => {address:#010X}"),
			Log::Continue(     address)                                     => format!("r15 => {address:#010X}"),
			Log::Link(         address)                                     => format!("r14 => {address:#010X}"),
			Log::Load(         register,    address,   base, offset, value) => format!("r{register} => r{base}{offset:+}={address:#010X} ({value:#010X})"),
			Log::MoveRegister( destination, source,    value)               => format!("r{destination} => r{source} ({value:#010X})"),
			Log::MoveImmediate(register,    immediate)                      => format!("r{register} => {immediate:#X}"),
			Log::Store(        address,     register,  base, offset, value) => format!("r{base}{offset:+}={address:#010X} => r{register} ({value:#010X})"),
		};

		eprintln!("{kind_string} : {message}");
	}
}
