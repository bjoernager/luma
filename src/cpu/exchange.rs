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

macro_rules! exchange {
	($cpu: ident, $t: expr) => {{
		use crate::log_status;
		use crate::cpu::Fetcher;

		log_status!("exchanging to {}", match $t {
			false => "ARM",
			true  => "Thumb",
		});

		const DATA: [(u32, Fetcher); 0x2] = [
			(0x4, Cpu::fetch_arm),
			(0x2, Cpu::fetch_thumb),
		];

		let index = $t as usize & 0b1;

		$cpu.instruction_size = unsafe { DATA.get_unchecked(index).0 };
		$cpu.fetcher          = unsafe { DATA.get_unchecked(index).1 };
	}};
}
pub(crate) use exchange;
