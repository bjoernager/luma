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

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum CpuMode {
	User                 = 0b10000,
	FastInterruptRequest = 0b10001,
	InterruptRequest     = 0b10010,
	Supervisor           = 0b10011,
	Abort                = 0b10111,
	Undefined            = 0b11011,
	System               = 0b11111,
}

impl CpuMode {
	pub fn from_raw(raw: u8) -> Self {
		use CpuMode::*;

		return match raw {
			0b10000 => User,
			0b10001 => FastInterruptRequest,
			0b10010 => InterruptRequest,
			0b10011 => Supervisor,
			0b10111 => Abort,
			0b11011 => Undefined,
			0b11111 => System,

			_ => panic!("invalid cpu mode {raw:#010b}"),
		};
	}

	pub fn name(self) -> &'static str {
		use CpuMode::*;

		return match self {
			Abort                => "abt",
			FastInterruptRequest => "fiq",
			InterruptRequest     => "irq",
			Supervisor           => "svc",
			System               => "sys",
			Undefined            => "und",
			User                 => "usr",
		};
	}
}
