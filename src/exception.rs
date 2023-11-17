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

use crate::cpu_mode::CpuMode;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Exception {
	Reset,
	UndefinedInstruction,
	SoftwareInterrupt,
	PrefetchAbort,
	DataAbort,
	InterruptRequest,
	FastInterruptRequest,
}

impl Exception {
	pub fn mode(self) -> CpuMode {
		return match self {
			Self::Reset                => CpuMode::Supervisor,
			Self::UndefinedInstruction => CpuMode::Undefined,
			Self::SoftwareInterrupt    => CpuMode::Supervisor,
			Self::PrefetchAbort        => CpuMode::Abort,
			Self::DataAbort            => CpuMode::Abort,
			Self::InterruptRequest     => CpuMode::InterruptRequest,
			Self::FastInterruptRequest => CpuMode::FastInterruptRequest,
		};
	}

	pub fn vector_address(self) -> u32 {
		use Exception::*;

		return match self {
			Reset                => 0x00000000,
			UndefinedInstruction => 0x00000004,
			SoftwareInterrupt    => 0x00000008,
			PrefetchAbort        => 0x0000000C,
			DataAbort            => 0x00000010,
			InterruptRequest     => 0x00000018,
			FastInterruptRequest => 0x0000001C,
		};
	}
}
