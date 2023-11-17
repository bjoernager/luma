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

use std::mem::transmute;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Predicate {
	Eq = 0b0000,
	Ne = 0b0001,
	Cs = 0b0010, // Hs
	Cc = 0b0011, // Lo
	Mi = 0b0100,
	Pl = 0b0101,
	Vs = 0b0110,
	Vc = 0b0111,
	Hi = 0b1000,
	Ls = 0b1001,
	Ge = 0b1010,
	Lt = 0b1011,
	Gt = 0b1100,
	Le = 0b1101,
	Al = 0b1110,
	Nv = 0b1111,
}

impl Predicate {
	pub fn from_raw(mut raw: u8) -> Self {
		raw &= 0b00001111;
		return unsafe { transmute(raw) };
	}

	pub fn test(self, cpsr: u32) -> bool {
		let v = cpsr & 0b00010000000000000000000000000000 != 0x0;
		let c = cpsr & 0b00100000000000000000000000000000 != 0x0;
		let z = cpsr & 0b01000000000000000000000000000000 != 0x0;
		let n = cpsr & 0b10000000000000000000000000000000 != 0x0;

		use Predicate::*;
		return match self {
			Eq => z,
			Ne => !z,
			Cs => c,
			Cc => !c,
			Mi => n,
			Pl => !n,
			Vs => v,
			Vc => !v,
			Hi => c && !z,
			Ls => !c && z,
			Ge => n == v,
			Lt => n != v,
			Gt => !z && n == v,
			Le => z && n != v,
			Al => true,
			Nv => false, // Unpredictable in ARMv4.
		};
	}

	pub fn code(self) -> &'static str {
		use Predicate::*;
		return match self {
			Eq => "eq",
			Ne => "ne",
			Cs => "cs",
			Cc => "cc",
			Mi => "mi",
			Pl => "pl",
			Vs => "vs",
			Vc => "vc",
			Hi => "hi",
			Ls => "ls",
			Ge => "ge",
			Lt => "lt",
			Gt => "gt",
			Le => "le",
			Al => "al",
			Nv => "nv",
		};
	}
}
